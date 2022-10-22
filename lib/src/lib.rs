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

use rug as num;

use grammar::repr::{Bytes, Repr};

pub fn interpret(src: &str) -> String {
    // todo impl
    return src.to_string();
}

pub fn parse(src: &str) -> Repr {
    let result = grammar::parse(src);
    match result {
        Ok(val) => val,
        Err(_) => Repr::from(vec![] as Bytes),
    }
}

pub fn eval(src: Repr) -> Repr {
    // todo impl
    return src;
}

pub mod grammar;
#[allow(dead_code)]
mod utils;
