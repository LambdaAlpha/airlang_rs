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

use thiserror::Error;

pub use self::{
    bool::Bool,
    bytes::Bytes,
    call::Call,
    ctx::{
        Ctx,
        CtxError,
        Invariant,
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
    extension::ValExt,
    float::Float,
    func::{
        CtxConstFn,
        CtxFreeFn,
        CtxMutableFn,
        Func,
    },
    int::Int,
    list::List,
    logic::Prop,
    map::Map,
    mode::{
        CallForSomeMode,
        CallMode,
        ListItemMode,
        ListMode,
        MapMode,
        Mode,
        ReverseForSomeMode,
        ReverseMode,
        TransformMode,
        ValMode,
    },
    pair::Pair,
    problem::{
        Answer,
        Verified,
    },
    reverse::Reverse,
    string::Str,
    symbol::Symbol,
    transform::Transform,
    unit::Unit,
    val::{
        answer::AnswerVal,
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
    syntax::ParseError,
    transform::eval::Eval,
    transformer::Transformer,
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
    Eval.transform(&mut ctx, input)
}

pub fn interpret_const(mut ctx: ConstCtx, input: Val) -> Val {
    Eval.transform(&mut ctx, input)
}

pub fn interpret_free(mut ctx: FreeCtx, input: Val) -> Val {
    Eval.transform(&mut ctx, input)
}

pub(crate) mod val;

pub(crate) mod transformer;

pub(crate) mod transform;

pub(crate) mod mode;

pub(crate) mod ctx;

pub(crate) mod ctx_access;

pub(crate) mod func;

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

pub(crate) mod annotation;

pub(crate) mod types;

pub(crate) mod traits;

#[allow(unused)]
pub(crate) mod utils;

#[cfg(test)]
mod test;
