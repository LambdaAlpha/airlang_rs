use std::rc::Rc;

use super::ConstFn;
use super::FreeFn;
use super::prim::impl_prim_func;
use super::setup::Setup;
use super::setup::impl_setup;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Either;
use crate::type_::Symbol;
use crate::type_::ref_::DynRef;

pub trait MutFn<Ctx, I, O>: ConstFn<Ctx, I, O> {
    fn mut_call(&self, ctx: &mut Ctx, input: I) -> O {
        self.const_call(ConstRef::new(ctx), input)
    }

    fn dyn_call(&self, ctx: DynRef<Ctx>, input: I) -> O {
        match ctx.into_either() {
            Either::This(ctx) => self.const_call(ctx, input),
            Either::That(ctx) => self.mut_call(ctx, input),
        }
    }

    fn opt_dyn_call(&self, ctx: Option<DynRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.dyn_call(ctx, input),
            None => self.free_call(input),
        }
    }
}

impl<Ctx, I, O, T> MutFn<Ctx, I, O> for &T
where T: MutFn<Ctx, I, O>
{
    fn mut_call(&self, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_call(ctx, input)
    }
}

impl<Ctx, I, O, T> MutFn<Ctx, I, O> for &mut T
where T: MutFn<Ctx, I, O>
{
    fn mut_call(&self, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct MutPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn MutFn<Val, Val, Val>>,
    pub(crate) setup: Setup,
}

impl FreeFn<Val, Val> for MutPrimFunc {
    fn free_call(&self, input: Val) -> Val {
        self.fn_.free_call(input)
    }
}

impl ConstFn<Val, Val, Val> for MutPrimFunc {
    fn const_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_call(ctx, input)
    }
}

impl MutFn<Val, Val, Val> for MutPrimFunc {
    fn mut_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.fn_.mut_call(ctx, input)
    }
}

impl_setup!(MutPrimFunc);

impl_prim_func!(MutPrimFunc);
