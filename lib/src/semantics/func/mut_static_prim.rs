use std::rc::Rc;

use super::ConstStaticFn;
use super::FreeStaticFn;
use super::prim::impl_prim_func;
use super::setup::Setup;
use super::setup::impl_setup;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Either;
use crate::type_::Symbol;
use crate::type_::ref_::DynRef;

pub trait MutStaticFn<Ctx, I, O>: ConstStaticFn<Ctx, I, O> {
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        self.const_static_call(ConstRef::new(ctx), input)
    }

    fn dyn_static_call(&self, ctx: DynRef<Ctx>, input: I) -> O {
        match ctx.into_either() {
            Either::This(ctx) => self.const_static_call(ctx, input),
            Either::That(ctx) => self.mut_static_call(ctx, input),
        }
    }

    fn opt_dyn_static_call(&self, ctx: Option<DynRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.dyn_static_call(ctx, input),
            None => self.free_static_call(input),
        }
    }
}

impl<Ctx, I, O, T> MutStaticFn<Ctx, I, O> for &T
where T: MutStaticFn<Ctx, I, O>
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_static_call(ctx, input)
    }
}

impl<Ctx, I, O, T> MutStaticFn<Ctx, I, O> for &mut T
where T: MutStaticFn<Ctx, I, O>
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_static_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct MutStaticPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn MutStaticFn<Val, Val, Val>>,
    pub(crate) setup: Setup,
}

impl FreeStaticFn<Val, Val> for MutStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for MutStaticPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Val, Val> for MutStaticPrimFunc {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.fn_.mut_static_call(ctx, input)
    }
}

impl_setup!(MutStaticPrimFunc);

impl_prim_func!(MutStaticPrimFunc);
