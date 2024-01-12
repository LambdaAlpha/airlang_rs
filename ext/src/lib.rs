#![feature(iterator_try_collect, let_chains)]

use airlang::MutableCtx;

use crate::prelude::{
    Prelude,
    PRELUDE,
};

pub fn init_ctx(ctx: MutableCtx) {
    PRELUDE.with(|prelude| prelude.put(ctx));
}

pub(crate) mod prelude;

#[cfg(test)]
mod test;
