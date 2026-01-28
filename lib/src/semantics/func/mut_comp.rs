use super::ConstFn;
use super::FreeComposite;
use super::FreeFn;
use super::MutFn;
use super::comp::DynComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;

#[derive(Clone, PartialEq, Eq)]
pub struct MutCompFunc {
    pub(crate) raw_input: bool,
    pub(crate) comp: DynComposite,
}

impl FreeFn<Cfg, Val, Val> for MutCompFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        FreeComposite::call(
            cfg,
            input,
            &mut self.comp.prelude.clone(),
            self.comp.input_name.clone(),
            self.comp.body.clone(),
        )
    }
}

impl ConstFn<Cfg, Val, Val, Val> for MutCompFunc {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(cfg, ctx.into_dyn(), input)
    }
}

impl MutFn<Cfg, Val, Val, Val> for MutCompFunc {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.comp.call(cfg, DynRef::new_mut(ctx), input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
        self.comp.call(cfg, ctx, input)
    }
}
