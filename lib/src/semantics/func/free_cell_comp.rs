use super::FreeCellFn;
use super::FreeStaticFn;
use super::Func;
use super::Setup;
use super::comp::FreeComposite;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCellCompFunc {
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<Setup>,
}

impl FreeStaticFn<Val, Val> for FreeCellCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), input)
    }
}

impl FreeCellFn<Val, Val> for FreeCellCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.comp.call(&mut self.ctx, input)
    }
}

impl Func for FreeCellCompFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        false
    }
}
