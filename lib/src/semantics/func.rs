pub use const_cell_comp::ConstCellCompFunc;
pub use const_cell_prim::ConstCellFn;
pub use const_cell_prim::ConstCellFnExt;
pub use const_cell_prim::ConstCellPrimFunc;
pub use const_static_comp::ConstStaticCompFunc;
pub use const_static_prim::ConstStaticFn;
pub use const_static_prim::ConstStaticImpl;
pub use const_static_prim::ConstStaticPrimFunc;
pub use free_cell_comp::FreeCellCompFunc;
pub use free_cell_prim::FreeCellFn;
pub use free_cell_prim::FreeCellFnExt;
pub use free_cell_prim::FreeCellPrimFunc;
pub use free_static_comp::FreeStaticCompFunc;
pub use free_static_prim::FreeStaticFn;
pub use free_static_prim::FreeStaticImpl;
pub use free_static_prim::FreeStaticPrimFunc;
pub use func_mode::FuncMode;
pub use mode::ModeFunc;
pub use mut_cell_comp::MutCellCompFunc;
pub use mut_cell_prim::MutCellFn;
pub use mut_cell_prim::MutCellFnExt;
pub use mut_cell_prim::MutCellPrimFunc;
pub use mut_static_comp::MutStaticCompFunc;
pub use mut_static_prim::MutStaticFn;
pub use mut_static_prim::MutStaticImpl;
pub use mut_static_prim::MutStaticPrimFunc;

_____!();

pub(crate) use comp::DynComposite;
pub(crate) use comp::FreeComposite;
pub(crate) use comp::composite_call;
pub(crate) use func_mode::DEFAULT_MODE;

_____!();

use super::val::Val;

pub(crate) trait FuncTrait {
    fn mode(&self) -> &FuncMode;

    fn ctx_explicit(&self) -> bool;

    fn code(&self) -> Val;
}

mod func_mode;

mod comp;

mod mode;

mod free_static_prim;

mod free_static_comp;

mod free_cell_prim;

mod free_cell_comp;

mod const_static_prim;

mod const_static_comp;

mod const_cell_prim;

mod const_cell_comp;

mod mut_static_prim;

mod mut_static_comp;

mod mut_cell_prim;

mod mut_cell_comp;
