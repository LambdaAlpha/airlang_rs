use crate::{
    AbstractVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractMode {
    pub func: Option<Mode>,
}

impl Transformer<AbstractVal, Val> for AbstractMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_abstract(&self.func, ctx, abstract1)
    }
}

impl From<UniMode> for AbstractMode {
    fn from(mode: UniMode) -> Self {
        Self { func: Some(Mode::Uni(mode)) }
    }
}
