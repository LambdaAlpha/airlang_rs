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
#![feature(trait_alias)]

use crate::ui::StdUi;

fn main() -> std::io::Result<()> {
    let mut std_ui = StdUi::new();
    repl::repl(&mut std_ui)
}

mod ctx;

mod eval;

mod repl;

mod prelude;

mod ui;
