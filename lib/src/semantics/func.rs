pub use self::const_comp::ConstCompFunc;
pub use self::const_prim::ConstFn;
pub use self::const_prim::ConstPrimFunc;
pub use self::free_comp::FreeCompFunc;
pub use self::free_prim::FreeFn;
pub use self::free_prim::FreePrimFunc;
pub use self::mut_comp::MutCompFunc;
pub use self::mut_prim::MutFn;
pub use self::mut_prim::MutPrimFunc;

_____!();

pub(crate) use self::comp::DynComposite;
pub(crate) use self::comp::FreeComposite;
pub(crate) use self::comp::composite_call;
pub(crate) use self::setup::Setup;
pub(crate) use self::setup::SetupFn;

_____!();

use std::rc::Rc;

use super::val::FuncVal;
use crate::semantics::core::Eval;
use crate::type_::Symbol;

pub(crate) trait FuncSetup {
    fn call(&self) -> Option<&FuncVal>;
    fn solve(&self) -> Option<&FuncVal>;
}

pub fn default_setup() -> FuncVal {
    FuncVal::MutPrim(
        MutPrimFunc {
            id: Symbol::from_str_unchecked("setup.default"),
            fn_: Rc::new(Eval),
            setup: Setup::none(),
        }
        .into(),
    )
}

// todo design self free/const/mut
// todo rename cell/static

mod prim;

mod comp;

mod setup;

mod free_prim;

mod free_comp;

mod const_prim;

mod const_comp;

mod mut_prim;

mod mut_comp;
