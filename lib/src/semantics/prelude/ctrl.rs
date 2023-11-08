use crate::{
    semantics::{
        ctx_access::{
            free::FreeCtx,
            CtxAccessor,
        },
        eval::Evaluator,
        eval_mode::{
            eval::{
                Eval,
                EvalByRef,
            },
            EvalMode,
        },
        func::{
            CtxMutableFn,
            Primitive,
        },
        input_mode::{
            InputMode,
            ListItemInputMode,
        },
        prelude::{
            names,
            PrimitiveFunc,
        },
        val::Val,
    },
    types::{
        List,
        Pair,
        Symbol,
    },
};

pub(crate) fn sequence() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::List(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::SEQUENCE,
        fn_sequence::<FreeCtx>,
        |ctx, val| fn_sequence(ctx, val),
        |ctx, val| fn_sequence(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_sequence<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut output = Val::default();
    for val in list {
        output = Eval.eval(&mut ctx, val);
    }
    output
}

pub(crate) fn breakable_sequence() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Value),
    )));
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::BREAKABLE_SEQUENCE,
        fn_breakable_sequence::<FreeCtx>,
        |ctx, val| fn_breakable_sequence(ctx, val),
        |ctx, val| fn_breakable_sequence(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_breakable_sequence<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name = match pair.first {
        Val::Symbol(s) => s,
        Val::Unit(_) => Symbol::from_str("return"),
        _ => return Val::default(),
    };
    let Val::List(list) = pair.second else {
        return Val::default();
    };

    let mut output = Val::default();
    let _ = ctx.remove(&name);
    for val in list {
        output = Eval.eval(&mut ctx, val);
        if let Ok(val) = ctx.get_const_ref(&name) {
            return val.clone();
        }
    }
    output
}

pub(crate) fn condition() -> PrimitiveFunc<CtxMutableFn> {
    let list = List::from(vec![
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Eval),
            ellipsis: false,
        },
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Value),
            ellipsis: false,
        },
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Value),
            ellipsis: false,
        },
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Value),
            ellipsis: false,
        },
    ]);
    let input_mode = InputMode::ListForSome(list);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::IF,
        fn_if::<FreeCtx>,
        |ctx, val| fn_if(ctx, val),
        |ctx, val| fn_if(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_if<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut iter = list.into_iter();
    let Some(condition) = iter.next() else {
        return Val::default();
    };
    if let Val::Bool(b) = condition {
        if b.bool() {
            let Some(branch) = iter.next() else {
                return Val::default();
            };
            Eval.eval(&mut ctx, branch)
        } else {
            let _ = iter.next();
            let Some(branch) = iter.next() else {
                return Val::default();
            };
            Eval.eval(&mut ctx, branch)
        }
    } else {
        let _ = iter.next();
        let _ = iter.next();
        let Some(default) = iter.next() else {
            return Val::default();
        };
        Eval.eval(&mut ctx, default)
    }
}

pub(crate) fn matching() -> PrimitiveFunc<CtxMutableFn> {
    let list = List::from(vec![
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Eval),
            ellipsis: false,
        },
        ListItemInputMode {
            input_mode: InputMode::Map(EvalMode::Value),
            ellipsis: false,
        },
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Value),
            ellipsis: false,
        },
    ]);
    let input_mode = InputMode::ListForSome(list);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::MATCH,
        fn_match::<FreeCtx>,
        |ctx, val| fn_match(ctx, val),
        |ctx, val| fn_match(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_match<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut iter = list.into_iter();
    let Some(val) = iter.next() else {
        return Val::default();
    };
    let Some(Val::Map(map)) = iter.next() else {
        return Val::default();
    };
    let eval = map
        .into_iter()
        .find_map(|(k, v)| {
            let k = Eval.eval(&mut ctx, k);
            if k == val {
                Some(v)
            } else {
                None
            }
        })
        .unwrap_or_else(|| {
            let Some(default) = iter.next() else {
                return Val::default();
            };
            default
        });
    Eval.eval(&mut ctx, eval)
}

pub(crate) fn while_loop() -> PrimitiveFunc<CtxMutableFn> {
    let list = List::from(vec![
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Value),
            ellipsis: false,
        },
        ListItemInputMode {
            input_mode: InputMode::Any(EvalMode::Value),
            ellipsis: false,
        },
    ]);
    let input_mode = InputMode::ListForSome(list);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::WHILE,
        fn_while::<FreeCtx>,
        |ctx, val| fn_while(ctx, val),
        |ctx, val| fn_while(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

#[allow(clippy::get_first)]
fn fn_while<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let Some(condition) = list.get(0) else {
        return Val::default();
    };
    let Some(body) = list.get(1) else {
        return Val::default();
    };
    loop {
        let Val::Bool(b) = EvalByRef.eval(&mut ctx, condition) else {
            return Val::default();
        };
        if b.bool() {
            EvalByRef.eval(&mut ctx, body);
        } else {
            break;
        }
    }
    Val::default()
}
