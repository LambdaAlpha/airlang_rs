use super::FreeStaticFn;
use super::comp::FreeComposite;
use super::setup::FreeSetup;
use super::setup::impl_free_setup;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeStaticCompFunc {
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: FreeSetup,
}

impl FreeStaticFn<Val, Val> for FreeStaticCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), input)
    }
}

impl_free_setup!(FreeStaticCompFunc);
