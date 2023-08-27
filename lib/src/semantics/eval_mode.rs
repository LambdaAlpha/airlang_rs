use crate::semantics::{
    ctx_access::CtxAccessor,
    eval::{
        output::OutputBuilder,
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
        inline::{
            Inline,
            InlineByRef,
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
pub(crate) enum BasicEvalMode {
    Value,
    Eval,
    Quote,
    Inline,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct EvalMode {
    pub(crate) default: BasicEvalMode,
    pub(crate) pair: Option<(BasicEvalMode, BasicEvalMode)>,
}

impl EvalMode {
    pub(crate) fn basic(eval_mode: BasicEvalMode) -> Self {
        Self {
            default: eval_mode,
            pair: None,
        }
    }
}

const QUOTE: Quote<Eval, Value, ValBuilder> = Quote {
    eval: Eval,
    value: Value,
    builder: ValBuilder,
};

const INLINE: Inline<Eval, Value, ValBuilder> = Inline {
    eval: Eval,
    value: Value,
    builder: ValBuilder,
};

impl<Ctx> Evaluator<Ctx, Val, Val> for BasicEvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, &Eval, &Value, &INLINE, &QUOTE)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(ctx, input, &Eval, &Value, &INLINE, &QUOTE, &ValBuilder)
    }
}

const QUOTE_BY_REF: QuoteByRef<EvalByRef, ValueByRef, ValBuilder> = QuoteByRef {
    eval: EvalByRef,
    value: ValueByRef,
    builder: ValBuilder,
};

const INLINE_BY_REF: InlineByRef<EvalByRef, ValueByRef, ValBuilder> = InlineByRef {
    eval: EvalByRef,
    value: ValueByRef,
    builder: ValBuilder,
};

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for BasicEvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_generic(
            ctx,
            input,
            &EvalByRef,
            &ValueByRef,
            &INLINE_BY_REF,
            &QUOTE_BY_REF,
        )
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalByRef,
            &ValueByRef,
            &INLINE_BY_REF,
            &QUOTE_BY_REF,
            &ValBuilder,
        )
    }
}

const QUOTE_FREE: Quote<EvalFree, ValueFreeConst, OpValBuilder> = Quote {
    eval: EvalFree,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

const INLINE_FREE: Inline<EvalFree, ValueFreeConst, OpValBuilder> = Inline {
    eval: EvalFree,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn eval_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFree,
            &ValueFreeConst,
            &INLINE_FREE,
            &QUOTE_FREE,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFree,
            &ValueFreeConst,
            &INLINE_FREE,
            &QUOTE_FREE,
            &OpValBuilder,
        )
    }
}

const QUOTE_FREE_BY_REF: QuoteByRef<EvalFreeByRef, ValueFreeConstByRef, OpValBuilder> =
    QuoteByRef {
        eval: EvalFreeByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

const INLINE_FREE_BY_REF: InlineByRef<EvalFreeByRef, ValueFreeConstByRef, OpValBuilder> =
    InlineByRef {
        eval: EvalFreeByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

impl BasicEvalMode {
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
            &INLINE_FREE_BY_REF,
            &QUOTE_FREE_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalFreeByRef,
            &ValueFreeConstByRef,
            &INLINE_FREE_BY_REF,
            &QUOTE_FREE_BY_REF,
            &OpValBuilder,
        )
    }
}

const QUOTE_CONST: Quote<EvalConst, ValueFreeConst, OpValBuilder> = Quote {
    eval: EvalConst,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

const INLINE_CONST: Inline<EvalConst, ValueFreeConst, OpValBuilder> = Inline {
    eval: EvalConst,
    value: ValueFreeConst,
    builder: OpValBuilder,
};

impl BasicEvalMode {
    #[allow(unused)]
    pub(crate) fn eval_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConst,
            &ValueFreeConst,
            &INLINE_CONST,
            &QUOTE_CONST,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConst,
            &ValueFreeConst,
            &INLINE_CONST,
            &QUOTE_CONST,
            &OpValBuilder,
        )
    }
}

const QUOTE_CONST_BY_REF: QuoteByRef<EvalConstByRef, ValueFreeConstByRef, OpValBuilder> =
    QuoteByRef {
        eval: EvalConstByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

const INLINE_CONST_BY_REF: InlineByRef<EvalConstByRef, ValueFreeConstByRef, OpValBuilder> =
    InlineByRef {
        eval: EvalConstByRef,
        value: ValueFreeConstByRef,
        builder: OpValBuilder,
    };

impl BasicEvalMode {
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
            &INLINE_CONST_BY_REF,
            &QUOTE_CONST_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn eval_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> Option<Val>
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalConstByRef,
            &ValueFreeConstByRef,
            &INLINE_CONST_BY_REF,
            &QUOTE_CONST_BY_REF,
            &OpValBuilder,
        )
    }
}

const QUOTE_FREE_CHECKER: Quote<EvalFreeChecker, ValueFreeConstChecker, BoolAndBuilder> = Quote {
    eval: EvalFreeChecker,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

const INLINE_FREE_CHECKER: Inline<EvalFreeChecker, ValueFreeConstChecker, BoolAndBuilder> =
    Inline {
        eval: EvalFreeChecker,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    };

impl BasicEvalMode {
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
            &INLINE_FREE_CHECKER,
            &QUOTE_FREE_CHECKER,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_free<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalFreeChecker,
            &ValueFreeConstChecker,
            &INLINE_FREE_CHECKER,
            &QUOTE_FREE_CHECKER,
            &BoolAndBuilder,
        )
    }
}

const QUOTE_FREE_CHECKER_BY_REF: QuoteByRef<
    EvalFreeCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = QuoteByRef {
    eval: EvalFreeCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

const INLINE_FREE_CHECKER_BY_REF: InlineByRef<
    EvalFreeCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = InlineByRef {
    eval: EvalFreeCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl BasicEvalMode {
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
            &INLINE_FREE_CHECKER_BY_REF,
            &QUOTE_FREE_CHECKER_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_free_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalFreeCheckerByRef,
            &ValueFreeConstChecker,
            &INLINE_FREE_CHECKER_BY_REF,
            &QUOTE_FREE_CHECKER_BY_REF,
            &BoolAndBuilder,
        )
    }
}

const QUOTE_CONST_CHECKER: Quote<EvalConstChecker, ValueFreeConstChecker, BoolAndBuilder> = Quote {
    eval: EvalConstChecker,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

const INLINE_CONST_CHECKER: Inline<EvalConstChecker, ValueFreeConstChecker, BoolAndBuilder> =
    Inline {
        eval: EvalConstChecker,
        value: ValueFreeConstChecker,
        builder: BoolAndBuilder,
    };

impl BasicEvalMode {
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
            &INLINE_CONST_CHECKER,
            &QUOTE_CONST_CHECKER,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_const<Ctx>(&self, ctx: &mut Ctx, input: Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_generic(
            ctx,
            input,
            &EvalConstChecker,
            &ValueFreeConstChecker,
            &INLINE_CONST_CHECKER,
            &QUOTE_CONST_CHECKER,
            &BoolAndBuilder,
        )
    }
}

const QUOTE_CONST_CHECKER_BY_REF: QuoteByRef<
    EvalConstCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = QuoteByRef {
    eval: EvalConstCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

const INLINE_CONST_CHECKER_BY_REF: InlineByRef<
    EvalConstCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = InlineByRef {
    eval: EvalConstCheckerByRef,
    value: ValueFreeConstChecker,
    builder: BoolAndBuilder,
};

impl BasicEvalMode {
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
            &INLINE_CONST_CHECKER_BY_REF,
            &QUOTE_CONST_CHECKER_BY_REF,
        )
    }
}

impl EvalMode {
    pub(crate) fn is_const_by_ref<Ctx>(&self, ctx: &mut Ctx, input: &Val) -> bool
    where
        Ctx: CtxAccessor,
    {
        self.eval_by_ref_generic(
            ctx,
            input,
            &EvalConstCheckerByRef,
            &ValueFreeConstChecker,
            &INLINE_CONST_CHECKER_BY_REF,
            &QUOTE_CONST_CHECKER_BY_REF,
            &BoolAndBuilder,
        )
    }
}

impl BasicEvalMode {
    fn eval_generic<Ctx, Input, Output, Eval, Value, Inline, Quote>(
        &self,
        ctx: &mut Ctx,
        input: Input,
        eval: &Eval,
        value: &Value,
        inline: &Inline,
        quote: &Quote,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Input, Output>,
        Value: Evaluator<Ctx, Input, Output>,
        Inline: Evaluator<Ctx, Input, Output>,
        Quote: Evaluator<Ctx, Input, Output>,
    {
        match self {
            BasicEvalMode::Value => value.eval(ctx, input),
            BasicEvalMode::Eval => eval.eval(ctx, input),
            BasicEvalMode::Quote => quote.eval(ctx, input),
            BasicEvalMode::Inline => inline.eval(ctx, input),
        }
    }
}

impl EvalMode {
    #[allow(clippy::too_many_arguments)]
    fn eval_generic<Ctx, Output, Eval, Value, Inline, Quote, Builder>(
        &self,
        ctx: &mut Ctx,
        input: Val,
        eval: &Eval,
        value: &Value,
        inline: &Inline,
        quote: &Quote,
        builder: &Builder,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Val, Output>,
        Value: Evaluator<Ctx, Val, Output>,
        Inline: Evaluator<Ctx, Val, Output>,
        Quote: Evaluator<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        match input {
            Val::Pair(pair) => {
                let (first, second) = match &self.pair {
                    None => (self.default, self.default),
                    Some((first, second)) => (*first, *second),
                };
                let first = first.eval_generic(ctx, pair.first, eval, value, inline, quote);
                let second = second.eval_generic(ctx, pair.second, eval, value, inline, quote);
                builder.from_pair(first, second)
            }
            input => self
                .default
                .eval_generic(ctx, input, eval, value, inline, quote),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn eval_by_ref_generic<'a, Ctx, Output, Eval, Value, Inline, Quote, Builder>(
        &self,
        ctx: &mut Ctx,
        input: &'a Val,
        eval: &Eval,
        value: &Value,
        inline: &Inline,
        quote: &Quote,
        builder: &Builder,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, &'a Val, Output>,
        Value: Evaluator<Ctx, &'a Val, Output>,
        Inline: Evaluator<Ctx, &'a Val, Output>,
        Quote: Evaluator<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        match input {
            Val::Pair(pair) => {
                let (first, second) = match &self.pair {
                    None => (self.default, self.default),
                    Some((first, second)) => (*first, *second),
                };
                let first = first.eval_generic(ctx, &pair.first, eval, value, inline, quote);
                let second = second.eval_generic(ctx, &pair.second, eval, value, inline, quote);
                builder.from_pair(first, second)
            }
            input => self
                .default
                .eval_generic(ctx, input, eval, value, inline, quote),
        }
    }
}

pub(crate) mod value;

pub(crate) mod quote;

pub(crate) mod inline;

pub(crate) mod eval;
