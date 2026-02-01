use super::CtxFn;
use super::FreeComposite;
use super::FreeFn;
use super::comp::DynComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

#[derive(Clone, PartialEq, Eq)]
pub struct CtxCompFunc {
    pub(crate) raw_input: bool,
    pub(crate) comp: DynComposite,
}

impl FreeFn<Cfg, Val, Val> for CtxCompFunc {
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

impl CtxFn<Cfg, Val, Val, Val> for CtxCompFunc {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.comp.call(cfg, ctx, self.comp.const_, input)
    }
}
