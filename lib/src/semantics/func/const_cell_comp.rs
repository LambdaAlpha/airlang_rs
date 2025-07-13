use super::ConstCellFn;
use super::ConstStaticFn;
use super::FreeCellFn;
use super::FreeStaticFn;
use super::comp::DynComposite;
use super::setup::DynSetup;
use super::setup::impl_dyn_setup;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstCellCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: DynSetup,
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

impl_dyn_setup!(ConstCellCompFunc);
