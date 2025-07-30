use super::ConstStaticFn;
use super::FreeCellFn;
use super::FreeStaticFn;
use super::prim::impl_prim_func;
use super::setup::Setup;
use super::setup::impl_setup;
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
    pub(crate) setup: Setup,
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

impl_setup!(ConstCellPrimFunc);

impl_prim_func!(ConstCellPrimFunc);
