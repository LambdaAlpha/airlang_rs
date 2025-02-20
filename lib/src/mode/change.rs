use crate::{
    Change,
    ChangeVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChangeMode {
    pub change: Change<Option<Mode>, Option<Mode>>,
}

impl Transformer<ChangeVal, Val> for ChangeMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_change(&self.change.from, &self.change.to, ctx, change)
    }
}

impl From<UniMode> for ChangeMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        ChangeMode { change: Change::new(m.clone(), m) }
    }
}
