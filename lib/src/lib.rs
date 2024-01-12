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

use thiserror::Error;

pub use self::{
    bool::Bool,
    bytes::Bytes,
    call::Call,
    ctx::{
        Ctx,
        CtxError,
        InvariantTag,
    },
    ctx_access::{
        constant::{
            ConstCtx,
            CtxForConstFn,
        },
        free::FreeCtx,
        mutable::{
            CtxForMutableFn,
            MutableCtx,
        },
    },
    eval_mode::EvalMode,
    extension::ValExt,
    float::Float,
    func::{
        CtxConstFn,
        CtxFreeFn,
        CtxMutableFn,
        Func,
    },
    int::Int,
    io_mode::{
        CallMode,
        IoMode,
        ListItemMode,
        ListMode,
        MapMode,
        MatchMode,
        PairMode,
        ReverseMode,
    },
    list::List,
    logic::Prop,
    map::Map,
    pair::Pair,
    reverse::Reverse,
    string::Str,
    symbol::Symbol,
    unit::Unit,
    val::{
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
        list::ListVal,
        map::MapVal,
        pair::PairVal,
        prop::PropVal,
        reverse::ReverseVal,
        Val,
    },
};
use crate::{
    eval::Evaluator,
    eval_mode::more::More,
    syntax::ParseError,
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
    prelude::initial_ctx()
}

pub fn interpret_mutable(mut ctx: MutableCtx, input: Val) -> Val {
    More.eval(&mut ctx, input)
}

pub fn interpret_const(mut ctx: ConstCtx, input: Val) -> Val {
    More.eval(&mut ctx, input)
}

pub fn interpret_free(mut ctx: FreeCtx, input: Val) -> Val {
    More.eval(&mut ctx, input)
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

pub(crate) mod unit;

pub(crate) mod bool;

pub(crate) mod int;

pub(crate) mod float;

pub(crate) mod bytes;

pub(crate) mod symbol;

pub(crate) mod string;

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod call;

pub(crate) mod reverse;

pub(crate) mod types;

pub(crate) mod traits;

#[allow(dead_code)]
pub(crate) mod utils;

#[cfg(test)]
mod test;
