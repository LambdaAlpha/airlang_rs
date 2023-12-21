#![feature(trait_alias)]

use {
    crate::prelude::{
        Prelude,
        PRELUDE,
    },
    airlang::MutableCtx,
};
pub use {
    func::{
        ExtCtxConstFn,
        ExtCtxFreeFn,
        ExtCtxMutableFn,
        ExtFn,
        ExtFunc,
    },
    val::ExtVal,
};

pub fn init_ctx(ctx: MutableCtx) {
    PRELUDE.with(|prelude| prelude.put(ctx));
}

pub(crate) mod val;

pub(crate) mod func;

pub(crate) mod prelude;

#[cfg(test)]
mod test;
