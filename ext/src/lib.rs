use airlang::MutCtx;

use crate::prelude::{
    PRELUDE,
    Prelude,
};

pub fn init_ctx(ctx: MutCtx) {
    PRELUDE.with(|prelude| prelude.put(ctx));
}

pub(crate) mod prelude;

#[cfg(test)]
mod test;
