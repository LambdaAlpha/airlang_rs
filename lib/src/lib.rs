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
#![allow(incomplete_features)]
#![feature(
    trait_upcasting,
    trait_alias,
    never_type,
    allocator_api,
    const_type_id,
    arc_unwrap_or_clone,
    iter_array_chunks,
    slice_concat_trait
)]

pub use {
    grammar::ParseError,
    repr::Repr,
};

use crate::semantics::{
    eval,
    val::Val,
    ReprError,
};

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    src.parse::<Repr>()
}

pub fn stringify(src: &Repr) -> String {
    src.to_string()
}

pub fn interpret(src: &Repr) -> Result<Repr, ReprError> {
    let input = Val::from(src);
    let output = eval(input);
    output.try_into()
}

pub(crate) mod grammar;
pub(crate) mod repr;
pub(crate) mod semantics;
pub(crate) mod types;
#[allow(dead_code)]
pub(crate) mod utils;
