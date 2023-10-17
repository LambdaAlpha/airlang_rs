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
            BasicEvalMode,
            EvalMode,
            ListItemEvalMode,
        },
        func::{
            CtxMutableFn,
            Primitive,
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
    let eval_mode = EvalMode::List(BasicEvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::SEQUENCE,
        fn_sequence::<FreeCtx>,
        |ctx, val| fn_sequence(ctx, val),
        |ctx, val| fn_sequence(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
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
    let eval_mode = EvalMode::Pair(Box::new(Pair::new(
        EvalMode::Symbol(BasicEvalMode::Value),
        EvalMode::List(BasicEvalMode::Value),
    )));
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::BREAKABLE_SEQUENCE,
        fn_breakable_sequence::<FreeCtx>,
        |ctx, val| fn_breakable_sequence(ctx, val),
        |ctx, val| fn_breakable_sequence(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
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
    ctx.remove(&name);
    for val in list {
        output = Eval.eval(&mut ctx, val);
        if let Some(val) = ctx.get_const_ref(&name) {
            return val.clone();
        }
    }
    output
}

pub(crate) fn condition() -> PrimitiveFunc<CtxMutableFn> {
    let list = List::from(vec![
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Eval),
            ellipsis: false,
        },
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Value),
            ellipsis: false,
        },
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Value),
            ellipsis: false,
        },
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Value),
            ellipsis: false,
        },
    ]);
    let eval_mode = EvalMode::ListForSome(list);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::IF,
        fn_if::<FreeCtx>,
        |ctx, val| fn_if(ctx, val),
        |ctx, val| fn_if(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
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
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Eval),
            ellipsis: false,
        },
        ListItemEvalMode {
            eval_mode: EvalMode::Map(BasicEvalMode::Value),
            ellipsis: false,
        },
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Value),
            ellipsis: false,
        },
    ]);
    let eval_mode = EvalMode::ListForSome(list);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::MATCH,
        fn_match::<FreeCtx>,
        |ctx, val| fn_match(ctx, val),
        |ctx, val| fn_match(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
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
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Value),
            ellipsis: false,
        },
        ListItemEvalMode {
            eval_mode: EvalMode::Any(BasicEvalMode::Value),
            ellipsis: false,
        },
    ]);
    let eval_mode = EvalMode::ListForSome(list);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::WHILE,
        fn_while::<FreeCtx>,
        |ctx, val| fn_while(ctx, val),
        |ctx, val| fn_while(ctx, val),
    );
    PrimitiveFunc::new(eval_mode, primitive)
}

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
