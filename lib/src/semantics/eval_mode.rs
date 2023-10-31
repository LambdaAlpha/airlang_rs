use crate::semantics::{
    ctx_access::CtxAccessor,
    eval::{
        BoolAndBuilder,
        Evaluator,
        OpValBuilder,
        ValBuilder,
    },
    eval_mode::{
        eval::{
            Eval,
            EvalByRef,
            EvalConst,
            EvalConstByRef,
            EvalConstChecker,
            EvalConstCheckerByRef,
            EvalFree,
            EvalFreeByRef,
            EvalFreeChecker,
            EvalFreeCheckerByRef,
        },
        quote::{
            Quote,
            QuoteByRef,
        },
        value::{
            Value,
            ValueByRef,
            ValueFreeConst,
            ValueFreeConstByRef,
            ValueFreeConstChecker,
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

pub(crate) const FREE: ByValEvaluators<EvalFree, ValueFreeConst, OpValBuilder> = Evaluators {
    eval: EvalFree,
    value: ValueFreeConst,
    quote: Quote {
        eval: EvalFree,
        value: ValueFreeConst,
        builder: OpValBuilder,
    },
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, FREE)
    }
}

pub(crate) const FREE_BY_REF: ByRefEvaluators<EvalFreeByRef, ValueFreeConstByRef, OpValBuilder> =
    Evaluators {
        eval: EvalFreeByRef,
        value: ValueFreeConstByRef,
        quote: QuoteByRef {
            eval: EvalFreeByRef,
            value: ValueFreeConstByRef,
            builder: OpValBuilder,
        },
    };

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, FREE_BY_REF)
    }
}

pub(crate) const CONST: ByValEvaluators<EvalConst, ValueFreeConst, OpValBuilder> = Evaluators {
    eval: EvalConst,
    value: ValueFreeConst,
    quote: Quote {
        eval: EvalConst,
        value: ValueFreeConst,
        builder: OpValBuilder,
    },
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, CONST)
    }
}

pub(crate) const CONST_BY_REF: ByRefEvaluators<EvalConstByRef, ValueFreeConstByRef, OpValBuilder> =
    Evaluators {
        eval: EvalConstByRef,
        value: ValueFreeConstByRef,
        quote: QuoteByRef {
            eval: EvalConstByRef,
            value: ValueFreeConstByRef,
            builder: OpValBuilder,
        },
    };

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, CONST_BY_REF)
    }
}

pub(crate) const FREE_CHECKER: ByValEvaluators<
    EvalFreeChecker,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Evaluators {
    eval: EvalFreeChecker,
    value: ValueFreeConstChecker,
    quote: Quote {
        eval: EvalFreeChecker,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    },
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, FREE_CHECKER)
    }
}

pub(crate) const FREE_CHECKER_BY_REF: ByRefEvaluators<
    EvalFreeCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Evaluators {
    eval: EvalFreeCheckerByRef,
    value: ValueFreeConstChecker,
    quote: QuoteByRef {
        eval: EvalFreeCheckerByRef,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    },
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, FREE_CHECKER_BY_REF)
    }
}

pub(crate) const CONST_CHECKER: ByValEvaluators<
    EvalConstChecker,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Evaluators {
    eval: EvalConstChecker,
    value: ValueFreeConstChecker,
    quote: Quote {
        eval: EvalConstChecker,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    },
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, CONST_CHECKER)
    }
}

pub(crate) const CONST_CHECKER_BY_REF: ByRefEvaluators<
    EvalConstCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Evaluators {
    eval: EvalConstCheckerByRef,
    value: ValueFreeConstChecker,
    quote: QuoteByRef {
        eval: EvalConstCheckerByRef,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    },
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, CONST_CHECKER_BY_REF)
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
