use crate::{
    ctx_access::CtxAccessor,
    transform::{
        eval::Eval,
        id::Id,
        lazy::Lazy,
    },
    transformer::Transformer,
    Val,
};

pub(crate) const EVAL: &str = "eval";
pub(crate) const ID: &str = "id";
pub(crate) const LAZY: &str = "lazy";

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Transform {
    #[default]
    Eval,
    Id,
    Lazy,
}

impl<Ctx> Transformer<Ctx, Val, Val> for Transform
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            Transform::Eval => Eval.transform(ctx, input),
            Transform::Id => Id.transform(ctx, input),
            Transform::Lazy => Lazy.transform(ctx, input),
        }
    }
}

pub(crate) mod eval;

pub(crate) mod id;

pub(crate) mod lazy;