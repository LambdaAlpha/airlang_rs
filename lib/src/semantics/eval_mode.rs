use crate::semantics::{
    ctx_access::CtxAccessor,
    eval::{
        Evaluator,
        ValBuilder,
    },
    eval_mode::{
        eval::{
            Eval,
            EvalByRef,
        },
        mix::{
            Mix,
            MixByRef,
        },
        value::{
            Value,
            ValueByRef,
        },
    },
    Val,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EvalMode {
    Value,
    Eval,
    Mix,
}

pub(crate) const BY_VAL: ByValEvaluators<Eval, Value, ValBuilder> = Evaluators {
    eval: Eval,
    value: Value,
    mix: Mix {
        eval: Eval,
        value: Value,
        builder: ValBuilder,
    },
};

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, BY_VAL)
    }
}

pub(crate) const BY_REF: ByRefEvaluators<EvalByRef, ValueByRef, ValBuilder> = Evaluators {
    eval: EvalByRef,
    value: ValueByRef,
    mix: MixByRef {
        eval: EvalByRef,
        value: ValueByRef,
        builder: ValBuilder,
    },
};

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_generic(ctx, input, BY_REF)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Evaluators<Eval, Value, Mix> {
    pub(crate) eval: Eval,
    pub(crate) value: Value,
    pub(crate) mix: Mix,
}

pub(crate) type ByValEvaluators<Eval, Value, Builder> =
    Evaluators<Eval, Value, Mix<Eval, Value, Builder>>;
pub(crate) type ByRefEvaluators<Eval, Value, Builder> =
    Evaluators<Eval, Value, MixByRef<Eval, Value, Builder>>;

impl EvalMode {
    pub(crate) fn eval_generic<Ctx, Input, Output, Eval, Value, Mix>(
        &self,
        ctx: &mut Ctx,
        input: Input,
        evaluators: Evaluators<Eval, Value, Mix>,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Input, Output>,
        Value: Evaluator<Ctx, Input, Output>,
        Mix: Evaluator<Ctx, Input, Output>,
    {
        match self {
            EvalMode::Value => evaluators.value.eval(ctx, input),
            EvalMode::Eval => evaluators.eval.eval(ctx, input),
            EvalMode::Mix => evaluators.mix.eval(ctx, input),
        }
    }
}

pub(crate) mod value;

pub(crate) mod mix;

pub(crate) mod eval;
