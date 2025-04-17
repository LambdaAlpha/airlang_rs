use crate::{
    InverseVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InverseMode {
    pub func: Option<Mode>,
}

impl Transformer<InverseVal, Val> for InverseMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_inverse(&self.func, ctx, inverse)
    }
}

impl From<UniMode> for InverseMode {
    fn from(mode: UniMode) -> Self {
        Self { func: Some(Mode::Uni(mode)) }
    }
}
