use super::ConstFn;
use super::FreeFn;
use super::comp::DynComposite;
use super::setup::Setup;
use super::setup::impl_setup;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstCompFunc {
    pub(crate) id: Symbol,
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Setup,
}

impl FreeFn<Cfg, Val, Val> for ConstCompFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.comp.free.call(cfg, &mut self.ctx.clone(), input)
    }
}

impl ConstFn<Cfg, Val, Val, Val> for ConstCompFunc {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(cfg, &mut self.ctx.clone(), ctx.into_dyn(), input)
    }
}

impl_setup!(ConstCompFunc);
