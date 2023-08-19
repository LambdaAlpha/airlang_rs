use crate::semantics::{
    ctx::CtxTrait,
    ctx_access::{
        constant::CtxForConstFn,
        mutable::CtxForMutableFn,
    },
};

pub(crate) trait CtxAccessor: CtxTrait {
    fn for_const_fn(&mut self) -> CtxForConstFn;

    fn for_mutable_fn(&mut self) -> CtxForMutableFn;
}

pub(crate) mod free;

pub(crate) mod constant;

pub(crate) mod mutable;
