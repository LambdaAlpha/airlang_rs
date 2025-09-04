use super::CallMode;
use super::CallPrimMode;
use super::ListMode;
use super::MapMode;
use super::PairMode;
use super::PrimMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Symbol;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompMode {
    pub default: PrimMode,
    pub pair: Option<Box<PairMode>>,
    pub call: Option<Box<CallMode>>,
    pub list: Option<Box<ListMode>>,
    pub map: Option<Box<MapMode>>,
}

impl FreeFn<Cfg, Val, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_call(cfg, symbol),
            Val::Pair(pair) => self.free_call(cfg, pair),
            Val::Call(call) => self.free_call(cfg, call),
            Val::List(list) => self.free_call(cfg, list),
            Val::Map(map) => self.free_call(cfg, map),
            v => v,
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.const_call(cfg, ctx, pair),
            Val::Call(call) => self.const_call(cfg, ctx, call),
            Val::List(list) => self.const_call(cfg, ctx, list),
            Val::Map(map) => self.const_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.mut_call(cfg, ctx, pair),
            Val::Call(call) => self.mut_call(cfg, ctx, call),
            Val::List(list) => self.mut_call(cfg, ctx, list),
            Val::Map(map) => self.mut_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl FreeFn<Cfg, Symbol, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        self.default.symbol.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        self.default.symbol.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        self.default.symbol.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, PairVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.free_call(cfg, input);
        };
        pair.form().free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.const_call(cfg, ctx, input);
        };
        pair.form().const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: PairVal) -> Val {
        let Some(pair) = &self.pair else {
            return self.default.mut_call(cfg, ctx, input);
        };
        pair.form().mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, CallVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: CallVal) -> Val {
        let Some(call) = &self.call else {
            return self.default.free_call(cfg, input);
        };
        match self.default.call {
            CallPrimMode::Form => call.form().free_call(cfg, input),
            CallPrimMode::Eval => call.eval().free_call(cfg, input),
        }
    }
}

impl ConstFn<Cfg, Val, CallVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: CallVal) -> Val {
        let Some(call) = &self.call else {
            return self.default.const_call(cfg, ctx, input);
        };
        match self.default.call {
            CallPrimMode::Form => call.form().const_call(cfg, ctx, input),
            CallPrimMode::Eval => call.eval().const_call(cfg, ctx, input),
        }
    }
}

impl MutFn<Cfg, Val, CallVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: CallVal) -> Val {
        let Some(call) = &self.call else {
            return self.default.mut_call(cfg, ctx, input);
        };
        match self.default.call {
            CallPrimMode::Form => call.form().mut_call(cfg, ctx, input),
            CallPrimMode::Eval => call.eval().mut_call(cfg, ctx, input),
        }
    }
}

impl FreeFn<Cfg, ListVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.free_call(cfg, input);
        };
        list.form().free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.const_call(cfg, ctx, input);
        };
        list.form().const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: ListVal) -> Val {
        let Some(list) = &self.list else {
            return self.default.mut_call(cfg, ctx, input);
        };
        list.form().mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, MapVal, Val> for CompMode {
    fn free_call(&self, cfg: &mut Cfg, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.free_call(cfg, input);
        };
        map.form().free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for CompMode {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.const_call(cfg, ctx, input);
        };
        map.form().const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for CompMode {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: MapVal) -> Val {
        let Some(map) = &self.map else {
            return self.default.mut_call(cfg, ctx, input);
        };
        map.form().mut_call(cfg, ctx, input)
    }
}

impl CompMode {
    pub const fn id() -> Self {
        Self { default: PrimMode::id(), pair: None, call: None, list: None, map: None }
    }

    pub const fn is_id(&self) -> bool {
        self.default.is_id()
            && self.pair.is_none()
            && self.call.is_none()
            && self.list.is_none()
            && self.map.is_none()
    }
}

impl From<PrimMode> for CompMode {
    fn from(default: PrimMode) -> Self {
        Self { default, pair: None, call: None, list: None, map: None }
    }
}
