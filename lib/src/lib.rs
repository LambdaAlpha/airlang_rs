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
#![allow(incomplete_features)]
#![feature(
    iter_array_chunks,
    try_trait_v2,
    iterator_try_collect,
    unsize,
    coerce_unsized
)]

pub mod syntax;

pub mod semantics;

pub(crate) mod types;

pub(crate) mod traits;

#[allow(dead_code)]
pub(crate) mod utils;
