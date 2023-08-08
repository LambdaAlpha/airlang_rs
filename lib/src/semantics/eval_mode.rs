use crate::{
    semantics::{
        ctx::CtxTrait,
        eval::Evaluator,
        eval_mode::{
            eval::Eval,
            inline::Inline,
            interpolate::Interpolate,
            value::Value,
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
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            BasicEvalMode::Value => Value.eval(ctx, input),
            BasicEvalMode::Eval => Eval.eval(ctx, input),
            BasicEvalMode::Interpolate => Interpolate.eval(ctx, input),
            BasicEvalMode::Inline => Inline.eval(ctx, input),
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxTrait,
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

pub(crate) mod value;

pub(crate) mod interpolate;

pub(crate) mod inline;

pub(crate) mod eval;
