use crate::{
    semantics::{
        ctx::{
            CtxTrait,
            TaggedRef,
        },
        ctx_access::{
            constant::ConstCtx,
            free::FreeCtx,
            mutable::{
                CtxForMutableFn,
                MutableCtx,
            },
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
            names,
            PrimitiveFunc,
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

pub(crate) fn chain() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Value),
        InputMode::Any(EvalMode::Value),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::CHAIN, fn_chain);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Eval.eval_call(&mut ctx, pair.second, pair.first)
}

pub(crate) fn call_with_ctx() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value))),
        InputMode::Call(Box::new(Call::new(
            InputMode::Any(EvalMode::Eval),
            InputMode::Any(EvalMode::Value),
        ))),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::CALL_WITH_CTX, fn_call_with_ctx);
    PrimitiveFunc::new(input_mode, primitive)
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
