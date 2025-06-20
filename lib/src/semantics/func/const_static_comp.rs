use super::ConstStaticFn;
use super::FreeStaticFn;
use super::Func;
use super::Setup;
use super::comp::DynComposite;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstStaticCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<Setup>,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for ConstStaticCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.free.call(&mut self.ctx.clone(), input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstStaticCompFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), ctx.into_dyn(), input)
    }
}

impl Func for ConstStaticCompFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }
}
