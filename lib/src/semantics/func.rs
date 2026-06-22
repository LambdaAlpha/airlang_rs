pub use self::comp::CompFunc;
pub use self::prim::ConstFunc;
pub use self::prim::ConstInputFreeFunc;
pub use self::prim::CtxFreeFunc;
pub use self::prim::DefaultFunc;
pub use self::prim::FreeFunc;
pub use self::prim::InputFreeFunc;
pub use self::prim::MutFunc;
pub use self::prim::MutInputFreeFunc;
pub use self::prim::PrimCtx;
pub use self::prim::PrimFunc;
pub use self::prim::PrimInput;

_____!();

pub(crate) use self::comp::CompCtx;
pub(crate) use self::comp::CompInput;

_____!();

use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;

pub trait DynFunc<C, I, O> {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<C>, input: I) -> O;
}

impl<C, I, O, T> DynFunc<C, I, O> for &T
where T: ?Sized + DynFunc<C, I, O>
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<C>, input: I) -> O {
        (**self).call(cfg, ctx, input)
    }
}

mod prim;

mod comp;
