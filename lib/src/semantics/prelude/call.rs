use crate::{
    semantics::{
        ctx::{
            DefaultCtx,
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
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxMutableFn,
            Primitive,
        },
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
    let eval_mode = EvalMode::Pair(Box::new(Pair::new(
        EvalMode::Any(BasicEvalMode::Value),
        EvalMode::Any(BasicEvalMode::Value),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::CHAIN, fn_chain);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_chain(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    Eval.eval_call(&mut ctx, pair.second, pair.first)
}

pub(crate) fn call_with_ctx() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Pair(Box::new(Pair::new(
        EvalMode::Symbol(BasicEvalMode::Value),
        EvalMode::Call(Box::new(Call::new(
            EvalMode::Any(BasicEvalMode::Eval),
            EvalMode::Any(BasicEvalMode::Value),
        ))),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::CALL_WITH_CTX, fn_call_with_ctx);
    PrimitiveFunc::new(eval_mode, primitive)
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
    let input = func.input_eval_mode.eval(&mut ctx, call.input);
    if let Val::Unit(_) = target_ctx {
        return func.eval(&mut FreeCtx, input);
    }
    DefaultCtx.get_tagged_ref(&mut ctx, target_ctx, |target_ctx| {
        let TaggedRef {
            val_ref: Val::Ctx(CtxVal(target_ctx)),
            is_const: target_ctx_const,
        } = target_ctx
        else {
            return Val::default();
        };
        if target_ctx_const {
            func.eval(&mut ConstCtx(target_ctx), input)
        } else {
            func.eval(&mut MutableCtx(target_ctx), input)
        }
    })
}
