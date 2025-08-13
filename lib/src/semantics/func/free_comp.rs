use super::FreeFn;
use super::comp::FreeComposite;
use super::setup::Setup;
use super::setup::impl_setup;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCompFunc {
    pub(crate) id: Symbol,
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Setup,
}

impl FreeFn<Val, Val> for FreeCompFunc {
    fn free_call(&self, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), input)
    }
}

impl_setup!(FreeCompFunc);
