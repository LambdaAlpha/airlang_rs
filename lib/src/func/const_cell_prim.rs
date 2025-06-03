use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::ConstStaticFn;
use crate::FreeCellFn;
use crate::FreeStaticFn;
use crate::FuncMode;
use crate::Symbol;
use crate::Val;
use crate::func::FuncTrait;
use crate::traits::dyn_safe::dyn_any_clone_eq_hash;
use crate::types::ref1::ConstRef;

pub trait ConstCellFn<Ctx, I, O>: FreeCellFn<I, O> + ConstStaticFn<Ctx, I, O> {
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: I) -> O;

    fn opt_const_cell_call(&mut self, ctx: Option<ConstRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.const_cell_call(ctx, input),
            None => self.free_cell_call(input),
        }
    }
}

dyn_any_clone_eq_hash!(pub ConstCellFnExt : ConstCellFn<Val, Val, Val>);

#[derive(Clone)]
pub struct ConstCellPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) extension: bool,
    pub(crate) fn1: Box<dyn ConstCellFnExt>,
    pub(crate) mode: FuncMode,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for ConstCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn1.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for ConstCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn1.free_cell_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstCellPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn1.const_static_call(ctx, input)
    }
}

impl ConstCellFn<Val, Val, Val> for ConstCellPrimFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn1.const_cell_call(ctx, input)
    }
}

impl FuncTrait for ConstCellPrimFunc {
    fn mode(&self) -> &FuncMode {
        &self.mode
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
    }

    fn code(&self) -> Val {
        Val::default()
    }
}

impl ConstCellPrimFunc {
    pub fn new(
        id: Symbol, fn1: Box<dyn ConstCellFnExt>, mode: FuncMode, ctx_explicit: bool,
    ) -> Self {
        Self { id, extension: true, fn1, mode, ctx_explicit }
    }
}

impl Debug for ConstCellPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialEq for ConstCellPrimFunc {
    fn eq(&self, other: &ConstCellPrimFunc) -> bool {
        self.id == other.id
    }
}

impl Eq for ConstCellPrimFunc {}

impl Hash for ConstCellPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
