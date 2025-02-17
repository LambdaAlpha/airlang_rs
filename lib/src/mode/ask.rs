use crate::{
    Ask,
    AskVal,
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
pub struct AskMode {
    pub code: CodeMode,
    pub ask: Ask<Option<Mode>, Option<Mode>>,
}

impl Transformer<AskVal, Val> for AskMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let func = &self.ask.func;
        let output = &self.ask.output;
        match self.code {
            CodeMode::Form => FormCore::transform_ask(func, output, ctx, ask),
            CodeMode::Eval => EvalCore::transform_ask(func, output, ctx, ask),
        }
    }
}

impl From<UniMode> for AskMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self {
            code: mode.code,
            ask: Ask::new(m.clone(), m),
        }
    }
}
