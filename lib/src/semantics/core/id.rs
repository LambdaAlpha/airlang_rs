use crate::semantics::cfg::Cfg;
use crate::semantics::func::CtxFn;
use crate::semantics::func::FreeFn;
use crate::semantics::val::Val;

#[derive(Default, Copy, Clone)]
pub(crate) struct Id;

impl FreeFn<Cfg, Val, Val> for Id {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        input
    }
}

impl CtxFn<Cfg, Val, Val, Val> for Id {
    fn ctx_call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        input
    }
}
