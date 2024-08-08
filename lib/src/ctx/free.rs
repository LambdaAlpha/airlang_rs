use crate::{
    ctx::{
        const1::ConstFnCtx,
        map::CtxMapRef,
        mut1::MutFnCtx,
        ref1::{
            CtxMeta,
            CtxRef,
        },
        CtxValue,
        DynRef,
    },
    Ctx,
    CtxError,
    FuncVal,
    Symbol,
    Val,
};

pub struct FreeCtx;

impl<'a> CtxMapRef<'a> for FreeCtx {
    fn get_ref(self, _name: Symbol) -> Result<&'a Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_mut(self, _name: Symbol) -> Result<&'a mut Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_dyn(self, _name: Symbol) -> Result<DynRef<'a, Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn remove(self, _name: Symbol) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_assignable(self, _name: Symbol) -> bool {
        false
    }

    fn put_value(self, _name: Symbol, _value: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(self, _name: Symbol) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_const(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(self, _name: Symbol) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl<'a> CtxRef<'a> for FreeCtx {
    fn get_solver(self) -> Result<&'a FuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_mut(self) -> Result<&'a mut FuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'a, FuncVal>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_solver(self, _solver: Option<FuncVal>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl<'a> CtxMeta<'a> for FreeCtx {
    type Reborrow<'s> = FreeCtx where Self: 's;

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
