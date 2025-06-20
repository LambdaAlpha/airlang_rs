use super::ConstCellFn;
use super::ConstStaticFn;
use super::FreeCellFn;
use super::FreeStaticFn;
use super::Func;
use super::MutCellFn;
use super::MutStaticFn;
use super::Setup;
use super::comp::DynComposite;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MutCellCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) setup: Option<Setup>,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for MutCellCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.free.call(&mut self.ctx.clone(), input)
    }
}

impl FreeCellFn<Val, Val> for MutCellCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.comp.free.call(&mut self.ctx, input)
    }
}

impl ConstStaticFn<Val, Val, Val> for MutCellCompFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), ctx.into_dyn(), input)
    }
}

impl ConstCellFn<Val, Val, Val> for MutCellCompFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(&mut self.ctx, ctx.into_dyn(), input)
    }
}

impl MutStaticFn<Val, Val, Val> for MutCellCompFunc {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), DynRef::new_mut(ctx), input)
    }
}

impl MutCellFn<Val, Val, Val> for MutCellCompFunc {
    fn mut_cell_call(&mut self, ctx: &mut Val, input: Val) -> Val {
        self.comp.call(&mut self.ctx, DynRef::new_mut(ctx), input)
    }
}

impl Func for MutCellCompFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }
}
