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
        interpolate::{
            Interpolate,
            InterpolateByRef,
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
    Interpolate,
    Inline,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum EvalMode {
    Basic(BasicEvalMode),
    Pair {
        first: BasicEvalMode,
        second: BasicEvalMode,
        non_pair: BasicEvalMode,
    },
}

const INTERPOLATE: Interpolate<Eval, Value, ValBuilder> = Interpolate {
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
        self.eval_generic(ctx, input, &Eval, &Value, &INLINE, &INTERPOLATE)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_generic(
            ctx,
            input,
            &Eval,
            &Value,
            &INLINE,
            &INTERPOLATE,
            &ValBuilder,
        )
    }
}

const INTERPOLATE_BY_REF: InterpolateByRef<EvalByRef, ValueByRef, ValBuilder> = InterpolateByRef {
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
            &INTERPOLATE_BY_REF,
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
            &INTERPOLATE_BY_REF,
            &ValBuilder,
        )
    }
}

const INTERPOLATE_FREE: Interpolate<EvalFree, ValueFreeConst, OpValBuilder> = Interpolate {
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
            &INTERPOLATE_FREE,
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
            &INTERPOLATE_FREE,
            &OpValBuilder,
        )
    }
}

const INTERPOLATE_FREE_BY_REF: InterpolateByRef<EvalFreeByRef, ValueFreeConstByRef, OpValBuilder> =
    InterpolateByRef {
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
            &INTERPOLATE_FREE_BY_REF,
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
            &INTERPOLATE_FREE_BY_REF,
            &OpValBuilder,
        )
    }
}

const INTERPOLATE_CONST: Interpolate<EvalConst, ValueFreeConst, OpValBuilder> = Interpolate {
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
            &INTERPOLATE_CONST,
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
            &INTERPOLATE_CONST,
            &OpValBuilder,
        )
    }
}

const INTERPOLATE_CONST_BY_REF: InterpolateByRef<
    EvalConstByRef,
    ValueFreeConstByRef,
    OpValBuilder,
> = InterpolateByRef {
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
            &INTERPOLATE_CONST_BY_REF,
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
            &INTERPOLATE_CONST_BY_REF,
            &OpValBuilder,
        )
    }
}

const INTERPOLATE_FREE_CHECKER: Interpolate<
    EvalFreeChecker,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Interpolate {
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
            &INTERPOLATE_FREE_CHECKER,
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
            &INTERPOLATE_FREE_CHECKER,
            &BoolAndBuilder,
        )
    }
}

const INTERPOLATE_FREE_CHECKER_BY_REF: InterpolateByRef<
    EvalFreeCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = InterpolateByRef {
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
            &INTERPOLATE_FREE_CHECKER_BY_REF,
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
            &INTERPOLATE_FREE_CHECKER_BY_REF,
            &BoolAndBuilder,
        )
    }
}

const INTERPOLATE_CONST_CHECKER: Interpolate<
    EvalConstChecker,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = Interpolate {
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
            &INTERPOLATE_CONST_CHECKER,
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
            &INTERPOLATE_CONST_CHECKER,
            &BoolAndBuilder,
        )
    }
}

const INTERPOLATE_CONST_CHECKER_BY_REF: InterpolateByRef<
    EvalConstCheckerByRef,
    ValueFreeConstChecker,
    BoolAndBuilder,
> = InterpolateByRef {
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
            &INTERPOLATE_CONST_CHECKER_BY_REF,
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
            &INTERPOLATE_CONST_CHECKER_BY_REF,
            &BoolAndBuilder,
        )
    }
}

impl BasicEvalMode {
    fn eval_generic<Ctx, Input, Output, Eval, Value, Inline, Interpolate>(
        &self,
        ctx: &mut Ctx,
        input: Input,
        eval: &Eval,
        value: &Value,
        inline: &Inline,
        interpolate: &Interpolate,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Input, Output>,
        Value: Evaluator<Ctx, Input, Output>,
        Inline: Evaluator<Ctx, Input, Output>,
        Interpolate: Evaluator<Ctx, Input, Output>,
    {
        match self {
            BasicEvalMode::Value => value.eval(ctx, input),
            BasicEvalMode::Eval => eval.eval(ctx, input),
            BasicEvalMode::Interpolate => interpolate.eval(ctx, input),
            BasicEvalMode::Inline => inline.eval(ctx, input),
        }
    }
}

impl EvalMode {
    #[allow(clippy::too_many_arguments)]
    fn eval_generic<Ctx, Output, Eval, Value, Inline, Interpolate, Builder>(
        &self,
        ctx: &mut Ctx,
        input: Val,
        eval: &Eval,
        value: &Value,
        inline: &Inline,
        interpolate: &Interpolate,
        builder: &Builder,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, Val, Output>,
        Value: Evaluator<Ctx, Val, Output>,
        Inline: Evaluator<Ctx, Val, Output>,
        Interpolate: Evaluator<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        match self {
            EvalMode::Basic(eval_mode) => {
                eval_mode.eval_generic(ctx, input, eval, value, inline, interpolate)
            }
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first =
                        first.eval_generic(ctx, pair.first, eval, value, inline, interpolate);
                    let second =
                        second.eval_generic(ctx, pair.second, eval, value, inline, interpolate);
                    builder.from_pair(first, second)
                }
                input => non_pair.eval_generic(ctx, input, eval, value, inline, interpolate),
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn eval_by_ref_generic<'a, Ctx, Output, Eval, Value, Inline, Interpolate, Builder>(
        &self,
        ctx: &mut Ctx,
        input: &'a Val,
        eval: &Eval,
        value: &Value,
        inline: &Inline,
        interpolate: &Interpolate,
        builder: &Builder,
    ) -> Output
    where
        Ctx: CtxAccessor,
        Eval: Evaluator<Ctx, &'a Val, Output>,
        Value: Evaluator<Ctx, &'a Val, Output>,
        Inline: Evaluator<Ctx, &'a Val, Output>,
        Interpolate: Evaluator<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        match self {
            EvalMode::Basic(eval_mode) => {
                eval_mode.eval_generic(ctx, input, eval, value, inline, interpolate)
            }
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first =
                        first.eval_generic(ctx, &pair.first, eval, value, inline, interpolate);
                    let second =
                        second.eval_generic(ctx, &pair.second, eval, value, inline, interpolate);
                    builder.from_pair(first, second)
                }
                input => non_pair.eval_generic(ctx, input, eval, value, inline, interpolate),
            },
        }
    }
}

pub(crate) mod value;

pub(crate) mod interpolate;

pub(crate) mod inline;

pub(crate) mod eval;
