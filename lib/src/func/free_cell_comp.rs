use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::FreeComposite;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeCellCompFunc {
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for FreeCellCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), input)
    }
}

impl FreeCellFn<Val, Val> for FreeCellCompFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.comp.call(&mut self.ctx, input)
    }
}

impl FuncTrait for FreeCellCompFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn ctx_explicit(&self) -> bool {
        false
    }

    fn code(&self) -> Val {
        self.comp.code()
    }
}
