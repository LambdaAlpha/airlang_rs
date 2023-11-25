use crate::{
    semantics::{
        ctx::{
            CtxTrait,
            NameMap,
            TaggedRef,
        },
        ctx_access::{
            constant::ConstCtx,
            free::FreeCtx,
            mutable::{
                CtxForMutableFn,
                MutableCtx,
            },
            CtxAccessor,
        },
        eval::{
            input::ByVal,
            Evaluator,
        },
        eval_mode::{
            eval::Eval,
            EvalMode,
        },
        func::{
            CtxMutableFn,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            named_mutable_fn,
            Named,
            Prelude,
        },
        val::{
            CtxVal,
            FuncVal,
        },
        Val,
    },
    types::{
        Call,
        Pair,
    },
};

#[derive(Clone)]
pub(crate) struct CallPrelude {
    pub(crate) call: Named<FuncVal>,
    pub(crate) chain: Named<FuncVal>,
    pub(crate) call_with_ctx: Named<FuncVal>,
}

impl Default for CallPrelude {
    fn default() -> Self {
        CallPrelude {
            call: call(),
            chain: chain(),
            call_with_ctx: call_with_ctx(),
        }
    }
}

impl Prelude for CallPrelude {
    fn put(&self, m: &mut NameMap) {
        self.call.put(m);
        self.chain.put(m);
        self.call_with_ctx.put(m);
    }
}

fn call() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::Any(EvalMode::Eval),
    )));
    let func = Primitive::<CtxMutableFn>::dispatch(
        fn_call::<FreeCtx>,
        |ctx, val| fn_call(ctx, val),
        |ctx, val| fn_call(ctx, val),
    );
    named_mutable_fn("$", input_mode, func)
}

fn fn_call<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Func(FuncVal(func)) = &pair.first else {
        return Val::default();
    };
    let input = func.input_mode.eval(&mut ctx, pair.second);
    func.eval(&mut ctx, input)
}

fn chain() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Value),
        InputMode::Any(EvalMode::Value),
    )));
    named_mutable_fn(".", input_mode, fn_chain)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Eval.eval_call(&mut ctx, pair.second, pair.first)
}

fn call_with_ctx() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value))),
        InputMode::Call(Box::new(Call::new(
            InputMode::Any(EvalMode::Eval),
            InputMode::Any(EvalMode::Value),
        ))),
    )));
    named_mutable_fn("do", input_mode, fn_call_with_ctx)
}

fn fn_call_with_ctx(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Call(call) = pair.second else {
        return Val::default();
    };
    let Val::Func(FuncVal(func)) = call.func else {
        return Val::default();
    };
    let target_ctx = pair.first;
    let input = func.input_mode.eval(&mut ctx, call.input);

    match target_ctx {
        Val::Unit(_) => func.eval(&mut FreeCtx, input),
        Val::Symbol(name) if &*name == "meta" => {
            let Ok(meta) = ctx.get_tagged_meta() else {
                return Val::default();
            };
            if meta.is_const {
                func.eval(&mut ConstCtx(meta.val_ref), input)
            } else {
                func.eval(&mut MutableCtx(meta.val_ref), input)
            }
        }
        Val::List(names) => get_ctx_nested(ctx, &names[..], |mut ctx| func.eval(&mut ctx, input)),
        _ => Val::default(),
    }
}

fn get_ctx_nested<F>(mut ctx: CtxForMutableFn, names: &[Val], f: F) -> Val
where
    F: for<'a> FnOnce(CtxForMutableFn<'a>) -> Val,
{
    let Some(Val::Symbol(name)) = names.first() else {
        return f(ctx);
    };
    let rest = &names[1..];

    let Ok(TaggedRef { val_ref, is_const }) = ctx.get_tagged_ref(name) else {
        return Val::default();
    };
    let Val::Ctx(CtxVal(ctx)) = val_ref else {
        return Val::default();
    };
    if is_const {
        get_ctx_nested(CtxForMutableFn::Const(ConstCtx(ctx)), rest, f)
    } else {
        get_ctx_nested(CtxForMutableFn::Mutable(MutableCtx(ctx)), rest, f)
    }
}
