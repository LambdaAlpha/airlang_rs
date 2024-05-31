use crate::{
    ctx::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
        CtxValue,
        DynRef,
    },
    Ctx,
    CtxError,
    Symbol,
    Val,
};

pub(crate) trait CtxRef<'a> {
    fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError>;

    fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError>;

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'a, Val>, CtxError>;

    fn remove(self, name: Symbol) -> Result<Val, CtxError>;

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError>;

    fn set_final(self, name: Symbol) -> Result<(), CtxError>;

    #[allow(clippy::wrong_self_convention)]
    fn is_final(self, name: Symbol) -> Result<bool, CtxError>;

    fn set_const(self, name: Symbol) -> Result<(), CtxError>;

    #[allow(clippy::wrong_self_convention)]
    fn is_const(self, name: Symbol) -> Result<bool, CtxError>;

    fn get_meta(self) -> Result<&'a Ctx, CtxError>;

    #[allow(unused)]
    fn get_meta_mut(self) -> Result<&'a mut Ctx, CtxError>;

    fn get_meta_dyn(self) -> Result<DynRef<'a, Ctx>, CtxError>;

    fn set_meta(self, meta: Option<Ctx>) -> Result<(), CtxError>;
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

    fn for_const_fn(self) -> CtxForConstFn<'a>;

    fn for_mutable_fn(self) -> CtxForMutableFn<'a>;
}
