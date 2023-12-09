use crate::{
    semantics::{
        ctx::{
            Ctx,
            CtxError,
            CtxTrait,
            TaggedRef,
            TaggedVal,
        },
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
            CtxAccessor,
        },
        val::Val,
    },
    types::Symbol,
};

pub struct FreeCtx;

impl CtxTrait for FreeCtx {
    fn get(&self, _name: &str) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn remove(&mut self, _name: &str) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_val(&mut self, _name: Symbol, _val: TaggedVal) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_val_local(&mut self, _name: Symbol, _val: TaggedVal) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(&mut self, _name: &str) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_const(&mut self, _name: &str) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(&self, _name: &str) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(&self, _name: &str) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_null(&self, _name: &str) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_local(&self, _name: &str) -> Result<bool, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_meta(&self) -> Result<&Ctx, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_tagged_meta(&mut self) -> Result<TaggedRef<Ctx>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_meta(&mut self, _meta: Option<Ctx>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_tagged_ref(&mut self, _name: &str) -> Result<TaggedRef<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_const_ref(&self, _name: &str) -> Result<&Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_many_const_ref<const N: usize>(&self, _names: [&str; N]) -> [Result<&Val, CtxError>; N]
    where
        Self: Sized,
    {
        [Err(CtxError::AccessDenied); N]
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
