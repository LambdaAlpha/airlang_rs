use crate::{
    ctx::{
        const1::ConstFnCtx,
        map::CtxMapRef,
        mut1::MutFnCtx,
        DynRef,
    },
    Ctx,
    CtxError,
    FuncVal,
};

pub(crate) trait CtxRef<'a>: CtxMapRef<'a> {
    fn get_solver(self) -> Result<&'a FuncVal, CtxError>;

    #[allow(unused)]
    fn get_solver_mut(self) -> Result<&'a mut FuncVal, CtxError>;

    #[allow(unused)]
    fn get_solver_dyn(self) -> Result<DynRef<'a, FuncVal>, CtxError>;

    fn set_solver(self, solver: Option<FuncVal>) -> Result<(), CtxError>;
}

pub(crate) trait CtxMeta<'a>: CtxRef<'a> {
    type Reborrow<'b>: CtxMeta<'b>
    where
        Self: 'b;

    fn reborrow(&mut self) -> Self::Reborrow<'_>;

    fn borrow(&self) -> Option<&Ctx>;

    #[allow(clippy::wrong_self_convention)]
    #[allow(unused)]
    fn is_ctx_free(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
    #[allow(unused)]
    fn is_ctx_const(self) -> bool;

    fn for_const_fn(self) -> ConstFnCtx<'a>;

    fn for_mut_fn(self) -> MutFnCtx<'a>;
}
