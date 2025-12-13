use super::FreeFn;
use super::comp::FreeComposite;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::Key;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeCompFunc {
    pub(crate) id: Key,
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
