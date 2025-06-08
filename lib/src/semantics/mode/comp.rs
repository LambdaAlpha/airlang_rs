use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::mode::CallMode;
use crate::semantics::mode::ListMode;
use crate::semantics::mode::MapMode;
use crate::semantics::mode::ModeFn;
use crate::semantics::mode::PairMode;
use crate::semantics::mode::PrimMode;
use crate::semantics::mode::SymbolMode;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<PairMode>,
    pub call: Option<CallMode>,
    pub list: Option<ListMode>,
    pub map: Option<MapMode>,
}

impl ModeFn for CompMode {}

impl FreeStaticFn<Val, Val> for CompMode {
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

impl ConstStaticFn<Val, Val, Val> for CompMode {
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

impl MutStaticFn<Val, Val, Val> for CompMode {
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

impl FreeStaticFn<Symbol, Val> for CompMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        self.symbol.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        self.symbol.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        self.symbol.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<PairVal, Val> for CompMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        self.pair.free_static_call(input)
    }
}

impl ConstStaticFn<Val, PairVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        self.pair.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, PairVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        self.pair.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<CallVal, Val> for CompMode {
    fn free_static_call(&self, input: CallVal) -> Val {
        self.call.free_static_call(input)
    }
}

impl ConstStaticFn<Val, CallVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: CallVal) -> Val {
        self.call.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, CallVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: CallVal) -> Val {
        self.call.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<ListVal, Val> for CompMode {
    fn free_static_call(&self, input: ListVal) -> Val {
        self.list.free_static_call(input)
    }
}

impl ConstStaticFn<Val, ListVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        self.list.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, ListVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        self.list.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<MapVal, Val> for CompMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        self.map.free_static_call(input)
    }
}

impl ConstStaticFn<Val, MapVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        self.map.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, MapVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        self.map.mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for CompMode {
    fn from(mode: PrimMode) -> Self {
        let symbol = mode.symbol;
        let pair = mode.pair.map(|_| mode.into());
        let call = mode.call.map(|_| mode.try_into().unwrap());
        let list = mode.list.map(|_| mode.into());
        let map = mode.map.map(|_| mode.into());
        Self { symbol, pair, call, list, map }
    }
}
