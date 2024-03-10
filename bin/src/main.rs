#![deny(
    bad_style,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_interfaces,
    private_bounds,
    unconditional_recursion,
    while_true
)]
#![cfg_attr(
    not(debug_assertions),
    deny(
        dead_code,
        unused,
        unused_allocation,
        unused_comparisons,
        unused_parens,
        clippy::needless_return,
        clippy::semicolon_if_nothing_returned,
    )
)]

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
