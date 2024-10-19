use crate::{
    Ask,
    AskVal,
    Mode,
    PrimitiveMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::{
        id::Id,
        recursive::SelfMode,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AskMode<M> {
    Id,
    Form(Ask<M, M>),
    Eval(Ask<M, M>),
}

impl Transformer<AskVal, Val> for AskMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            AskMode::Id => Id.transform_ask(ctx, ask),
            AskMode::Form(mode) => FormCore::transform_ask(&mode.func, &mode.output, ctx, ask),
            AskMode::Eval(mode) => EvalCore::transform_ask(&mode.func, &mode.output, ctx, ask),
        }
    }
}

impl<M: Default> Default for AskMode<M> {
    fn default() -> Self {
        Self::Eval(Ask::default())
    }
}

impl From<PrimitiveMode> for AskMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => AskMode::Id,
            PrimitiveMode::Form => AskMode::Form(Ask::new(
                Mode::Primitive(PrimitiveMode::Form),
                Mode::Primitive(PrimitiveMode::Form),
            )),
            PrimitiveMode::Eval => AskMode::Eval(Ask::new(
                Mode::Primitive(PrimitiveMode::Eval),
                Mode::Primitive(PrimitiveMode::Eval),
            )),
        }
    }
}

impl From<PrimitiveMode> for AskMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => AskMode::Id,
            PrimitiveMode::Form => AskMode::Form(Ask::new(SelfMode::Self1, SelfMode::Self1)),
            PrimitiveMode::Eval => AskMode::Eval(Ask::new(SelfMode::Self1, SelfMode::Self1)),
        }
    }
}
