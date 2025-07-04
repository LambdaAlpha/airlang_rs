pub use self::const_cell_comp::ConstCellCompFunc;
pub use self::const_cell_prim::ConstCellFn;
pub use self::const_cell_prim::ConstCellFnVal;
pub use self::const_cell_prim::ConstCellPrimFunc;
pub use self::const_static_comp::ConstStaticCompFunc;
pub use self::const_static_prim::ConstStaticFn;
pub use self::const_static_prim::ConstStaticPrimFunc;
pub use self::free_cell_comp::FreeCellCompFunc;
pub use self::free_cell_prim::FreeCellFn;
pub use self::free_cell_prim::FreeCellFnVal;
pub use self::free_cell_prim::FreeCellPrimFunc;
pub use self::free_static_comp::FreeStaticCompFunc;
pub use self::free_static_prim::FreeStaticFn;
pub use self::free_static_prim::FreeStaticPrimFunc;
pub use self::mut_cell_comp::MutCellCompFunc;
pub use self::mut_cell_prim::MutCellFn;
pub use self::mut_cell_prim::MutCellFnVal;
pub use self::mut_cell_prim::MutCellPrimFunc;
pub use self::mut_static_comp::MutStaticCompFunc;
pub use self::mut_static_prim::MutStaticFn;
pub use self::mut_static_prim::MutStaticPrimFunc;

_____!();

pub(crate) use self::comp::DynComposite;
pub(crate) use self::comp::FreeComposite;
pub(crate) use self::comp::composite_call;

_____!();

use std::rc::Rc;

use super::val::FuncVal;
use crate::semantics::core::Eval;
use crate::type_::Symbol;

pub(crate) trait Func {
    fn setup(&self) -> Option<&Setup>;

    fn ctx_explicit(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Setup {
    pub forward: FuncVal,
    pub reverse: FuncVal,
}

pub fn default_setup() -> FuncVal {
    FuncVal::MutStaticPrim(
        MutStaticPrimFunc {
            id: Symbol::from_str_unchecked("setup.default"),
            fn_: Rc::new(Eval),
            setup: None,
            ctx_explicit: false,
        }
        .into(),
    )
}

// todo rename cell/static

mod comp;

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
