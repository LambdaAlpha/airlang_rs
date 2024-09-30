use crate::{
    Ctx,
    CtxError,
    ctx::{
        DynRef,
        const1::ConstFnCtx,
        map::CtxMap,
        mut1::MutFnCtx,
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    val::func::FreeFuncVal,
};

pub struct FreeCtx;

impl<'a> CtxRef<'a> for FreeCtx {
    fn get_variables(self) -> Result<&'a CtxMap, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_variables_mut(self) -> Result<&'a mut CtxMap, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_variables_dyn(self) -> Result<DynRef<'a, CtxMap>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver(self) -> Result<&'a FreeFuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_mut(self) -> Result<&'a mut FreeFuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'a, FreeFuncVal>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_solver(self, _solver: Option<FreeFuncVal>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl<'a> CtxMeta<'a> for FreeCtx {
    type Reborrow<'s>
        = FreeCtx
    where
        Self: 's;

    fn reborrow(&mut self) -> Self::Reborrow<'_> {
        FreeCtx
    }

    fn borrow(&self) -> Option<&Ctx> {
        None
    }

    fn is_ctx_free(self) -> bool {
        true
    }

    fn is_ctx_const(self) -> bool {
        true
    }

    fn for_const_fn(self) -> ConstFnCtx<'a> {
        ConstFnCtx::Free(FreeCtx)
    }

    fn for_mut_fn(self) -> MutFnCtx<'a> {
        MutFnCtx::Free(FreeCtx)
    }
}
