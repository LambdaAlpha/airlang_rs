use crate::{
    Call,
    CallVal,
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
pub struct CallMode {
    pub code: CodeMode,
    pub call: Call<Option<Mode>, Option<Mode>>,
}

impl Transformer<CallVal, Val> for CallMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let func = &self.call.func;
        let input = &self.call.input;
        match self.code {
            CodeMode::Form => FormCore::transform_call(func, input, ctx, call),
            CodeMode::Eval => EvalCore::transform_call(func, input, ctx, call),
        }
    }
}

impl From<UniMode> for CallMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self {
            code: mode.code,
            call: Call::new(m.clone(), m),
        }
    }
}
