use crate::{
    Change,
    ChangeVal,
    Mode,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ChangeMode {
    Id(Id),
    Form(Change<Mode, Mode>),
}

impl Transformer<ChangeVal, Val> for ChangeMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            ChangeMode::Id(mode) => mode.transform_change(ctx, change),
            ChangeMode::Form(mode) => FormCore::transform_change(&mode.from, &mode.to, ctx, change),
        }
    }
}

impl Default for ChangeMode {
    fn default() -> Self {
        Self::Form(Change::default())
    }
}

impl From<UniMode> for ChangeMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => ChangeMode::Id(mode),
            UniMode::Form(mode) => ChangeMode::Form(Change::new(
                Mode::Uni(UniMode::Form(mode)),
                Mode::Uni(UniMode::Form(mode)),
            )),
            UniMode::Eval(mode) => ChangeMode::Form(Change::new(
                Mode::Uni(UniMode::Eval(mode)),
                Mode::Uni(UniMode::Eval(mode)),
            )),
        }
    }
}
