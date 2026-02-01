use std::rc::Rc;

use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;

pub trait CtxFn<Cfg, Ctx, I, O> {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O;
}

impl<Cfg, Ctx, I, O, T> CtxFn<Cfg, Ctx, I, O> for &T
where T: CtxFn<Cfg, Ctx, I, O>
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        (**self).ctx_call(cfg, ctx, input)
    }
}

#[derive(Clone)]
pub struct CtxPrimFunc {
    pub(crate) raw_input: bool,
    pub(crate) fn_: Rc<dyn CtxFn<Cfg, Val, Val, Val>>,
    pub(crate) const_: bool,
}

impl CtxFn<Cfg, Val, Val, Val> for CtxPrimFunc {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.fn_.ctx_call(cfg, ctx, input)
    }
}

impl PartialEq for CtxPrimFunc {
    fn eq(&self, other: &CtxPrimFunc) -> bool {
        self.raw_input == other.raw_input && Rc::ptr_eq(&self.fn_, &other.fn_)
    }
}

impl Eq for CtxPrimFunc {}
