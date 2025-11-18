use std::rc::Rc;

use super::ConstFn;
use super::FreeFn;
use super::prim::impl_prim_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Either;
use crate::type_::Symbol;
use crate::type_::ref_::DynRef;

pub trait MutFn<Cfg, Ctx, I, O>: ConstFn<Cfg, Ctx, I, O> {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        self.const_call(cfg, ConstRef::new(ctx), input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Ctx>, input: I) -> O {
        match ctx.into_either() {
            Either::This(ctx) => self.const_call(cfg, ctx, input),
            Either::That(ctx) => self.mut_call(cfg, ctx, input),
        }
    }

    fn opt_dyn_call(&self, cfg: &mut Cfg, ctx: Option<DynRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.dyn_call(cfg, ctx, input),
            None => self.free_call(cfg, input),
        }
    }
}

impl<Cfg, Ctx, I, O, T> MutFn<Cfg, Ctx, I, O> for &T
where T: MutFn<Cfg, Ctx, I, O>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_call(cfg, ctx, input)
    }
}

impl<Cfg, Ctx, I, O, T> MutFn<Cfg, Ctx, I, O> for &mut T
where T: MutFn<Cfg, Ctx, I, O>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_call(cfg, ctx, input)
    }
}

#[derive(Clone)]
pub struct MutPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) raw_input: bool,
    pub(crate) fn_: Rc<dyn MutFn<Cfg, Val, Val, Val>>,
}

impl FreeFn<Cfg, Val, Val> for MutPrimFunc {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        self.fn_.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Val, Val> for MutPrimFunc {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Val, Val> for MutPrimFunc {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        self.fn_.mut_call(cfg, ctx, input)
    }
}

impl_prim_func!(MutPrimFunc);
