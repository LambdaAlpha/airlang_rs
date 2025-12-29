use std::fmt::Debug;
use std::fmt::Formatter;

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
            &mut self.comp.ctx.clone(),
            self.comp.input_name.clone(),
            self.comp.body.clone(),
        )
    }
}

impl Debug for FreeCompFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FreeCompFunc")
            .field("raw_input", &self.raw_input)
            .field("ctx", &self.comp.ctx)
            .field("input_name", &self.comp.input_name)
            .field("body", &self.comp.body)
            .finish()
    }
}
