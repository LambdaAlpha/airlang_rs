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
    iter_advance_by,
    try_trait_v2,
    iterator_try_collect,
    unsize,
    coerce_unsized,
    array_methods,
    assert_matches,
    let_chains,
    variant_count
)]

use {
    crate::{
        ctx_access::mutable::MutableCtx,
        eval::Evaluator,
        eval_mode::more::More,
        extension::{
            CallExtension,
            ReverseExtension,
            CALL_EXTENSION,
            REVERSE_EXTENSION,
        },
        syntax::ParseError,
    },
    thiserror::Error,
};

pub use self::val::{
    CallVal as Call,
    CtxVal as Ctx,
    FuncVal as Func,
    ListVal as List,
    MapVal as Map,
    PairVal as Pair,
    PropVal as Prop,
    ReverseVal as Reverse,
    Val,
};

pub fn parse(src: &str) -> Result<Val, ParseError> {
    syntax::parser::parse(src)
}

#[derive(Error, Debug)]
#[error("ReprError")]
pub struct ReprError {}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    syntax::generator::generate_pretty(src)
}

pub fn initial_ctx() -> Ctx {
    let ctx = prelude::initial_ctx();
    Ctx(Box::new(ctx))
}

pub fn interpret(ctx: &mut Ctx, input: Val) -> Val {
    More.eval(&mut MutableCtx(&mut ctx.0), input)
}

pub fn set_call_extension(f: CallExtension) {
    CALL_EXTENSION.set(f);
}

pub fn set_reverse_extension(f: ReverseExtension) {
    REVERSE_EXTENSION.set(f);
}

pub(crate) mod val;

pub(crate) mod eval;

pub(crate) mod ctx;

pub(crate) mod ctx_access;

pub(crate) mod func;

pub(crate) mod eval_mode;

pub(crate) mod io_mode;

pub(crate) mod logic;

pub(crate) mod nondeterministic;

pub(crate) mod problem;

pub(crate) mod extension;

pub(crate) mod prelude;

pub mod syntax;

pub mod types;

pub(crate) mod traits;

#[allow(dead_code)]
pub(crate) mod utils;

#[cfg(test)]
mod test;
