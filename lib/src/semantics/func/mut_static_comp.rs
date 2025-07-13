use super::ConstStaticFn;
use super::FreeStaticFn;
use super::MutStaticFn;
use super::comp::DynComposite;
use super::setup::DynSetup;
use super::setup::impl_dyn_setup;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MutStaticCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: DynSetup,
}

impl FreeStaticFn<Val, Val> for MutStaticCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.free.call(&mut self.ctx.clone(), input)
    }
}

impl ConstStaticFn<Val, Val, Val> for MutStaticCompFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), ctx.into_dyn(), input)
    }
}

impl MutStaticFn<Val, Val, Val> for MutStaticCompFunc {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), DynRef::new_mut(ctx), input)
    }
}

impl_dyn_setup!(MutStaticCompFunc);
