#![feature(iterator_try_collect)]

pub use func::{
    ExtCtxConstFn,
    ExtCtxFreeFn,
    ExtCtxMutableFn,
    ExtFn,
    ExtFunc,
    ExtFuncVal,
};
use {
    crate::prelude::{
        Prelude,
        PRELUDE,
    },
    airlang::MutableCtx,
};

pub fn init_ctx(ctx: MutableCtx) {
    PRELUDE.with(|prelude| prelude.put(ctx));
}

pub(crate) mod func;

pub(crate) mod prelude;

#[cfg(test)]
mod test;
