use super::FreeFn;
use super::comp::FreeComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

#[derive(Clone, PartialEq, Eq)]
pub struct FreeCompFunc {
    pub(crate) raw_input: bool,
    pub(crate) comp: FreeComposite,
}

impl FreeFn<Cfg, Val, Val> for FreeCompFunc {
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
