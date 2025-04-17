use crate::{
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
    pub from: Option<Mode>,
    pub to: Option<Mode>,
}

impl Transformer<ChangeVal, Val> for ChangeMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_change(&self.from, &self.to, ctx, change)
    }
}

impl From<UniMode> for ChangeMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { from: m.clone(), to: m }
    }
}
