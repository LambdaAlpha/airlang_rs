use crate::PairVal;
use crate::UniMode;
use crate::Val;
use crate::core::FormCore;
use crate::ctx::ref1::CtxMeta;
use crate::mode::Mode;
use crate::transformer::Transformer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PairMode {
    pub first: Option<Mode>,
    pub second: Option<Mode>,
}

impl Transformer<PairVal, Val> for PairMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_pair(&self.first, &self.second, ctx, pair)
    }
}

impl From<UniMode> for PairMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { first: m.clone(), second: m }
    }
}
