use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::DynRef;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::func::prim::Primitive;
use crate::traits::dyn_safe::dyn_any_clone_eq_hash;

pub trait FreeCellFn<I, O>: FreeStaticFn<I, O> {
    fn free_cell_call(&mut self, input: I) -> O {
        self.free_static_call(input)
    }
}

dyn_any_clone_eq_hash!(pub FreeCellFnExt : FreeCellFn<Val, Val>);

#[derive(Clone)]
pub struct FreeCellPrimFunc {
    pub(crate) prim: Primitive,
    pub(crate) fn1: Box<dyn FreeCellFnExt>,
    pub(crate) mode: FuncMode,
}

impl FreeStaticFn<Val, Val> for FreeCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for FreeCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn1.free_cell_call(input)
    }
}

impl ConstStaticFn<Ctx, Val, Val> for FreeCellPrimFunc {
    fn opt_const_static_call(&self, _ctx: Option<ConstRef<Ctx>>, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl ConstCellFn<Ctx, Val, Val> for FreeCellPrimFunc {
    fn const_cell_call(&mut self, _ctx: ConstRef<Ctx>, input: Val) -> Val {
        self.free_cell_call(input)
    }

    fn opt_const_cell_call(&mut self, _ctx: Option<ConstRef<Ctx>>, input: Val) -> Val {
        self.free_cell_call(input)
    }
}

impl MutStaticFn<Ctx, Val, Val> for FreeCellPrimFunc {
    fn dyn_static_call(&self, _ctx: DynRef<Ctx>, input: Val) -> Val {
        self.free_static_call(input)
    }

    fn opt_dyn_static_call(&self, _ctx: Option<DynRef<Ctx>>, input: Val) -> Val {
        self.free_static_call(input)
    }
}

impl MutCellFn<Ctx, Val, Val> for FreeCellPrimFunc {
    fn mut_cell_call(&mut self, _ctx: &mut Ctx, input: Val) -> Val {
        self.free_cell_call(input)
    }

    fn dyn_cell_call(&mut self, _ctx: DynRef<Ctx>, input: Val) -> Val {
        self.free_cell_call(input)
    }

    fn opt_dyn_cell_call(&mut self, _ctx: Option<DynRef<Ctx>>, input: Val) -> Val {
        self.free_cell_call(input)
    }
}

impl FuncTrait for FreeCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl FreeCellPrimFunc {
    pub fn new_extension(id: Symbol, fn1: Box<dyn FreeCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: true }, fn1, mode }
    }

    pub(crate) fn new(id: Symbol, fn1: Box<dyn FreeCellFnExt>, mode: FuncMode) -> Self {
        Self { prim: Primitive { id, is_extension: false }, fn1, mode }
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
