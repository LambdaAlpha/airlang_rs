use crate::{
    Call,
    CallVal,
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
pub enum CallMode {
    Id(Id),
    Form(Call<Mode, Mode>),
    Eval(Call<Mode, Mode>),
}

impl Transformer<CallVal, Val> for CallMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            CallMode::Id(mode) => mode.transform_call(ctx, call),
            CallMode::Form(mode) => FormCore::transform_call(&mode.func, &mode.input, ctx, call),
            CallMode::Eval(mode) => EvalCore::transform_call(&mode.func, &mode.input, ctx, call),
        }
    }
}

impl Default for CallMode {
    fn default() -> Self {
        Self::Eval(Call::default())
    }
}

impl From<UniMode> for CallMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => CallMode::Id(mode),
            UniMode::Form(mode) => CallMode::Form(Call::new(
                Mode::Uni(UniMode::Form(mode)),
                Mode::Uni(UniMode::Form(mode)),
            )),
            UniMode::Eval(mode) => CallMode::Eval(Call::new(
                Mode::Uni(UniMode::Eval(mode)),
                Mode::Uni(UniMode::Eval(mode)),
            )),
        }
    }
}
