use crate::{
    Optimize,
    OptimizeVal,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OptimizeMode {
    pub optimize: Optimize<Option<Mode>>,
}

impl Transformer<OptimizeVal, Val> for OptimizeMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, optimize: OptimizeVal) -> Val
    where Ctx: CtxMeta<'a> {
        let func = &self.optimize.func;
        FormCore::transform_optimize(func, ctx, optimize)
    }
}

impl From<UniMode> for OptimizeMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        Self { optimize: Optimize::new(m) }
    }
}
