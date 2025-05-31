use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::traits::dyn_safe::dyn_any_clone_eq_hash;

pub trait FreeCellFn<I, O>: FreeStaticFn<I, O> {
    fn free_cell_call(&mut self, input: I) -> O {
        self.free_static_call(input)
    }
}

dyn_any_clone_eq_hash!(pub FreeCellFnExt : FreeCellFn<Val, Val>);

#[derive(Clone)]
pub struct FreeCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn FreeCellFnExt>,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for FreeCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for FreeCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn1.free_cell_call(input)
    }
}

impl FuncTrait for FreeCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn ctx_explicit(&self) -> bool {
        false
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl FreeCellPrimFunc {
    pub fn new(id: Symbol, fn1: Box<dyn FreeCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }
}

impl Debug for FreeCellPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for FreeCellPrimFunc {
    fn eq(&self, other: &FreeCellPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for FreeCellPrimFunc {}

impl Hash for FreeCellPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}
