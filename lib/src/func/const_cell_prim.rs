use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::ConstFnCtx;
use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::ctx::ref1::CtxMeta;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::traits::dyn_safe::dyn_any_clone_eq_hash;
use crate::transformer::Transformer;

pub trait ConstCellFn {
    fn call(&mut self, ctx: ConstFnCtx, input: Val) -> Val;
}

dyn_any_clone_eq_hash!(pub ConstCellFnExt : ConstCellFn);

#[derive(Clone)]
pub struct ConstCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn ConstCellFnExt>,
    pub(crate) mode: FuncMode,
}

impl Transformer<Val, Val> for ConstCellPrimFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.dyn_clone().call(ctx.for_const_fn(), input)
    }
}

impl FuncTrait for ConstCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        Val::default()
    }

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.call(ctx.for_const_fn(), input)
    }
}

impl ConstCellPrimFunc {
    pub fn new_extension(id: Symbol, fn1: Box<dyn ConstCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Box<dyn ConstCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
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
