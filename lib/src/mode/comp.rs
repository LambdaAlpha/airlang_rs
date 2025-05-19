use crate::CallVal;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::ListMode;
use crate::ListVal;
use crate::MapMode;
use crate::MapVal;
use crate::MutStaticFn;
use crate::PairMode;
use crate::PairVal;
use crate::PrimMode;
use crate::Symbol;
use crate::Val;
use crate::mode::ModeFn;
use crate::mode::call::CallMode;
use crate::mode::symbol::SymbolMode;

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

impl ConstStaticFn<Ctx, Val, Val> for CompMode {
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

impl MutStaticFn<Ctx, Val, Val> for CompMode {
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

impl FreeStaticFn<Symbol, Val> for CompMode {
    fn free_static_call(&self, input: Symbol) -> Val {
        self.symbol.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, Symbol, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Symbol) -> Val {
        self.symbol.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, Symbol, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Symbol) -> Val {
        self.symbol.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<PairVal, Val> for CompMode {
    fn free_static_call(&self, input: PairVal) -> Val {
        self.pair.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, PairVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: PairVal) -> Val {
        self.pair.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, PairVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: PairVal) -> Val {
        self.pair.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<CallVal, Val> for CompMode {
    fn free_static_call(&self, input: CallVal) -> Val {
        self.call.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, CallVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: CallVal) -> Val {
        self.call.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, CallVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        self.call.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<ListVal, Val> for CompMode {
    fn free_static_call(&self, input: ListVal) -> Val {
        self.list.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, ListVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: ListVal) -> Val {
        self.list.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, ListVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: ListVal) -> Val {
        self.list.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<MapVal, Val> for CompMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        self.map.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, MapVal, Val> for CompMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: MapVal) -> Val {
        self.map.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, MapVal, Val> for CompMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: MapVal) -> Val {
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
