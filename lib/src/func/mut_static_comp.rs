use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::DynRef;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutStaticFn;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::comp::DynComposite;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MutStaticCompFunc {
    pub(crate) comp: DynComposite,
    pub(crate) ctx: Ctx,
    pub(crate) mode: FuncMode,
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

impl FuncTrait for MutStaticCompFunc {
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
