use crate::semantics::{
    ctx_access::CtxAccessor,
    eval::Evaluator,
    eval_mode::{
        less::{
            Less,
            LessByRef,
        },
        more::{
            More,
            MoreByRef,
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
    Less,
    More,
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Value => Value.eval(ctx, input),
            EvalMode::Less => Less.eval(ctx, input),
            EvalMode::More => More.eval(ctx, input),
        }
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            EvalMode::Value => ValueByRef.eval(ctx, input),
            EvalMode::Less => LessByRef.eval(ctx, input),
            EvalMode::More => MoreByRef.eval(ctx, input),
        }
    }
}

pub(crate) mod value;

pub(crate) mod less;

pub(crate) mod more;
