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
        quote::{
            Quote,
            QuoteByRef,
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
    Quote,
}

pub(crate) const BY_VAL: ByValEvaluators<Eval, Value, ValBuilder> = Evaluators {
    eval: Eval,
    value: Value,
    quote: Quote {
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
    quote: QuoteByRef {
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
pub(crate) struct Evaluators<Eval, Value, Quote> {
    pub(crate) eval: Eval,
    pub(crate) value: Value,
    pub(crate) quote: Quote,
}

pub(crate) type ByValEvaluators<Eval, Value, Builder> =
    Evaluators<Eval, Value, Quote<Eval, Value, Builder>>;
pub(crate) type ByRefEvaluators<Eval, Value, Builder> =
    Evaluators<Eval, Value, QuoteByRef<Eval, Value, Builder>>;

impl EvalMode {
    pub(crate) fn eval_generic<Ctx, Input, Output, Eval, Value, Quote>(
        &self,
        ctx: &mut Ctx,
        input: Input,
        evaluators: Evaluators<Eval, Value, Quote>,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Input, Output>,
        Value: Evaluator<Ctx, Input, Output>,
        Quote: Evaluator<Ctx, Input, Output>,
    {
        match self {
            EvalMode::Value => evaluators.value.eval(ctx, input),
            EvalMode::Eval => evaluators.eval.eval(ctx, input),
            EvalMode::Quote => evaluators.quote.eval(ctx, input),
        }
    }
}

pub(crate) mod value;

pub(crate) mod quote;

pub(crate) mod eval;
