use super::ConstFn;
use super::FreeFn;
use super::Setup;
use super::comp::DynComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ConstCompFunc {
    pub(crate) id: Symbol,
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<FuncVal>,
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

impl Setup for ConstCompFunc {
    fn setup(&self) -> Option<&FuncVal> {
        self.setup.as_ref()
    }
}
