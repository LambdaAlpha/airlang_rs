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
    ConstCtx,
    FreeCtx,
    FuncMode,
    MutCtx,
    MutFnCtx,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
    func::{
        FuncTrait,
        prim::Primitive,
    },
    transformer::Transformer,
};

pub trait MutStaticFn {
    fn call(&self, ctx: MutFnCtx, input: Val) -> Val;
}

#[derive(Clone)]
pub struct MutStaticPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Rc<dyn MutStaticFn>,
    pub(crate) mode: FuncMode,
}

impl Transformer<Val, Val> for MutStaticPrimFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.call(ctx.for_mut_fn(), input)
    }
}

impl FuncTrait for MutStaticPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn call(&self) -> Val {
        Val::default()
    }
}

impl MutStaticPrimFunc {
    pub fn new_extension(id: Symbol, fn1: Rc<dyn MutStaticFn>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Rc<dyn MutStaticFn>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
    }
}

impl Debug for MutStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for MutStaticPrimFunc {
    fn eq(&self, other: &MutStaticPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for MutStaticPrimFunc {}

impl Hash for MutStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}

pub(crate) struct MutDispatcher<Free, Const, Mut> {
    free_fn: Free,
    const_fn: Const,
    mut_fn: Mut,
}

impl<Free, Const, Mut> MutDispatcher<Free, Const, Mut>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
    Mut: Fn(MutCtx, Val) -> Val + 'static,
{
    pub(crate) fn new(free_fn: Free, const_fn: Const, mut_fn: Mut) -> Self {
        Self { free_fn, const_fn, mut_fn }
    }
}

impl<Free, Const, Mut> MutStaticFn for MutDispatcher<Free, Const, Mut>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
    Mut: Fn(MutCtx, Val) -> Val + 'static,
{
    fn call(&self, ctx: MutFnCtx, input: Val) -> Val {
        match ctx {
            MutFnCtx::Free(ctx) => (self.free_fn)(ctx, input),
            MutFnCtx::Const(ctx) => (self.const_fn)(ctx, input),
            MutFnCtx::Mut(ctx) => (self.mut_fn)(ctx, input),
        }
    }
}

impl<T> MutStaticFn for T
where T: Fn(MutFnCtx, Val) -> Val
{
    fn call(&self, ctx: MutFnCtx, input: Val) -> Val {
        self(ctx, input)
    }
}
