use std::rc::Rc;

use super::prim::impl_prim_func;
use super::setup::FreeSetup;
use super::setup::impl_free_setup;
use crate::semantics::val::Val;
use crate::type_::Symbol;

pub trait FreeStaticFn<I, O> {
    fn free_static_call(&self, input: I) -> O;
}

impl<I, O, T> FreeStaticFn<I, O> for &T
where T: FreeStaticFn<I, O>
{
    fn free_static_call(&self, input: I) -> O {
        (**self).free_static_call(input)
    }
}

impl<I, O, T> FreeStaticFn<I, O> for &mut T
where T: FreeStaticFn<I, O>
{
    fn free_static_call(&self, input: I) -> O {
        (**self).free_static_call(input)
    }
}

#[derive(Clone)]
pub struct FreeStaticPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn FreeStaticFn<Val, Val>>,
    pub(crate) setup: FreeSetup,
}

impl FreeStaticFn<Val, Val> for FreeStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl_free_setup!(FreeStaticPrimFunc);

impl_prim_func!(FreeStaticPrimFunc);
