use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;

use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeCellFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::Func;
use crate::semantics::func::Setup;
use crate::semantics::val::Val;
use crate::trait_::dyn_safe::dyn_any_debug_clone_eq_hash;
use crate::type_::Symbol;
use crate::type_::ref_::ConstRef;

pub trait ConstCellFn<Ctx, I, O>: FreeCellFn<I, O> + ConstStaticFn<Ctx, I, O> {
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: I) -> O;

    fn opt_const_cell_call(&mut self, ctx: Option<ConstRef<Ctx>>, input: I) -> O {
        match ctx {
            Some(ctx) => self.const_cell_call(ctx, input),
            None => self.free_cell_call(input),
        }
    }
}

dyn_any_debug_clone_eq_hash!(pub ConstCellFnVal : ConstCellFn<Val, Val, Val>);

impl<Ctx, I, O, T> ConstCellFn<Ctx, I, O> for &mut T
where T: ConstCellFn<Ctx, I, O>
{
    fn const_cell_call(&mut self, ctx: ConstRef<Ctx>, input: I) -> O {
        (**self).const_cell_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct ConstCellPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Box<dyn ConstCellFnVal>,
    pub(crate) setup: Option<Setup>,
    pub(crate) ctx_explicit: bool,
}

impl FreeStaticFn<Val, Val> for ConstCellPrimFunc {
    fn free_static_call(&self, input: Val) -> Val {
        self.fn_.free_static_call(input)
    }
}

impl FreeCellFn<Val, Val> for ConstCellPrimFunc {
    fn free_cell_call(&mut self, input: Val) -> Val {
        self.fn_.free_cell_call(input)
    }
}

impl ConstStaticFn<Val, Val, Val> for ConstCellPrimFunc {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_static_call(ctx, input)
    }
}

impl ConstCellFn<Val, Val, Val> for ConstCellPrimFunc {
    fn const_cell_call(&mut self, ctx: ConstRef<Val>, input: Val) -> Val {
        self.fn_.const_cell_call(ctx, input)
    }
}

impl Func for ConstCellPrimFunc {
    fn setup(&self) -> Option<&Setup> {
        self.setup.as_ref()
    }

    fn ctx_explicit(&self) -> bool {
        self.ctx_explicit
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
