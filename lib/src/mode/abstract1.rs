use crate::{
    Abstract,
    AbstractVal,
    CodeMode,
    UniMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AbstractMode {
    pub code: CodeMode,
    pub abstract1: Abstract<Option<Mode>, Option<Mode>>,
}

impl Transformer<AbstractVal, Val> for AbstractMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let func = &self.abstract1.func;
        let input = &self.abstract1.input;
        match self.code {
            CodeMode::Form => FormCore::transform_abstract(func, input, ctx, abstract1),
            CodeMode::Eval => EvalCore::transform_abstract(func, input, ctx, abstract1),
        }
    }
}

impl From<UniMode> for AbstractMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self {
            code: mode.code,
            abstract1: Abstract::new(m.clone(), m),
        }
    }
}
