use crate::{
    Mode,
    Pair,
    PairVal,
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
pub enum PairMode {
    Id(Id),
    Form(Pair<Mode, Mode>),
}

impl Transformer<PairVal, Val> for PairMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PairMode::Id(mode) => mode.transform_pair(ctx, pair),
            PairMode::Form(mode) => FormCore::transform_pair(&mode.first, &mode.second, ctx, pair),
        }
    }
}

impl Default for PairMode {
    fn default() -> Self {
        Self::Form(Pair::default())
    }
}

impl From<UniMode> for PairMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => PairMode::Id(mode),
            UniMode::Form(mode) => PairMode::Form(Pair::new(
                Mode::Uni(UniMode::Form(mode)),
                Mode::Uni(UniMode::Form(mode)),
            )),
            UniMode::Eval(mode) => PairMode::Form(Pair::new(
                Mode::Uni(UniMode::Eval(mode)),
                Mode::Uni(UniMode::Eval(mode)),
            )),
        }
    }
}
