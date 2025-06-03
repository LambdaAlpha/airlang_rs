use crate::Ctx;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::FreeComposite;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FreeStaticCompFunc {
    pub(crate) comp: FreeComposite,
    pub(crate) ctx: Ctx,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for FreeStaticCompFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.comp.call(&mut self.ctx.clone(), input)
    }
}

impl FuncTrait for FreeStaticCompFunc {
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
