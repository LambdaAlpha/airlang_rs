use crate::{
    semantics::{
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
            },
        },
        Val,
    },
    types::Pair,
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

impl<Ctx> Evaluator<Ctx, Val, Val> for BasicEvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            BasicEvalMode::Value => Value.eval(ctx, input),
            BasicEvalMode::Eval => Eval.eval(ctx, input),
            BasicEvalMode::Interpolate => INTERPOLATE.eval(ctx, input),
            BasicEvalMode::Inline => INLINE.eval(ctx, input),
        }
    }
}
impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for BasicEvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            BasicEvalMode::Value => ValueByRef.eval(ctx, input),
            BasicEvalMode::Eval => EvalByRef.eval(ctx, input),
            BasicEvalMode::Interpolate => INTERPOLATE_BY_REF.eval(ctx, input),
            BasicEvalMode::Inline => INLINE_BY_REF.eval(ctx, input),
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Basic(eval_mode) => eval_mode.eval(ctx, input),
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first = first.eval(ctx, pair.first);
                    let second = second.eval(ctx, pair.second);
                    let pair = Pair::new(first, second);
                    Val::Pair(Box::new(pair))
                }
                input => non_pair.eval(ctx, input),
            },
        }
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            EvalMode::Basic(eval_mode) => eval_mode.eval(ctx, input),
            EvalMode::Pair {
                first,
                second,
                non_pair,
            } => match input {
                Val::Pair(pair) => {
                    let first = first.eval(ctx, &pair.first);
                    let second = second.eval(ctx, &pair.second);
                    let pair = Pair::new(first, second);
                    Val::Pair(Box::new(pair))
                }
                input => non_pair.eval(ctx, input),
            },
        }
    }
}

const INTERPOLATE: Interpolate<Eval, Value, ValBuilder> = Interpolate {
    eval: Eval,
    value: Value,
    builder: ValBuilder,
};

const INTERPOLATE_BY_REF: InterpolateByRef<EvalByRef, ValueByRef, ValBuilder> = InterpolateByRef {
    eval: EvalByRef,
    value: ValueByRef,
    builder: ValBuilder,
};

const INLINE: Inline<Eval, Value, ValBuilder> = Inline {
    eval: Eval,
    value: Value,
    builder: ValBuilder,
};

const INLINE_BY_REF: InlineByRef<EvalByRef, ValueByRef, ValBuilder> = InlineByRef {
    eval: EvalByRef,
    value: ValueByRef,
    builder: ValBuilder,
};

pub(crate) mod value;

pub(crate) mod interpolate;

pub(crate) mod inline;

pub(crate) mod eval;
