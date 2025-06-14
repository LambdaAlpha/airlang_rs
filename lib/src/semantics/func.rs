pub use self::const_cell_comp::ConstCellCompFunc;
pub use self::const_cell_prim::ConstCellFn;
pub use self::const_cell_prim::ConstCellFnExt;
pub use self::const_cell_prim::ConstCellPrimFunc;
pub use self::const_static_comp::ConstStaticCompFunc;
pub use self::const_static_prim::ConstStaticFn;
pub use self::const_static_prim::ConstStaticImpl;
pub use self::const_static_prim::ConstStaticPrimFunc;
pub use self::free_cell_comp::FreeCellCompFunc;
pub use self::free_cell_prim::FreeCellFn;
pub use self::free_cell_prim::FreeCellFnExt;
pub use self::free_cell_prim::FreeCellPrimFunc;
pub use self::free_static_comp::FreeStaticCompFunc;
pub use self::free_static_prim::FreeStaticFn;
pub use self::free_static_prim::FreeStaticImpl;
pub use self::free_static_prim::FreeStaticPrimFunc;
pub use self::func_mode::FuncMode;
pub use self::mode::ModeFunc;
pub use self::mut_cell_comp::MutCellCompFunc;
pub use self::mut_cell_prim::MutCellFn;
pub use self::mut_cell_prim::MutCellFnExt;
pub use self::mut_cell_prim::MutCellPrimFunc;
pub use self::mut_static_comp::MutStaticCompFunc;
pub use self::mut_static_prim::MutStaticFn;
pub use self::mut_static_prim::MutStaticImpl;
pub use self::mut_static_prim::MutStaticPrimFunc;

_____!();

pub(crate) use self::comp::DynComposite;
pub(crate) use self::comp::FreeComposite;
pub(crate) use self::comp::composite_call;
pub(crate) use self::func_mode::DEFAULT_MODE;

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
