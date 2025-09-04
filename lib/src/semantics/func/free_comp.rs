use super::FreeFn;
use super::Setup;
use super::comp::FreeComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCompFunc {
    pub(crate) id: Symbol,
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<FuncVal>,
}

impl FreeFn<Cfg, Val, Val> for FreeCompFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.comp.call(cfg, &mut self.ctx.clone(), input)
    }
}

impl Setup for FreeCompFunc {
    fn setup(&self) -> Option<&FuncVal> {
        self.setup.as_ref()
    }
}
