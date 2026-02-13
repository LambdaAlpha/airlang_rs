use crate::semantics::cfg::Cfg;
use crate::semantics::func::DynFunc;
use crate::semantics::val::Val;

#[derive(Default, Copy, Clone)]
pub struct Id;

impl DynFunc<Cfg, Val, Val, Val> for Id {
    fn call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        input
    }
}
