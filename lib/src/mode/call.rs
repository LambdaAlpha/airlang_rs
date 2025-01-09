use crate::{
    Call,
    CallVal,
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
pub enum CallMode<M> {
    Id,
    Form(Call<M, M>),
    Eval(Call<M, M>),
}

impl Transformer<CallVal, Val> for CallMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            CallMode::Id => Id.transform_call(ctx, call),
            CallMode::Form(mode) => FormCore::transform_call(&mode.func, &mode.input, ctx, call),
            CallMode::Eval(mode) => EvalCore::transform_call(&mode.func, &mode.input, ctx, call),
        }
    }
}

impl<M: Default> Default for CallMode<M> {
    fn default() -> Self {
        Self::Eval(Call::default())
    }
}

impl From<PrimitiveMode> for CallMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => CallMode::Id,
            PrimitiveMode::Form(mode) => CallMode::Form(Call::new(
                Mode::Primitive(PrimitiveMode::Form(mode)),
                Mode::Primitive(PrimitiveMode::Form(mode)),
            )),
            PrimitiveMode::Eval(mode) => CallMode::Eval(Call::new(
                Mode::Primitive(PrimitiveMode::Eval(mode)),
                Mode::Primitive(PrimitiveMode::Eval(mode)),
            )),
        }
    }
}

impl From<PrimitiveMode> for CallMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => CallMode::Id,
            PrimitiveMode::Form(_) => CallMode::Form(Call::new(SelfMode::Self1, SelfMode::Self1)),
            PrimitiveMode::Eval(_) => CallMode::Eval(Call::new(SelfMode::Self1, SelfMode::Self1)),
        }
    }
}
