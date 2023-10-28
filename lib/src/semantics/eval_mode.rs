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

pub(crate) const QUOTE: Quote<Eval, Value, ValBuilder> = Quote {
    eval: Eval,
    value: Value,
    builder: ValBuilder,
};

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, &Eval, &Value, &QUOTE)
    }
}

pub(crate) const QUOTE_BY_REF: QuoteByRef<EvalByRef, ValueByRef, ValBuilder> = QuoteByRef {
    eval: EvalByRef,
    value: ValueByRef,
    builder: ValBuilder,
};

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_generic(ctx, input, &EvalByRef, &ValueByRef, &QUOTE_BY_REF)
    }
}

pub(crate) const QUOTE_FREE: Quote<EvalFree, ValueFreeConst, OpValBuilder> = Quote {
    eval: EvalFree,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, &EvalFree, &ValueFreeConst, &QUOTE_FREE)
    }
}

pub(crate) const QUOTE_FREE_BY_REF: QuoteByRef<EvalFreeByRef, ValueFreeConstByRef, OpValBuilder> =
    QuoteByRef {
        eval: EvalFreeByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeByRef,
            &ValueFreeConstByRef,
            &QUOTE_FREE_BY_REF,
        )
    }
}

pub(crate) const QUOTE_CONST: Quote<EvalConst, ValueFreeConst, OpValBuilder> = Quote {
    eval: EvalConst,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(ctx, input, &EvalConst, &ValueFreeConst, &QUOTE_CONST)
    }
}

pub(crate) const QUOTE_CONST_BY_REF: QuoteByRef<EvalConstByRef, ValueFreeConstByRef, OpValBuilder> =
    QuoteByRef {
        eval: EvalConstByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstByRef,
            &ValueFreeConstByRef,
            &QUOTE_CONST_BY_REF,
        )
    }
}

pub(crate) const QUOTE_FREE_CHECKER: Quote<EvalFreeChecker, ValueFreeConstChecker, BoolAndBuilder> =
    Quote {
        eval: EvalFreeChecker,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    };

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeChecker,
            &ValueFreeConstChecker,
            &QUOTE_FREE_CHECKER,
        )
    }
}

pub(crate) const QUOTE_FREE_CHECKER_BY_REF: QuoteByRef<
    EvalFreeCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = QuoteByRef {
    eval: EvalFreeCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeCheckerByRef,
            &ValueFreeConstChecker,
            &QUOTE_FREE_CHECKER_BY_REF,
        )
    }
}

pub(crate) const QUOTE_CONST_CHECKER: Quote<
    EvalConstChecker,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Quote {
    eval: EvalConstChecker,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstChecker,
            &ValueFreeConstChecker,
            &QUOTE_CONST_CHECKER,
        )
    }
}

pub(crate) const QUOTE_CONST_CHECKER_BY_REF: QuoteByRef<
    EvalConstCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = QuoteByRef {
    eval: EvalConstCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl EvalMode {
    #[allow(unused)]
    pub(crate) fn is_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstCheckerByRef,
            &ValueFreeConstChecker,
            &QUOTE_CONST_CHECKER_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_generic<Ctx, Input, Output, Eval, Value, Quote>(
        &self,
        ctx: &mut Ctx,
        input: Input,
        eval: &Eval,
        value: &Value,
        quote: &Quote,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Input, Output>,
        Value: Evaluator<Ctx, Input, Output>,
        Quote: Evaluator<Ctx, Input, Output>,
    {
        match self {
            EvalMode::Value => value.eval(ctx, input),
            EvalMode::Eval => eval.eval(ctx, input),
            EvalMode::Quote => quote.eval(ctx, input),
        }
    }
}

pub(crate) mod value;

pub(crate) mod quote;

pub(crate) mod eval;
