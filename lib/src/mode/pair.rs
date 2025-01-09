use crate::{
    Mode,
    Pair,
    PairVal,
    PrimitiveMode,
    Val,
    core::FormCore,
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
pub enum PairMode<M> {
    Id,
    Form(Pair<M, M>),
}

impl Transformer<PairVal, Val> for PairMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PairMode::Id => Id.transform_pair(ctx, pair),
            PairMode::Form(mode) => FormCore::transform_pair(&mode.first, &mode.second, ctx, pair),
        }
    }
}

impl<M: Default> Default for PairMode<M> {
    fn default() -> Self {
        Self::Form(Pair::default())
    }
}

impl From<PrimitiveMode> for PairMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => PairMode::Id,
            PrimitiveMode::Form(mode) => PairMode::Form(Pair::new(
                Mode::Primitive(PrimitiveMode::Form(mode)),
                Mode::Primitive(PrimitiveMode::Form(mode)),
            )),
            PrimitiveMode::Eval(mode) => PairMode::Form(Pair::new(
                Mode::Primitive(PrimitiveMode::Eval(mode)),
                Mode::Primitive(PrimitiveMode::Eval(mode)),
            )),
        }
    }
}

impl From<PrimitiveMode> for PairMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => PairMode::Id,
            PrimitiveMode::Form(_) => PairMode::Form(Pair::new(SelfMode::Self1, SelfMode::Self1)),
            PrimitiveMode::Eval(_) => PairMode::Form(Pair::new(SelfMode::Self1, SelfMode::Self1)),
        }
    }
}
