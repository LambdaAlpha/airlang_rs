use crate::{
    Adapt,
    AdaptVal,
    Mode,
    PrimitiveMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::{
        id::Id,
        recursive::SelfMode,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AdaptMode<M> {
    Id,
    Form(Adapt<M, M>),
    Eval(Adapt<M, M>),
}

impl Transformer<AdaptVal, Val> for AdaptMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, adapt: AdaptVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            AdaptMode::Id => Id.transform_adapt(ctx, adapt),
            AdaptMode::Form(mode) => FormCore::transform_adapt(&mode.spec, &mode.value, ctx, adapt),
            AdaptMode::Eval(mode) => EvalCore::transform_adapt(&mode.spec, &mode.value, ctx, adapt),
        }
    }
}

impl<M: Default> Default for AdaptMode<M> {
    fn default() -> Self {
        Self::Eval(Adapt::default())
    }
}

impl From<PrimitiveMode> for AdaptMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => AdaptMode::Id,
            PrimitiveMode::Form => AdaptMode::Form(Adapt::new(
                Mode::Primitive(PrimitiveMode::Form),
                Mode::Primitive(PrimitiveMode::Form),
            )),
            PrimitiveMode::Eval => AdaptMode::Eval(Adapt::new(
                Mode::Primitive(PrimitiveMode::Form),
                Mode::Primitive(PrimitiveMode::Form),
            )),
        }
    }
}

impl From<PrimitiveMode> for AdaptMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => AdaptMode::Id,
            PrimitiveMode::Form => AdaptMode::Form(Adapt::new(SelfMode::Self1, SelfMode::Self1)),
            PrimitiveMode::Eval => AdaptMode::Eval(Adapt::new(SelfMode::Self1, SelfMode::Self1)),
        }
    }
}
