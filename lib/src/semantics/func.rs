pub use self::comp::CompFunc;
pub use self::prim::PrimFunc;

_____!();

pub(crate) use self::comp::CompCtx;
pub(crate) use self::comp::CompFn;
pub(crate) use self::prim::PrimCtx;

_____!();

pub trait DynFunc<Cfg, Ctx, I, O> {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O;
}

impl<Cfg, Ctx, I, O, T> DynFunc<Cfg, Ctx, I, O> for &T
where T: DynFunc<Cfg, Ctx, I, O>
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, input: I) -> O {
        (**self).call(cfg, ctx, input)
    }
}

mod prim;

mod comp;
