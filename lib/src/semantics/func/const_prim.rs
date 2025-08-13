use std::rc::Rc;

use super::FreeFn;
use super::prim::impl_prim_func;
use super::setup::Setup;
use super::setup::impl_setup;
use crate::semantics::val::Val;
use crate::type_::Symbol;
use crate::type_::ref_::ConstRef;

pub trait ConstFn<Ctx, I, O>: FreeFn<I, O> {
    #[allow(unused_variables)]
    fn const_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        self.free_call(input)
    }

    fn opt_const_call(&self, ctx: Option<ConstRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.const_call(ctx, input),
            None => self.free_call(input),
        }
    }
}

impl<Ctx, I, O, T> ConstFn<Ctx, I, O> for &T
where T: ConstFn<Ctx, I, O>
{
    fn const_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_call(ctx, input)
    }
}

impl<Ctx, I, O, T> ConstFn<Ctx, I, O> for &mut T
where T: ConstFn<Ctx, I, O>
{
    fn const_call(&self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct ConstPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn ConstFn<Val, Val, Val>>,
    pub(crate) setup: Setup,
}

impl FreeFn<Val, Val> for ConstPrimFunc {
    fn free_call(&self, input: Val) -> Val {
        self.fn_.free_call(input)
    }
}

impl ConstFn<Val, Val, Val> for ConstPrimFunc {
    fn const_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_call(ctx, input)
    }
}

impl_setup!(ConstPrimFunc);

impl_prim_func!(ConstPrimFunc);
