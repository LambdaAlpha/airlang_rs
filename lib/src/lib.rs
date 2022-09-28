#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]

use val::{Bytes, Val};

pub fn interpret(src: &str) -> String {
    // todo impl
    return src.to_string();
}

pub fn parse(src: &str) -> Val {
    let result = grammar::parse(src);
    match result {
        Ok(val) => val,
        Err(_) => Val::from(vec![] as Bytes),
    }
}

pub fn eval(src: Val) -> Val {
    // todo impl
    return src;
}

pub mod val;

mod grammar;
#[allow(dead_code)]
mod utils;
use rug as num;
