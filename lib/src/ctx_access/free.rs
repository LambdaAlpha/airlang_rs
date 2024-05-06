use crate::{
    ctx::{
        Ctx,
        CtxError,
        CtxTrait,
        CtxValue,
        DynRef,
    },
    ctx_access::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
        CtxAccessor,
    },
    symbol::Symbol,
    val::Val,
};

pub struct FreeCtx;

impl CtxTrait for FreeCtx {
    fn get_ref(&self, _name: &Symbol) -> Result<&Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_mut(&mut self, _name: &Symbol) -> Result<&mut Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_dyn(&mut self, _name: &Symbol) -> Result<DynRef<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn remove(&mut self, _name: &Symbol) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_value(&mut self, _name: Symbol, _value: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(&mut self, _name: &Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(&self, _name: &Symbol) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_const(&mut self, _name: &Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(&self, _name: &Symbol) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_mut(&mut self) -> Result<&mut Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta_dyn(&mut self) -> Result<DynRef<Ctx>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_meta(&mut self, _meta: Option<Ctx>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl CtxAccessor for FreeCtx {
    fn is_ctx_free(&self) -> bool {
        true
    }

    fn is_ctx_const(&self) -> bool {
        true
    }

    fn for_const_fn(&mut self) -> CtxForConstFn {
        CtxForConstFn::Free(FreeCtx)
    }

    fn for_mutable_fn(&mut self) -> CtxForMutableFn {
        CtxForMutableFn::Free(FreeCtx)
    }
}
