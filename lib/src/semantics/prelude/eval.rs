use crate::semantics::{
    ctx_access::{
        free::FreeCtx,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    eval::Evaluator,
    eval_mode::{
        eval::Eval,
        EvalMode,
        BY_VAL,
    },
    func::{
        CtxFreeFn,
        CtxMutableFn,
        Primitive,
    },
    input_mode::InputMode,
    prelude::{
        names,
        PrimitiveFunc,
    },
    val::Val,
};

pub(crate) fn value() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::VALUE, fn_value);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_value(input: Val) -> Val {
    input
}

pub(crate) fn eval() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::EVAL, fn_eval);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_eval(mut ctx: CtxForMutableFn, input: Val) -> Val {
    Eval.eval(&mut ctx, input)
}

pub(crate) fn quote() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::QUOTE, fn_quote);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_quote(mut ctx: CtxForMutableFn, input: Val) -> Val {
    BY_VAL.quote.eval(&mut ctx, input)
}

pub(crate) fn eval_twice() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::EVAL_TWICE,
        fn_eval_twice::<FreeCtx>,
        |ctx, val| fn_eval_twice(ctx, val),
        |ctx, val| fn_eval_twice(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_eval_twice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val = Eval.eval(&mut ctx, input);
    Eval.eval(&mut ctx, val)
}

pub(crate) fn eval_thrice() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new_dispatch(
        names::EVAL_THRICE,
        fn_eval_thrice::<FreeCtx>,
        |ctx, val| fn_eval_thrice(ctx, val),
        |ctx, val| fn_eval_thrice(ctx, val),
    );
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_eval_thrice<Ctx: CtxAccessor>(mut ctx: Ctx, input: Val) -> Val {
    let val1 = Eval.eval(&mut ctx, input);
    let val2 = Eval.eval(&mut ctx, val1);
    Eval.eval(&mut ctx, val2)
}
