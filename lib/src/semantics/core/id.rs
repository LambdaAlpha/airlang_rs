use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Id;

impl FreeFn<Cfg, Val, Val> for Id {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        cfg.step();
        input
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Id {
    fn const_call(&self, cfg: &mut Cfg, _ctx: ConstRef<Val>, input: Val) -> Val {
        cfg.step();
        input
    }
}

impl MutFn<Cfg, Val, Val, Val> for Id {
    fn mut_call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        cfg.step();
        input
    }
}
