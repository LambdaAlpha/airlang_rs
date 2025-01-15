use crate::{
    Abstract,
    AbstractVal,
    Mode,
    UniMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AbstractMode {
    Id(Id),
    Form(Abstract<Mode, Mode>),
    Eval(Abstract<Mode, Mode>),
}

impl Transformer<AbstractVal, Val> for AbstractMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            AbstractMode::Id(mode) => mode.transform_abstract(ctx, abstract1),
            AbstractMode::Form(mode) => {
                FormCore::transform_abstract(&mode.func, &mode.input, ctx, abstract1)
            }
            AbstractMode::Eval(mode) => {
                EvalCore::transform_abstract(&mode.func, &mode.input, ctx, abstract1)
            }
        }
    }
}

impl Default for AbstractMode {
    fn default() -> Self {
        Self::Eval(Abstract::default())
    }
}

impl From<UniMode> for AbstractMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => AbstractMode::Id(mode),
            UniMode::Form(mode) => AbstractMode::Form(Abstract::new(
                Mode::Uni(UniMode::Form(mode)),
                Mode::Uni(UniMode::Form(mode)),
            )),
            UniMode::Eval(mode) => AbstractMode::Eval(Abstract::new(
                Mode::Uni(UniMode::Eval(mode)),
                Mode::Uni(UniMode::Eval(mode)),
            )),
        }
    }
}
