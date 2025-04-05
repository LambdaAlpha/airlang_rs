use crate::{
    Inverse,
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
    pub inverse: Inverse<Option<Mode>>,
}

impl Transformer<InverseVal, Val> for InverseMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.inverse.func;
        FormCore::transform_inverse(func, ctx, inverse)
    }
}

impl From<UniMode> for InverseMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { inverse: Inverse::new(m) }
    }
}
