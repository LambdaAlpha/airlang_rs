use crate::{
    ctx_access::CtxAccessor,
    eval::Evaluator,
    eval_mode::{
        eager::{
            Eager,
            EagerByRef,
        },
        id::{
            Id,
            IdByRef,
        },
        lazy::{
            Lazy,
            LazyByRef,
        },
    },
    Val,
};

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum EvalMode {
    Id,
    Lazy,
    #[default]
    Eager,
}

impl<Ctx> Evaluator<Ctx, Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            EvalMode::Id => Id.eval(ctx, input),
            EvalMode::Lazy => Lazy.eval(ctx, input),
            EvalMode::Eager => Eager.eval(ctx, input),
        }
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            EvalMode::Id => IdByRef.eval(ctx, input),
            EvalMode::Lazy => LazyByRef.eval(ctx, input),
            EvalMode::Eager => EagerByRef.eval(ctx, input),
        }
    }
}

pub(crate) mod id;

pub(crate) mod lazy;

pub(crate) mod eager;
