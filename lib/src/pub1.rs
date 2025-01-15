pub use crate::{
    abstract1::Abstract,
    ask::Ask,
    bit::Bit,
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
        FuncMode,
        const_cell_comp::ConstCellCompFunc,
        const_cell_prim::{
            ConstCellFn,
            ConstCellFnExt,
            ConstCellPrimFunc,
        },
        const_static_comp::ConstStaticCompFunc,
        const_static_prim::{
            ConstStaticFn,
            ConstStaticPrimFunc,
        },
        free_cell_comp::FreeCellCompFunc,
        free_cell_prim::{
            FreeCellFn,
            FreeCellFnExt,
            FreeCellPrimFunc,
        },
        free_static_comp::FreeStaticCompFunc,
        free_static_prim::{
            FreeStaticFn,
            FreeStaticPrimFunc,
        },
        mode::ModeFunc,
        mut_cell_comp::MutCellCompFunc,
        mut_cell_prim::{
            MutCellFn,
            MutCellFnExt,
            MutCellPrimFunc,
        },
        mut_static_comp::MutStaticCompFunc,
        mut_static_prim::{
            MutStaticFn,
            MutStaticPrimFunc,
        },
    },
    int::Int,
    list::List,
    map::Map,
    mode::{
        Mode,
        abstract1::AbstractMode,
        ask::AskMode,
        call::CallMode,
        comp::CompMode,
        eval::{
            Eval,
            EvalMode,
        },
        form::{
            Form,
            FormMode,
        },
        id::Id,
        list::ListMode,
        map::MapMode,
        pair::PairMode,
        prim::PrimMode,
        symbol::{
            PrefixMode,
            SymbolMode,
        },
        united::UniMode,
    },
    number::Number,
    pair::Pair,
    symbol::Symbol,
    text::Text,
    unit::Unit,
    val::{
        Val,
        abstract1::AbstractVal,
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
            FuncVal,
            const_cell_comp::ConstCellCompFuncVal,
            const_cell_prim::ConstCellPrimFuncVal,
            const_static_comp::ConstStaticCompFuncVal,
            const_static_prim::ConstStaticPrimFuncVal,
            free_cell_comp::FreeCellCompFuncVal,
            free_cell_prim::FreeCellPrimFuncVal,
            free_static_comp::FreeStaticCompFuncVal,
            free_static_prim::FreeStaticPrimFuncVal,
            mode::ModeFuncVal,
            mut_cell_comp::MutCellCompFuncVal,
            mut_cell_prim::MutCellPrimFuncVal,
            mut_static_comp::MutStaticCompFuncVal,
            mut_static_prim::MutStaticPrimFuncVal,
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
    syntax::{
        ParseError,
        ReprError,
        generator::PRETTY_FMT,
    },
    transformer::Transformer,
};

pub fn parse(src: &str) -> Result<Val, ParseError> {
    syntax::parser::parse(src)
}

pub fn generate(src: &Val) -> Result<String, ReprError> {
    let repr = src.try_into()?;
    let repr = syntax::generator::generate(repr, PRETTY_FMT);
    Ok(repr)
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
