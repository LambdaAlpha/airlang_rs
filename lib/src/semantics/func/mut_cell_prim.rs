use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::semantics::func::ConstCellFn;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeCellFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncMode;
use crate::semantics::func::FuncTrait;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::Val;
use crate::trait_::dyn_safe::dyn_any_clone_eq_hash;
use crate::type_::ConstRef;
use crate::type_::Either;
use crate::type_::Symbol;
use crate::type_::ref_::DynRef;

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

dyn_any_clone_eq_hash!(pub MutCellFnExt : MutCellFn<Val, Val, Val>);

#[derive(Clone)]
pub struct MutCellPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Box<dyn MutCellFnExt>,
    pub(crate) mode: FuncMode,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for MutCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for MutCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn_.free_cell_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for MutCellPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_static_call(ctx, input)
    }
}

impl ConstCellFn<Val, Val, Val> for MutCellPrimFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_cell_call(ctx, input)
    }
}

impl MutStaticFn<Val, Val, Val> for MutCellPrimFunc {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        self.fn_.mut_static_call(ctx, input)
    }
}

impl MutCellFn<Val, Val, Val> for MutCellPrimFunc {
    fn mut_cell_call(&mut self, ctx: &mut Val, input: Val) -> Val {
        self.fn_.mut_cell_call(ctx, input)
    }
}

impl FuncTrait for MutCellPrimFunc {
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

impl MutCellPrimFunc {
    pub fn new(id: Symbol, fn_: Box<dyn MutCellFnExt>, mode: FuncMode, ctx_explicit: bool) -> Self {
        Self { id, fn_, mode, ctx_explicit }
    }
}

impl Debug for MutCellPrimFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.id.fmt(f)
    }
}

impl PartialEq for MutCellPrimFunc {
    fn eq(&self, other: &MutCellPrimFunc) -> bool {
        self.id == other.id
    }
}

impl Eq for MutCellPrimFunc {}

impl Hash for MutCellPrimFunc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
