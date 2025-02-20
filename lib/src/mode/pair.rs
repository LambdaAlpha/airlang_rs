use crate::{
    Pair,
    PairVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairMode {
    pub pair: Pair<Option<Mode>, Option<Mode>>,
}

impl Transformer<PairVal, Val> for PairMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_pair(&self.pair.first, &self.pair.second, ctx, pair)
    }
}

impl From<UniMode> for PairMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        PairMode { pair: Pair::new(m.clone(), m) }
    }
}
