pub use ctx::{
    constant::{
        ConstCtx,
        CtxForConstFn,
    },
    free::FreeCtx,
    mutable::{
        CtxForMutableFn,
        MutableCtx,
    },
};

pub use self::{
    ask::Ask,
    bool::Bool,
    bytes::Bytes,
    call::Call,
    ctx::{
        Ctx,
        CtxError,
        Invariant,
    },
    extension::ValExt,
    func::{
        CtxConstFn,
        CtxFreeFn,
        CtxMutableFn,
        Func,
    },
    int::Int,
    list::List,
    logic::Assert,
    map::Map,
    mode::{
        ListItemMode,
        ListMode,
        MapMode,
        Mode,
        PairMode,
        ValMode,
    },
    number::Number,
    pair::Pair,
    problem::{
        Answer,
        Verified,
    },
    string::Str,
    symbol::Symbol,
    syntax::generator::ReprError,
    transform::Transform,
    unit::Unit,
    val::{
        answer::AnswerVal,
        ask::AskVal,
        assert::AssertVal,
        bytes::BytesVal,
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
        int::IntVal,
        list::ListVal,
        map::MapVal,
        number::NumberVal,
        pair::PairVal,
        string::StrVal,
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

pub fn generate(src: &Val) -> Result<String, ReprError> {
    syntax::generator::generate_pretty(src)
}

pub fn initial_ctx() -> Ctx {
    prelude::initial_ctx()
}

pub fn interpret_mutable(ctx: MutableCtx, input: Val) -> Val {
    Eval.transform(ctx, input)
}

pub fn interpret_const(ctx: ConstCtx, input: Val) -> Val {
    Eval.transform(ctx, input)
}

pub fn interpret_free(ctx: FreeCtx, input: Val) -> Val {
    Eval.transform(ctx, input)
}

pub(crate) mod val;

pub(crate) mod transformer;

pub(crate) mod transform;

pub(crate) mod mode;

pub(crate) mod ctx;

pub(crate) mod func;

pub(crate) mod logic;

pub(crate) mod arbitrary;

pub(crate) mod problem;

pub(crate) mod extension;

pub(crate) mod prelude;

pub mod syntax;

pub(crate) mod unit;

pub(crate) mod bool;

pub(crate) mod int;

pub(crate) mod number;

pub(crate) mod bytes;

pub(crate) mod symbol;

pub(crate) mod string;

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod call;

pub(crate) mod ask;

pub(crate) mod annotate;

pub(crate) mod types;

pub(crate) mod traits;

#[allow(unused)]
pub(crate) mod utils;

#[cfg(test)]
mod test;
