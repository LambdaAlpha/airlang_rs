use std::io::stdout;

use airlang::MutableCtx;

use crate::{
    prelude::{
        Prelude,
        PRELUDE,
    },
    repl::Repl,
};

fn main() -> std::io::Result<()> {
    let mut repl = Repl::new(stdout());
    repl.run()
}

pub(crate) fn init_ctx(mut ctx: MutableCtx) {
    airlang_ext::init_ctx(ctx.reborrow());
    PRELUDE.with(|prelude| prelude.put(ctx));
}

mod repl;

mod prelude;
