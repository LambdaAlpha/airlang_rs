use crate::{
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
    pub func: Option<Mode>,
    pub input: Option<Mode>,
}

impl Transformer<CallVal, Val> for CallMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.code {
            CodeMode::Form => FormCore::transform_call(&self.func, &self.input, ctx, call),
            CodeMode::Eval => EvalCore::transform_call(&self.func, &self.input, ctx, call),
        }
    }
}

impl From<UniMode> for CallMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { code: mode.code, func: m.clone(), input: m }
    }
}
