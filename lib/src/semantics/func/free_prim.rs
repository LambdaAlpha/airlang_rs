use std::rc::Rc;

use super::prim::impl_prim_func;
use super::setup::Setup;
use super::setup::impl_setup;
use crate::semantics::val::Val;
use crate::type_::Symbol;

pub trait FreeFn<I, O> {
    fn free_call(&self, input: I) -> O;
}

impl<I, O, T> FreeFn<I, O> for &T
where T: FreeFn<I, O>
{
    fn free_call(&self, input: I) -> O {
        (**self).free_call(input)
    }
}

impl<I, O, T> FreeFn<I, O> for &mut T
where T: FreeFn<I, O>
{
    fn free_call(&self, input: I) -> O {
        (**self).free_call(input)
    }
}

#[derive(Clone)]
pub struct FreePrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Rc<dyn FreeFn<Val, Val>>,
    pub(crate) setup: Setup,
}

impl FreeFn<Val, Val> for FreePrimFunc {
    fn free_call(&self, input: Val) -> Val {
        self.fn_.free_call(input)
    }
}

impl Default for FreePrimFunc {
    fn default() -> Self {
        struct F;
        impl FreeFn<Val, Val> for F {
            fn free_call(&self, _input: Val) -> Val {
                Val::default()
            }
        }
        Self { id: Symbol::default(), fn_: Rc::new(F), setup: Setup::none() }
    }
}

impl_setup!(FreePrimFunc);

impl_prim_func!(FreePrimFunc);
