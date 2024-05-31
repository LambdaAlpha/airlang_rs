use crate::{
    ctx::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
        ref1::{
            CtxMeta,
            CtxRef,
        },
        CtxValue,
        DynRef,
    },
    Ctx,
    CtxError,
    Symbol,
    Val,
};

pub struct FreeCtx;

impl<'a> CtxRef<'a> for FreeCtx {
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

    fn get_meta(self) -> Result<&'a Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_mut(self) -> Result<&'a mut Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_dyn(self) -> Result<DynRef<'a, Ctx>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_meta(self, _meta: Option<Ctx>) -> Result<(), CtxError> {
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

    fn for_const_fn(self) -> CtxForConstFn<'a> {
        CtxForConstFn::Free(FreeCtx)
    }

    fn for_mutable_fn(self) -> CtxForMutableFn<'a> {
        CtxForMutableFn::Free(FreeCtx)
    }
}
