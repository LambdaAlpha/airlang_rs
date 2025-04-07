use crate::{
    Generate,
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
    pub generate: Generate<Option<Mode>>,
}

impl Transformer<GenerateVal, Val> for GenerateMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.generate.func;
        FormCore::transform_generate(func, ctx, generate)
    }
}

impl From<UniMode> for GenerateMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { generate: Generate::new(m) }
    }
}
