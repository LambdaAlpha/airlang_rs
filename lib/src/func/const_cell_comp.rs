use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::DynComposite;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ConstCellCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) mode: FuncMode,
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

impl FuncTrait for ConstCellCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }

    fn code(&self) -> Val {
        self.comp.code()
    }
}
