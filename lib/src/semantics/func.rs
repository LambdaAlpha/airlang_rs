pub use self::comp::CompFunc;
pub use self::prim::CtxConstInputEvalFunc;
pub use self::prim::CtxConstInputFreeFunc;
pub use self::prim::CtxConstInputRawFunc;
pub use self::prim::CtxFreeInputEvalFunc;
pub use self::prim::CtxFreeInputFreeFunc;
pub use self::prim::CtxFreeInputRawFunc;
pub use self::prim::CtxMutInputEvalFunc;
pub use self::prim::CtxMutInputFreeFunc;
pub use self::prim::CtxMutInputRawFunc;
pub use self::prim::PrimCtx;
pub use self::prim::PrimFunc;
pub use self::prim::PrimInput;

_____!();

pub(crate) use self::comp::CompCtx;
pub(crate) use self::comp::CompInput;

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
