#![deny(
    bad_style,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
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
    )
)]
#![feature(trait_alias)]

use crate::repl::ui::StdUi;

mod repl;

fn main() {
    let mut std_ui = StdUi::new();
    repl::repl(&mut std_ui);
}
