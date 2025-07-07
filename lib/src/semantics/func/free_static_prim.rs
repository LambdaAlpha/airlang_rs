use std::rc::Rc;

use super::Setup;
use crate::semantics::func::Func;
use crate::semantics::func::prim::impl_prim_func;
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
    pub(crate) setup: Option<Setup>,
}

impl FreeStaticFn<Val, Val> for FreeStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl Func for FreeStaticPrimFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        false
    }
}

impl_prim_func!(FreeStaticPrimFunc);
