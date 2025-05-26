use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;

pub trait FreeStaticFn<I, O> {
    fn free_static_call(&self, input: I) -> O;
}

pub struct FreeStaticImpl<I, O> {
    pub free: fn(I) -> O,
}

#[derive(Clone)]
pub struct FreeStaticPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Rc<dyn FreeStaticFn<Val, Val>>,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for FreeStaticPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl FuncTrait for FreeStaticPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl FreeStaticPrimFunc {
    pub fn new_extension(id: Symbol, fn1: Rc<dyn FreeStaticFn<Val, Val>>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Rc<dyn FreeStaticFn<Val, Val>>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
    }
}

impl Debug for FreeStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for FreeStaticPrimFunc {
    fn eq(&self, other: &FreeStaticPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for FreeStaticPrimFunc {}

impl Hash for FreeStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}

impl<I, O> FreeStaticFn<I, O> for FreeStaticImpl<I, O> {
    fn free_static_call(&self, input: I) -> O {
        (self.free)(input)
    }
}

impl<I, O> FreeStaticImpl<I, O> {
    pub fn new(free: fn(I) -> O) -> Self {
        Self { free }
    }

    pub fn default(_input: I) -> O
    where O: Default {
        O::default()
    }
}
