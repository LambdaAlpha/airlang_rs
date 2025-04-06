use crate::{
    Equiv,
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
    pub equiv: Equiv<Option<Mode>>,
}

impl Transformer<EquivVal, Val> for EquivMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.equiv.func;
        FormCore::transform_equiv(func, ctx, equiv)
    }
}

impl From<UniMode> for EquivMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { equiv: Equiv::new(m) }
    }
}
