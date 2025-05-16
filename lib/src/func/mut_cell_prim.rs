use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutStaticFn;
use crate::Symbol;
use crate::Val;
use crate::either::Either;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::traits::dyn_safe::dyn_any_clone_eq_hash;
use crate::types::ref1::DynRef;

pub trait MutCellFn<Ctx, I, O>: ConstCellFn<Ctx, I, O> + MutStaticFn<Ctx, I, O> {
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: I) -> O;

    fn dyn_cell_call(&mut self, ctx: DynRef<Ctx>, input: I) -> O {
        match ctx.into_either() {
            Either::This(ctx) => self.const_cell_call(ctx, input),
            Either::That(ctx) => self.mut_cell_call(ctx, input),
        }
    }

    fn opt_dyn_cell_call(&mut self, ctx: Option<DynRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.dyn_cell_call(ctx, input),
            None => self.free_cell_call(input),
        }
    }
}

dyn_any_clone_eq_hash!(pub MutCellFnExt : MutCellFn<Ctx, Val, Val>);

#[derive(Clone)]
pub struct MutCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn MutCellFnExt>,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for MutCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for MutCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn1.free_cell_call(input)
    }
}

impl ConstStaticFn<Ctx, Val, Val> for MutCellPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        self.fn1.const_static_call(ctx, input)
    }
}

impl ConstCellFn<Ctx, Val, Val> for MutCellPrimFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        self.fn1.const_cell_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, Val, Val> for MutCellPrimFunc {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.fn1.mut_static_call(ctx, input)
    }
}

impl MutCellFn<Ctx, Val, Val> for MutCellPrimFunc {
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: Val) -> Val {
        self.fn1.mut_cell_call(ctx, input)
    }
}

impl FuncTrait for MutCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        Val::default()
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
