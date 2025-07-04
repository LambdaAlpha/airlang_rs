use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use super::Setup;
use crate::semantics::func::Func;
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

impl Debug for FreeStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialEq for FreeStaticPrimFunc {
    fn eq(&self, other: &FreeStaticPrimFunc) -> bool {
        self.id == other.id
    }
}

impl Eq for FreeStaticPrimFunc {}

impl Hash for FreeStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
