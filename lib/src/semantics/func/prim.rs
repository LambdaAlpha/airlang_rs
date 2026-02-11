use std::rc::Rc;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::DynFunc;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct PrimFunc {
    pub(crate) raw_input: bool,
    pub(crate) fn_: Rc<dyn DynFunc<Cfg, Val, Val, Val>>,
    pub(crate) ctx: PrimCtx,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub(crate) enum PrimCtx {
    Free,
    Const_,
    Mut,
}

impl DynFunc<Cfg, Val, Val, Val> for PrimFunc {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.fn_.call(cfg, ctx, input)
    }
}

impl PartialEq for PrimFunc {
    fn eq(&self, other: &PrimFunc) -> bool {
        self.raw_input == other.raw_input
            && Rc::ptr_eq(&self.fn_, &other.fn_)
            && self.ctx == other.ctx
    }
}

impl Eq for PrimFunc {}

impl Default for PrimFunc {
    fn default() -> Self {
        struct F;
        impl DynFunc<Cfg, Val, Val, Val> for F {
            fn call(&self, _cfg: &mut Cfg, _ctx: &mut Val, _input: Val) -> Val {
                Val::default()
            }
        }
        Self { fn_: Rc::new(F), raw_input: false, ctx: PrimCtx::Free }
    }
}
