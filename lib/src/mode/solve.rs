use crate::{
    Solve,
    SolveVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SolveMode {
    pub solve: Solve<Option<Mode>>,
}

impl Transformer<SolveVal, Val> for SolveMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, solve: SolveVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.solve.func;
        FormCore::transform_solve(func, ctx, solve)
    }
}

impl From<UniMode> for SolveMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { solve: Solve::new(m) }
    }
}
