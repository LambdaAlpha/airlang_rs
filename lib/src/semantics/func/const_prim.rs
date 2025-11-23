use std::rc::Rc;

use super::FreeFn;
use super::prim::impl_prim_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::ref_::ConstRef;

pub trait ConstFn<Cfg, Ctx, I, O>: FreeFn<Cfg, I, O> {
    #[allow(unused_variables)]
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        self.free_call(cfg, input)
    }

    fn opt_const_call(&self, cfg: &mut Cfg, ctx: Option<ConstRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.const_call(cfg, ctx, input),
            None => self.free_call(cfg, input),
        }
    }
}

impl<Cfg, Ctx, I, O, T> ConstFn<Cfg, Ctx, I, O> for &T
where T: ConstFn<Cfg, Ctx, I, O>
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_call(cfg, ctx, input)
    }
}

impl<Cfg, Ctx, I, O, T> ConstFn<Cfg, Ctx, I, O> for &mut T
where T: ConstFn<Cfg, Ctx, I, O>
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_call(cfg, ctx, input)
    }
}

#[derive(Clone)]
pub struct ConstPrimFunc {
    pub(crate) id: Key,
    pub(crate) raw_input: bool,
    pub(crate) fn_: Rc<dyn ConstFn<Cfg, Val, Val, Val>>,
}

impl FreeFn<Cfg, Val, Val> for ConstPrimFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.fn_.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Val, Val> for ConstPrimFunc {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_call(cfg, ctx, input)
    }
}

impl_prim_func!(ConstPrimFunc);
