use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    rc::Rc,
};

use crate::{
    FuncMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        prim::Primitive,
    },
    transformer::Transformer,
};

pub trait FreeStaticFn {
    fn call(&self, input: Val) -> Val;
}

#[derive(Clone)]
pub struct FreeStaticPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Rc<dyn FreeStaticFn>,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for FreeStaticPrimFunc {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.call(input)
    }
}

impl FuncTrait for FreeStaticPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn call(&self) -> Val {
        Val::default()
    }
}

impl FreeStaticPrimFunc {
    pub fn new_extension(
        id: Symbol, fn1: Rc<dyn FreeStaticFn>, mode: FuncMode, cacheable: bool,
    ) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode, cacheable }
    }

    pub(crate) fn new(
        id: Symbol, fn1: Rc<dyn FreeStaticFn>, mode: FuncMode, cacheable: bool,
    ) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode, cacheable }
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

impl<T> FreeStaticFn for T
where T: Fn(Val) -> Val
{
    fn call(&self, input: Val) -> Val {
        self(input)
    }
}
