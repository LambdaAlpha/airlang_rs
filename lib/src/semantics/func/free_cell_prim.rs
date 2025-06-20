use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::Func;
use crate::semantics::func::Setup;
use crate::semantics::val::Val;
use crate::trait_::dyn_safe::dyn_any_debug_clone_eq_hash;
use crate::type_::Symbol;

pub trait FreeCellFn<I, O>: FreeStaticFn<I, O> {
    fn free_cell_call(&mut self, input: I) -> O {
        self.free_static_call(input)
    }
}

dyn_any_debug_clone_eq_hash!(pub FreeCellFnVal : FreeCellFn<Val, Val>);

impl<I, O, T> FreeCellFn<I, O> for &mut T
where T: FreeCellFn<I, O>
{
    fn free_cell_call(&mut self, input: I) -> O {
        (**self).free_cell_call(input)
    }
}

#[derive(Clone)]
pub struct FreeCellPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Box<dyn FreeCellFnVal>,
    pub(crate) setup: Option<Setup>,
}

impl FreeStaticFn<Val, Val> for FreeCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for FreeCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn_.free_cell_call(input)
    }
}

impl Func for FreeCellPrimFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        false
    }
}

impl Debug for FreeCellPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialEq for FreeCellPrimFunc {
    fn eq(&self, other: &FreeCellPrimFunc) -> bool {
        self.id == other.id
    }
}

impl Eq for FreeCellPrimFunc {}

impl Hash for FreeCellPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
