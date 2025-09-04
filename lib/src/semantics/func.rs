pub use self::const_comp::ConstCompFunc;
pub use self::const_prim::ConstFn;
pub use self::const_prim::ConstPrimFunc;
pub use self::free_comp::FreeCompFunc;
pub use self::free_prim::FreeFn;
pub use self::free_prim::FreePrimFunc;
pub use self::mut_comp::MutCompFunc;
pub use self::mut_prim::MutFn;
pub use self::mut_prim::MutPrimFunc;
pub use self::setup::default_setup;

_____!();

pub(crate) use self::comp::DynComposite;
pub(crate) use self::comp::FreeComposite;
pub(crate) use self::comp::composite_call;
pub(crate) use self::setup::Setup;
pub(crate) use self::setup::SetupFn;

_____!();

// todo design self free/const/mut

mod prim;

mod comp;

mod setup;

mod free_prim;

mod free_comp;

mod const_prim;

mod const_comp;

mod mut_prim;

mod mut_comp;
