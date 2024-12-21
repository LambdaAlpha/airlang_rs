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
    ConstFnCtx,
    FuncMode,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    ext,
    func::{
        FuncTrait,
        prim::Primitive,
    },
    transformer::Transformer,
};

pub trait ConstCellFn {
    fn call(&mut self, ctx: ConstFnCtx, input: Val) -> Val;
}

ext!(pub ConstCellFnExt : ConstCellFn);

#[derive(Clone)]
pub struct ConstCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn ConstCellFnExt>,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for ConstCellPrimFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.dyn_clone().call(ctx.for_const_fn(), input)
    }
}

impl FuncTrait for ConstCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn cacheable(&self) -> bool {
        self.cacheable
    }

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.call(ctx.for_const_fn(), input)
    }
}

impl ConstCellPrimFunc {
    pub fn new_extension(
        id: Symbol,
        fn1: Box<dyn ConstCellFnExt>,
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
        fn1: Box<dyn ConstCellFnExt>,
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

impl Debug for ConstCellPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for ConstCellPrimFunc {
    fn eq(&self, other: &ConstCellPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for ConstCellPrimFunc {}

impl Hash for ConstCellPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}
