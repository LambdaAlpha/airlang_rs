use super::ConstStaticFn;
use super::FreeStaticFn;
use super::Func;
use super::MutStaticFn;
use super::Setup;
use super::comp::DynComposite;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MutStaticCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<Setup>,
    pub(crate) ctx_explicit: bool,
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

impl Func for MutStaticCompFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }
}
