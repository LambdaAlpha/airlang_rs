use super::CallMapMode;
use super::ModeFn;
use super::SymbolMode;
use crate::semantics::core::CallEval;
use crate::semantics::core::CallForm;
use crate::semantics::core::ListUniForm;
use crate::semantics::core::MapUniForm;
use crate::semantics::core::PairForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<DataMode>,
    pub call: Option<CodeMode>,
    pub list: Option<DataMode>,
    pub map: Option<DataMode>,
}

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

impl ConstStaticFn<Val, Val, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
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

impl MutStaticFn<Val, Val, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
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

impl ConstStaticFn<Val, Symbol, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        match self.symbol {
            None => Val::Symbol(input),
            Some(mode) => mode.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, Symbol, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
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

impl ConstStaticFn<Val, PairVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        match self.pair {
            None => Val::Pair(input),
            Some(_) => PairForm { first: self, second: self }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, PairVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
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
                CodeMode::Form => {
                    CallForm { func: self, ctx: self, input: self, some: &CallMapMode::default() }
                        .free_static_call(input)
                }
                CodeMode::Eval => {
                    CallEval { func: self, ctx: self, input: self }.free_static_call(input)
                }
            },
        }
    }
}

impl ConstStaticFn<Val, CallVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: CallVal) -> Val {
        match self.call {
            None => Val::Call(input),
            Some(mode) => match mode {
                CodeMode::Form => {
                    CallForm { func: self, ctx: self, input: self, some: &CallMapMode::default() }
                        .const_static_call(ctx, input)
                }
                CodeMode::Eval => {
                    CallEval { func: self, ctx: self, input: self }.const_static_call(ctx, input)
                }
            },
        }
    }
}

impl MutStaticFn<Val, CallVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: CallVal) -> Val {
        match self.call {
            None => Val::Call(input),
            Some(mode) => match mode {
                CodeMode::Form => {
                    CallForm { func: self, ctx: self, input: self, some: &CallMapMode::default() }
                        .mut_static_call(ctx, input)
                }
                CodeMode::Eval => {
                    CallEval { func: self, ctx: self, input: self }.mut_static_call(ctx, input)
                }
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

impl ConstStaticFn<Val, ListVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        match self.list {
            None => Val::List(input),
            Some(_) => ListUniForm { item: self }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, ListVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
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

impl ConstStaticFn<Val, MapVal, Val> for PrimMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        match self.map {
            None => Val::Map(input),
            Some(_) => MapUniForm { key: self, value: self }.const_static_call(ctx, input),
        }
    }
}

impl MutStaticFn<Val, MapVal, Val> for PrimMode {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
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
