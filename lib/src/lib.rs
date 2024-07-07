pub use self::{
    answer::Answer,
    ask::Ask,
    bool::Bool,
    byte::Byte,
    cache::Cache,
    call::Call,
    case::Case,
    comment::Comment,
    ctx::{
        const1::{
            ConstCtx,
            ConstFnCtx,
        },
        free::FreeCtx,
        mut1::{
            MutCtx,
            MutFnCtx,
        },
        Ctx,
        CtxError,
        Invariant,
    },
    extension::ValExt,
    func::{
        const1::{
            ConstFn,
            ConstFunc,
        },
        free::{
            FreeFn,
            FreeFunc,
        },
        mut1::{
            MutFn,
            MutFunc,
        },
    },
    int::Int,
    list::List,
    map::Map,
    mode::{
        basic::BasicMode,
        list::{
            ListItemMode,
            ListMode,
        },
        map::MapMode,
        pair::PairMode,
        Mode,
        ValMode,
    },
    number::Number,
    pair::Pair,
    symbol::Symbol,
    syntax::generator::ReprError,
    text::Text,
    unit::Unit,
    val::{
        answer::AnswerVal,
        ask::AskVal,
        byte::ByteVal,
        call::CallVal,
        case::{
            CacheCaseVal,
            CaseVal,
            TrivialCaseVal,
        },
        comment::CommentVal,
        ctx::CtxVal,
        func::{
            ConstFuncVal,
            FreeFuncVal,
            FuncVal,
            MutFuncVal,
        },
        int::IntVal,
        list::ListVal,
        map::MapVal,
        number::NumberVal,
        pair::PairVal,
        text::TextVal,
        Val,
    },
};
use crate::{
    mode::eval::Eval,
    syntax::ParseError,
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

pub fn interpret_mut(ctx: MutCtx, input: Val) -> Val {
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

pub(crate) mod mode;

pub(crate) mod ctx;

pub(crate) mod func;

pub(crate) mod arbitrary;

pub(crate) mod problem;

pub(crate) mod extension;

pub(crate) mod prelude;

pub mod syntax;

pub(crate) mod unit;

pub(crate) mod bool;

pub(crate) mod int;

pub(crate) mod number;

pub(crate) mod byte;

pub(crate) mod symbol;

pub(crate) mod text;

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod call;

pub(crate) mod ask;

pub(crate) mod case;

pub(crate) mod cache;

pub(crate) mod answer;

pub(crate) mod comment;

pub(crate) mod types;

pub(crate) mod traits;

#[allow(unused)]
pub(crate) mod utils;

#[cfg(test)]
mod test;
