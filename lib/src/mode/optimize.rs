use crate::{
    CodeMode,
    Optimize,
    OptimizeVal,
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
pub struct OptimizeMode {
    pub code: CodeMode,
    pub optimize: Optimize<Option<Mode>, Option<Mode>>,
}

impl Transformer<OptimizeVal, Val> for OptimizeMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, optimize: OptimizeVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.optimize.func;
        let input = &self.optimize.input;
        match self.code {
            CodeMode::Form => FormCore::transform_optimize(func, input, ctx, optimize),
            CodeMode::Eval => EvalCore::transform_optimize(func, input, ctx, optimize),
        }
    }
}

impl From<UniMode> for OptimizeMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { code: mode.code, optimize: Optimize::new(m.clone(), m) }
    }
}
