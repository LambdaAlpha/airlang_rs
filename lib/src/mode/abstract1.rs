use crate::{
    Abstract,
    AbstractVal,
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
pub enum AbstractMode<M> {
    Id,
    Form(Abstract<M, M>),
    Eval(Abstract<M, M>),
}

impl Transformer<AbstractVal, Val> for AbstractMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            AbstractMode::Id => Id.transform_abstract(ctx, abstract1),
            AbstractMode::Form(mode) => {
                FormCore::transform_abstract(&mode.func, &mode.input, ctx, abstract1)
            }
            AbstractMode::Eval(mode) => {
                EvalCore::transform_abstract(&mode.func, &mode.input, ctx, abstract1)
            }
        }
    }
}

impl<M: Default> Default for AbstractMode<M> {
    fn default() -> Self {
        Self::Eval(Abstract::default())
    }
}

impl From<PrimitiveMode> for AbstractMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => AbstractMode::Id,
            PrimitiveMode::Form(mode) => AbstractMode::Form(Abstract::new(
                Mode::Primitive(PrimitiveMode::Form(mode)),
                Mode::Primitive(PrimitiveMode::Form(mode)),
            )),
            PrimitiveMode::Eval(mode) => AbstractMode::Eval(Abstract::new(
                Mode::Primitive(PrimitiveMode::Eval(mode)),
                Mode::Primitive(PrimitiveMode::Eval(mode)),
            )),
        }
    }
}

impl From<PrimitiveMode> for AbstractMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => AbstractMode::Id,
            PrimitiveMode::Form(_) => {
                AbstractMode::Form(Abstract::new(SelfMode::Self1, SelfMode::Self1))
            }
            PrimitiveMode::Eval(_) => {
                AbstractMode::Eval(Abstract::new(SelfMode::Self1, SelfMode::Self1))
            }
        }
    }
}
