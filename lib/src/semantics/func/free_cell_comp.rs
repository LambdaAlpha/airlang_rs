use super::FreeCellFn;
use super::FreeStaticFn;
use super::Setup;
use super::comp::FreeComposite;
use super::setup::impl_setup;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCellCompFunc {
    pub(crate) id: Symbol,
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Setup,
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

impl_setup!(FreeCellCompFunc);
