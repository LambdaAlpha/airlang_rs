use crate::{
    EquivVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EquivMode {
    pub func: Option<Mode>,
}

impl Transformer<EquivVal, Val> for EquivMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_equiv(&self.func, ctx, equiv)
    }
}

impl From<UniMode> for EquivMode {
    fn from(mode: UniMode) -> Self {
        Self { func: Some(Mode::Uni(mode)) }
    }
}
