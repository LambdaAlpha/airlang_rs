use super::ConstFn;
use super::FreeComposite;
use super::FreeFn;
use super::comp::DynComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone, PartialEq, Eq)]
pub struct ConstCompFunc {
    pub(crate) raw_input: bool,
    pub(crate) comp: DynComposite,
}

impl FreeFn<Cfg, Val, Val> for ConstCompFunc {
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

impl ConstFn<Cfg, Val, Val, Val> for ConstCompFunc {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        self.comp.call(cfg, ctx.into_dyn(), input)
    }
}
