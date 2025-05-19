use const_format::concatcp;

use crate::CallVal;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::ListVal;
use crate::MapVal;
use crate::MutStaticFn;
use crate::PairVal;
use crate::Symbol;
use crate::SymbolMode;
use crate::Val;
use crate::core::CallEval;
use crate::core::CallForm;
use crate::core::ListUniForm;
use crate::core::MapUniForm;
use crate::core::PairForm;
use crate::mode::ModeFn;
use crate::mode::symbol::LITERAL_CHAR;
use crate::mode::symbol::MOVE_CHAR;
use crate::mode::symbol::REF_CHAR;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<DataMode>,
    pub call: Option<CodeMode>,
    pub list: Option<DataMode>,
    pub map: Option<DataMode>,
}

pub(crate) const FORM: &str = "form";
pub(crate) const EVAL: &str = "eval";

pub(crate) const FORM_LITERAL: &str = concatcp!(FORM, LITERAL_CHAR);
pub(crate) const FORM_REF: &str = concatcp!(FORM, REF_CHAR);
pub(crate) const FORM_MOVE: &str = concatcp!(FORM, MOVE_CHAR);
pub(crate) const EVAL_LITERAL: &str = concatcp!(EVAL, LITERAL_CHAR);
pub(crate) const EVAL_REF: &str = concatcp!(EVAL, REF_CHAR);
pub(crate) const EVAL_MOVE: &str = concatcp!(EVAL, MOVE_CHAR);

impl ModeFn for PrimMode {}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DataMode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CodeMode {
    Form,
    Eval,
}

impl FreeStaticFn<Val, Val> for PrimMode {
    fn free_static_call(&self, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_static_call(symbol),
            Val::Pair(pair) => self.free_static_call(pair),
            Val::Call(call) => self.free_static_call(call),
            Val::List(list) => self.free_static_call(list),
            Val::Map(map) => self.free_static_call(map),
            v => v,
        }
    }
}

impl ConstStaticFn<Ctx, Val, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_static_call(ctx, symbol),
            Val::Pair(pair) => self.const_static_call(ctx, pair),
            Val::Call(call) => self.const_static_call(ctx, call),
            Val::List(list) => self.const_static_call(ctx, list),
            Val::Map(map) => self.const_static_call(ctx, map),
            v => v,
        }
    }
}

impl MutStaticFn<Ctx, Val, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_static_call(ctx, symbol),
            Val::Pair(pair) => self.mut_static_call(ctx, pair),
            Val::Call(call) => self.mut_static_call(ctx, call),
            Val::List(list) => self.mut_static_call(ctx, list),
            Val::Map(map) => self.mut_static_call(ctx, map),
            v => v,
        }
    }
}

impl FreeStaticFn<Symbol, Val> for PrimMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Ctx, Symbol, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Ctx, Symbol, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.mut_static_call(ctx, input),
        }
    }
}

impl FreeStaticFn<PairVal, Val> for PrimMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        match self.pair {
            None => Val::Pair(input),
            Some(_) => PairForm { first: self, second: self }.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Ctx, PairVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: PairVal) -> Val {
        match self.pair {
            None => Val::Pair(input),
            Some(_) => PairForm { first: self, second: self }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Ctx, PairVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: PairVal) -> Val {
        match self.pair {
            None => Val::Pair(input),
            Some(_) => PairForm { first: self, second: self }.mut_static_call(ctx, input),
        }
    }
}

impl FreeStaticFn<CallVal, Val> for PrimMode {
    fn free_static_call(&self, input: CallVal) -> Val {
        match self.call {
            None => Val::Call(input),
            Some(mode) => match mode {
                CodeMode::Form => CallForm { func: self, input: self }.free_static_call(input),
                CodeMode::Eval => CallEval { func: self, input: self }.free_static_call(input),
            },
        }
    }
}

impl ConstStaticFn<Ctx, CallVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: CallVal) -> Val {
        match self.call {
            None => Val::Call(input),
            Some(mode) => match mode {
                CodeMode::Form => {
                    CallForm { func: self, input: self }.const_static_call(ctx, input)
                }
                CodeMode::Eval => {
                    CallEval { func: self, input: self }.const_static_call(ctx, input)
                }
            },
        }
    }
}

impl MutStaticFn<Ctx, CallVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        match self.call {
            None => Val::Call(input),
            Some(mode) => match mode {
                CodeMode::Form => CallForm { func: self, input: self }.mut_static_call(ctx, input),
                CodeMode::Eval => CallEval { func: self, input: self }.mut_static_call(ctx, input),
            },
        }
    }
}

impl FreeStaticFn<ListVal, Val> for PrimMode {
    fn free_static_call(&self, input: ListVal) -> Val {
        match self.list {
            None => Val::List(input),
            Some(_) => ListUniForm { item: self }.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Ctx, ListVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: ListVal) -> Val {
        match self.list {
            None => Val::List(input),
            Some(_) => ListUniForm { item: self }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Ctx, ListVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: ListVal) -> Val {
        match self.list {
            None => Val::List(input),
            Some(_) => ListUniForm { item: self }.mut_static_call(ctx, input),
        }
    }
}

impl FreeStaticFn<MapVal, Val> for PrimMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        match self.map {
            None => Val::Map(input),
            Some(_) => MapUniForm { key: self, value: self }.free_static_call(input),
        }
    }
}

impl ConstStaticFn<Ctx, MapVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: MapVal) -> Val {
        match self.map {
            None => Val::Map(input),
            Some(_) => MapUniForm { key: self, value: self }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Ctx, MapVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: MapVal) -> Val {
        match self.map {
            None => Val::Map(input),
            Some(_) => MapUniForm { key: self, value: self }.mut_static_call(ctx, input),
        }
    }
}

impl PrimMode {
    pub(crate) const fn symbol_call(symbol: SymbolMode, call: CodeMode) -> PrimMode {
        PrimMode {
            symbol: Some(symbol),
            call: Some(call),
            pair: Some(DataMode),
            list: Some(DataMode),
            map: Some(DataMode),
        }
    }
}
