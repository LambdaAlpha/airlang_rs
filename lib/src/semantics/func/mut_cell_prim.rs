use super::ConstCellFn;
use super::ConstStaticFn;
use super::FreeCellFn;
use super::FreeStaticFn;
use super::MutStaticFn;
use super::prim::impl_prim_func;
use super::setup::DynSetup;
use super::setup::impl_dyn_setup;
use crate::semantics::val::Val;
use crate::trait_::dyn_safe::dyn_any_debug_clone_eq_hash;
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

dyn_any_debug_clone_eq_hash!(pub MutCellFnVal : MutCellFn<Val, Val, Val>);

impl<Ctx, I, O, T> MutCellFn<Ctx, I, O> for &mut T
where T: MutCellFn<Ctx, I, O>
{
    fn mut_cell_call(&mut self, ctx: &mut Ctx, input: I) -> O {
        (**self).mut_cell_call(ctx, input)
    }
}

#[derive(Clone)]
pub struct MutCellPrimFunc {
    pub(crate) id: Symbol,
    pub(crate) fn_: Box<dyn MutCellFnVal>,
    pub(crate) setup: DynSetup,
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

impl_dyn_setup!(MutCellPrimFunc);

impl_prim_func!(MutCellPrimFunc);
