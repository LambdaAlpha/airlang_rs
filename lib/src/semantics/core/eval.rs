use log::error;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::form::ListForm;
use crate::semantics::core::form::MapForm;
use crate::semantics::core::form::PairForm;
use crate::semantics::core::id::Id;
use crate::semantics::core::symbol::SymbolEval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::Symbol;

pub(crate) struct CallEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> FreeFn<Cfg, CallVal, Val> for CallEval<'a, Func>
where Func: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, call: CallVal) -> Val {
        cfg.step();
        let call = Call::from(call);
        let func = self.func.free_call(cfg, call.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        let input = if func.raw_input() { call.input } else { Eval.free_call(cfg, call.input) };
        func.free_call(cfg, input)
    }
}

impl<'a, Func> ConstFn<Cfg, Val, CallVal, Val> for CallEval<'a, Func>
where Func: ConstFn<Cfg, Val, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Val>, call: CallVal) -> Val {
        cfg.step();
        let call = Call::from(call);
        let func = self.func.const_call(cfg, ctx.reborrow(), call.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        let input = if func.raw_input() {
            call.input
        } else {
            Eval.const_call(cfg, ctx.reborrow(), call.input)
        };
        func.const_call(cfg, ctx, input)
    }
}

impl<'a, Func> MutFn<Cfg, Val, CallVal, Val> for CallEval<'a, Func>
where Func: MutFn<Cfg, Val, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        cfg.step();
        let call = Call::from(call);
        let func = self.func.mut_call(cfg, ctx, call.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        let input = if func.raw_input() { call.input } else { Eval.mut_call(cfg, ctx, call.input) };
        func.mut_call(cfg, ctx, input)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Eval;

impl FreeFn<Cfg, Val, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_call(cfg, symbol),
            Val::Pair(pair) => self.free_call(cfg, pair),
            Val::Call(call) => self.free_call(cfg, call),
            Val::List(list) => self.free_call(cfg, list),
            Val::Map(map) => self.free_call(cfg, map),
            v => Id.free_call(cfg, v),
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.const_call(cfg, ctx, pair),
            Val::Call(call) => self.const_call(cfg, ctx, call),
            Val::List(list) => self.const_call(cfg, ctx, list),
            Val::Map(map) => self.const_call(cfg, ctx, map),
            v => Id.const_call(cfg, ctx, v),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.mut_call(cfg, ctx, pair),
            Val::Call(call) => self.mut_call(cfg, ctx, call),
            Val::List(list) => self.mut_call(cfg, ctx, list),
            Val::Map(map) => self.mut_call(cfg, ctx, map),
            v => Id.mut_call(cfg, ctx, v),
        }
    }
}

impl FreeFn<Cfg, Symbol, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        SymbolEval.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolEval.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        SymbolEval.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, PairVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: PairVal) -> Val {
        PairForm { first: self, second: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: PairVal) -> Val {
        PairForm { first: self, second: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: PairVal) -> Val {
        PairForm { first: self, second: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, CallVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: CallVal) -> Val {
        CallEval { func: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, CallVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: CallVal) -> Val {
        CallEval { func: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, CallVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: CallVal) -> Val {
        CallEval { func: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, ListVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: ListVal) -> Val {
        ListForm { item: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: ListVal) -> Val {
        ListForm { item: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: ListVal) -> Val {
        ListForm { item: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, MapVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: MapVal) -> Val {
        MapForm { value: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: MapVal) -> Val {
        MapForm { value: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: MapVal) -> Val {
        MapForm { value: self }.mut_call(cfg, ctx, input)
    }
}
