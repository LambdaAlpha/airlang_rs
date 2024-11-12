use crate::{
    Ctx,
    CtxError,
    ctx::{
        DynRef,
        const1::ConstFnCtx,
        map::CtxMap,
        mut1::MutFnCtx,
    },
    val::func::CellFuncVal,
};

pub(crate) trait CtxRef<'a> {
    fn get_variables(self) -> Result<&'a CtxMap, CtxError>;

    fn get_variables_mut(self) -> Result<&'a mut CtxMap, CtxError>;

    fn get_variables_dyn(self) -> Result<DynRef<'a, CtxMap>, CtxError>;

    fn get_solver(self) -> Result<&'a CellFuncVal, CtxError>;

    #[allow(unused)]
    fn get_solver_mut(self) -> Result<&'a mut CellFuncVal, CtxError>;

    #[allow(unused)]
    fn get_solver_dyn(self) -> Result<DynRef<'a, CellFuncVal>, CtxError>;

    fn set_solver(self, solver: Option<CellFuncVal>) -> Result<(), CtxError>;
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
