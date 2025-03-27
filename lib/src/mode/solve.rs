use crate::{
    CodeMode,
    Solve,
    SolveVal,
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
pub struct SolveMode {
    pub code: CodeMode,
    pub solve: Solve<Option<Mode>, Option<Mode>>,
}

impl Transformer<SolveVal, Val> for SolveMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, solve: SolveVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.solve.func;
        let output = &self.solve.output;
        match self.code {
            CodeMode::Form => FormCore::transform_solve(func, output, ctx, solve),
            CodeMode::Eval => EvalCore::transform_solve(func, output, ctx, solve),
        }
    }
}

impl From<UniMode> for SolveMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { code: mode.code, solve: Solve::new(m.clone(), m) }
    }
}
