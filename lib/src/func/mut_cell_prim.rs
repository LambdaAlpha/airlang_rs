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
    MutFnCtx,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    extension::ext,
    func::{
        FuncTrait,
        prim::Primitive,
    },
    transformer::Transformer,
};

pub trait MutCellFn {
    fn call(&mut self, ctx: MutFnCtx, input: Val) -> Val;
}

ext!(pub MutCellFnExt : MutCellFn);

#[derive(Clone)]
pub struct MutCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn MutCellFnExt>,
    pub(crate) mode: FuncMode,
    pub(crate) cacheable: bool,
}

impl Transformer<Val, Val> for MutCellPrimFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.fn1.dyn_clone().call(ctx.for_mut_fn(), input)
    }
}

impl FuncTrait for MutCellPrimFunc {
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
        self.fn1.call(ctx.for_mut_fn(), input)
    }
}

impl MutCellPrimFunc {
    pub fn new_extension(
        id: Symbol,
        fn1: Box<dyn MutCellFnExt>,
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
        fn1: Box<dyn MutCellFnExt>,
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

impl Debug for MutCellPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for MutCellPrimFunc {
    fn eq(&self, other: &MutCellPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for MutCellPrimFunc {}

impl Hash for MutCellPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}
