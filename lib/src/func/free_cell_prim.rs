use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
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
    traits::dyn_safe::dyn_any_clone_eq_hash,
    transformer::Transformer,
};

pub trait FreeCellFn {
    fn call(&mut self, input: Val) -> Val;
}

dyn_any_clone_eq_hash!(pub FreeCellFnExt : FreeCellFn);

#[derive(Clone)]
pub struct FreeCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn FreeCellFnExt>,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for FreeCellPrimFunc {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.dyn_clone().call(input)
    }
}

impl FuncTrait for FreeCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn transform_mut<'a, Ctx>(&mut self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(input)
    }
}

impl FreeCellPrimFunc {
    pub fn new_extension(
        id: Symbol,
        fn1: Box<dyn FreeCellFnExt>,
        mode: FuncMode,
        cacheable: bool,
    ) -> Self {
        Self {
            prim: Primitive {
                id,
                is_extension: true,
            },
            fn1,
            mode,
            cacheable,
        }
    }

    pub(crate) fn new(
        id: Symbol,
        fn1: Box<dyn FreeCellFnExt>,
        mode: FuncMode,
        cacheable: bool,
    ) -> Self {
        Self {
            prim: Primitive {
                id,
                is_extension: false,
            },
            fn1,
            mode,
            cacheable,
        }
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
