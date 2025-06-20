use super::ConstCellFn;
use super::ConstStaticFn;
use super::FreeCellFn;
use super::FreeStaticFn;
use super::Func;
use super::Setup;
use super::comp::DynComposite;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstCellCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<Setup>,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for ConstCellCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.free.call(&mut self.ctx.clone(), input)
    }
}

impl FreeCellFn<Val, Val> for ConstCellCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.comp.free.call(&mut self.ctx, input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstCellCompFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), ctx.into_dyn(), input)
    }
}

impl ConstCellFn<Val, Val, Val> for ConstCellCompFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(&mut self.ctx, ctx.into_dyn(), input)
    }
}

impl Func for ConstCellCompFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }
}
