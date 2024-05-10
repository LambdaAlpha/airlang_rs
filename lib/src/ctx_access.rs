use crate::{
    ctx::CtxRef,
    ctx_access::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
    },
    Ctx,
};

pub(crate) trait CtxAccessor<'a>: CtxRef<'a> {
    type Reborrow<'b>: CtxAccessor<'b>
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

pub(crate) mod free;

pub(crate) mod constant;

pub(crate) mod mutable;
