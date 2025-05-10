use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::rc::Rc;

use crate::ConstCtx;
use crate::ConstFnCtx;
use crate::FreeCtx;
use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::ctx::ref1::CtxMeta;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::transformer::Transformer;

pub trait ConstStaticFn {
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val;
}

#[derive(Clone)]
pub struct ConstStaticPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Rc<dyn ConstStaticFn>,
    pub(crate) mode: FuncMode,
}

impl Transformer<Val, Val> for ConstStaticPrimFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.call(ctx.for_const_fn(), input)
    }
}

impl FuncTrait for ConstStaticPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl ConstStaticPrimFunc {
    pub fn new_extension(id: Symbol, fn1: Rc<dyn ConstStaticFn>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Rc<dyn ConstStaticFn>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
    }
}

impl Debug for ConstStaticPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.prim.fmt(f)
    }
}

impl PartialEq for ConstStaticPrimFunc {
    fn eq(&self, other: &ConstStaticPrimFunc) -> bool {
        self.prim == other.prim
    }
}

impl Eq for ConstStaticPrimFunc {}

impl Hash for ConstStaticPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.prim.hash(state);
    }
}

pub(crate) struct ConstDispatcher<Free, Const> {
    free_fn: Free,
    const_fn: Const,
}

impl<Free, Const> ConstDispatcher<Free, Const>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
{
    #[expect(dead_code)]
    pub(crate) fn new(free_fn: Free, const_fn: Const) -> Self {
        Self { free_fn, const_fn }
    }
}

impl<Free, Const> ConstStaticFn for ConstDispatcher<Free, Const>
where
    Free: Fn(FreeCtx, Val) -> Val + 'static,
    Const: Fn(ConstCtx, Val) -> Val + 'static,
{
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val {
        match ctx {
            ConstFnCtx::Free(ctx) => (self.free_fn)(ctx, input),
            ConstFnCtx::Const(ctx) => (self.const_fn)(ctx, input),
        }
    }
}

impl<T> ConstStaticFn for T
where T: Fn(ConstFnCtx, Val) -> Val
{
    fn call(&self, ctx: ConstFnCtx, input: Val) -> Val {
        self(ctx, input)
    }
}
