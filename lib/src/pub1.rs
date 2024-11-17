pub use crate::{
    adapt::Adapt,
    answer::Answer,
    ask::Ask,
    bool::Bool,
    byte::Byte,
    cache::Cache,
    call::Call,
    case::Case,
    ctx::{
        Ctx,
        CtxError,
        Invariant,
        const1::{
            ConstCtx,
            ConstFnCtx,
        },
        free::FreeCtx,
        mut1::{
            MutCtx,
            MutFnCtx,
        },
    },
    extension::ValExt,
    func::{
        cell::{
            CellFn,
            CellFnExt,
            CellFunc,
        },
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
        Mode,
        adapt::AdaptMode,
        ask::AskMode,
        call::CallMode,
        composite::CompositeMode,
        list::ListMode,
        map::MapMode,
        pair::PairMode,
        primitive::PrimitiveMode,
        recursive::SelfMode,
        symbol::SymbolMode,
    },
    number::Number,
    pair::Pair,
    symbol::Symbol,
    syntax::generator::ReprError,
    text::Text,
    unit::Unit,
    val::{
        Val,
        adapt::AdaptVal,
        answer::AnswerVal,
        ask::AskVal,
        byte::ByteVal,
        call::CallVal,
        case::{
            CacheCaseVal,
            CaseVal,
            TrivialCaseVal,
        },
        ctx::CtxVal,
        func::{
            CellFuncVal,
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
    },
};
use crate::{
    prelude,
    syntax,
    syntax::ParseError,
    transformer::Transformer,
};

pub fn parse(src: &str) -> Result<Val, ParseError> {
    syntax::parser::parse(src)
}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    syntax::generator::generate_pretty(src)
}

#[derive(Debug, Clone)]
pub struct AirCell {
    mode: Mode,
    ctx: Ctx,
}

impl AirCell {
    pub fn new(mode: Mode, ctx: Ctx) -> Self {
        Self { mode, ctx }
    }

    pub fn initial_ctx() -> Ctx {
        prelude::initial_ctx()
    }

    pub fn interpret(&mut self, input: Val) -> Val {
        self.mode.transform(MutCtx::new(&mut self.ctx), input)
    }

    pub fn ctx_mut(&mut self) -> MutCtx {
        MutCtx::new(&mut self.ctx)
    }
}

impl Default for AirCell {
    fn default() -> Self {
        Self {
            mode: Mode::default(),
            ctx: Self::initial_ctx(),
        }
    }
}
