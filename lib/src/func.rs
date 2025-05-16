use crate::Ctx;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::func::func_mode::FuncMode;
use crate::val::Val;

pub(crate) trait FuncTrait: MutStaticFn<Ctx, Val, Val> + MutCellFn<Ctx, Val, Val> {
    fn mode(&self) -> &FuncMode;

    fn code(&self) -> Val;
}

pub(crate) mod func_mode;

pub(crate) mod prim;

pub(crate) mod comp;

pub(crate) mod mode;

pub(crate) mod free_static_prim;

pub(crate) mod free_static_comp;

pub(crate) mod free_cell_prim;

pub(crate) mod free_cell_comp;

pub(crate) mod const_static_prim;

pub(crate) mod const_static_comp;

pub(crate) mod const_cell_prim;

pub(crate) mod const_cell_comp;

pub(crate) mod mut_static_prim;

pub(crate) mod mut_static_comp;

pub(crate) mod mut_cell_prim;

pub(crate) mod mut_cell_comp;

pub(crate) mod repr;
