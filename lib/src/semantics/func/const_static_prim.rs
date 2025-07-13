use std::rc::Rc;

use super::FreeStaticFn;
use super::prim::impl_prim_func;
use super::setup::DynSetup;
use super::setup::impl_dyn_setup;
use crate::semantics::val::Val;
use crate::type_::Symbol;
use crate::type_::ref_::ConstRef;

pub trait ConstStaticFn<Ctx, I, O>: FreeStaticFn<I, O> {
    #[allow(unused_variables)]
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        self.free_static_call(input)
    }

    fn opt_const_static_call(&self, ctx: Option<ConstRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.const_static_call(ctx, input),
            None => self.free_static_call(input),
        }
    }
}

impl<Ctx, I, O, T> ConstStaticFn<Ctx, I, O> for &T
where T: ConstStaticFn<Ctx, I, O>
{
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_static_call(ctx, input)
    }
}

impl<Ctx, I, O, T> ConstStaticFn<Ctx, I, O> for &mut T
where T: ConstStaticFn<Ctx, I, O>
{
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_static_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct ConstStaticPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn ConstStaticFn<Val, Val, Val>>,
    pub(crate) setup: DynSetup,
}

impl FreeStaticFn<Val, Val> for ConstStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstStaticPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_static_call(ctx, input)
    }
}

impl_dyn_setup!(ConstStaticPrimFunc);

impl_prim_func!(ConstStaticPrimFunc);
