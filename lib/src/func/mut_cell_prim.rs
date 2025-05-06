use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::FuncMode;
use crate::MutFnCtx;
use crate::Symbol;
use crate::Val;
use crate::ctx::ref1::CtxMeta;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::traits::dyn_safe::dyn_any_clone_eq_hash;
use crate::transformer::Transformer;

pub trait MutCellFn {
    fn call(&mut self, ctx: MutFnCtx, input: Val) -> Val;
}

dyn_any_clone_eq_hash!(pub MutCellFnExt : MutCellFn);

#[derive(Clone)]
pub struct MutCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn MutCellFnExt>,
    pub(crate) mode: FuncMode,
}

impl Transformer<Val, Val> for MutCellPrimFunc {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.dyn_clone().call(ctx.for_mut_fn(), input)
    }
}

impl FuncTrait for MutCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn call(&self) -> Val {
        Val::default()
    }

    fn transform_mut<'a, Ctx>(&mut self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        self.fn1.call(ctx.for_mut_fn(), input)
    }
}

impl MutCellPrimFunc {
    pub fn new_extension(id: Symbol, fn1: Box<dyn MutCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Box<dyn MutCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
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
