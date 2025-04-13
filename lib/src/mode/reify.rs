use crate::{
    Reify,
    ReifyVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReifyMode {
    pub reify: Reify<Option<Mode>>,
}

impl Transformer<ReifyVal, Val> for ReifyMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, reify: ReifyVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.reify.func;
        FormCore::transform_reify(func, ctx, reify)
    }
}

impl From<UniMode> for ReifyMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { reify: Reify::new(m) }
    }
}
