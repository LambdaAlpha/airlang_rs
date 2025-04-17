use crate::{
    GenerateVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenerateMode {
    pub func: Option<Mode>,
}

impl Transformer<GenerateVal, Val> for GenerateMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_generate(&self.func, ctx, generate)
    }
}

impl From<UniMode> for GenerateMode {
    fn from(mode: UniMode) -> Self {
        Self { func: Some(Mode::Uni(mode)) }
    }
}
