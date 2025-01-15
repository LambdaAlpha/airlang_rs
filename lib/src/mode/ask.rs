use crate::{
    Ask,
    AskVal,
    Mode,
    UniMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AskMode {
    Id(Id),
    Form(Ask<Mode, Mode>),
    Eval(Ask<Mode, Mode>),
}

impl Transformer<AskVal, Val> for AskMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            AskMode::Id(mode) => mode.transform_ask(ctx, ask),
            AskMode::Form(mode) => FormCore::transform_ask(&mode.func, &mode.output, ctx, ask),
            AskMode::Eval(mode) => EvalCore::transform_ask(&mode.func, &mode.output, ctx, ask),
        }
    }
}

impl Default for AskMode {
    fn default() -> Self {
        Self::Eval(Ask::default())
    }
}

impl From<UniMode> for AskMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => AskMode::Id(mode),
            UniMode::Form(mode) => AskMode::Form(Ask::new(
                Mode::Uni(UniMode::Form(mode)),
                Mode::Uni(UniMode::Form(mode)),
            )),
            UniMode::Eval(mode) => AskMode::Eval(Ask::new(
                Mode::Uni(UniMode::Eval(mode)),
                Mode::Uni(UniMode::Eval(mode)),
            )),
        }
    }
}
