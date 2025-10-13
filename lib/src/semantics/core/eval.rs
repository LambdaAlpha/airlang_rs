use log::error;

use super::ListForm;
use super::MapForm;
use super::PairForm;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::form::Form;
use crate::semantics::ctx::DynCtx;
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
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;

pub(crate) struct SymbolEval<'a, Fn> {
    pub(crate) default: char,
    pub(crate) f: &'a Fn,
}

pub(crate) const SYMBOL_LITERAL_CHAR: char = '.';
pub(crate) const SYMBOL_REF_CHAR: char = '@';
pub(crate) const SYMBOL_EVAL_CHAR: char = '$';

impl<'a, Fn> SymbolEval<'a, Fn> {
    fn recognize(&self, input: Symbol) -> (char, Symbol) {
        match input.chars().next() {
            Some(SYMBOL_LITERAL_CHAR) => {
                (SYMBOL_LITERAL_CHAR, Symbol::from_str_unchecked(&input[1 ..]))
            }
            Some(SYMBOL_REF_CHAR) => (SYMBOL_REF_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            Some(SYMBOL_EVAL_CHAR) => (SYMBOL_EVAL_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            _ => (self.default, input),
        }
    }
}

impl<'a, Fn> FreeFn<Cfg, Symbol, Val> for SymbolEval<'a, Fn> {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        cfg.step();
        let (prefix, s) = self.recognize(input.clone());
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                error!("symbol {input:?} should be evaluated in a ctx");
                Val::default()
            }
            SYMBOL_EVAL_CHAR => {
                error!("symbol {input:?} should be evaluated in a ctx");
                Val::default()
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> ConstFn<Cfg, Val, Symbol, Val> for SymbolEval<'a, Fn>
where Fn: ConstFn<Cfg, Val, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        cfg.step();
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                let Some(val) = ctx.unwrap().ref_(s) else {
                    return Val::default();
                };
                val.clone()
            }
            SYMBOL_EVAL_CHAR => {
                let ctx = ctx.unwrap();
                let Some(val) = ctx.ref_(s) else {
                    return Val::default();
                };
                let val = val.clone();
                self.f.const_call(cfg, ConstRef::new(ctx), val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> MutFn<Cfg, Val, Symbol, Val> for SymbolEval<'a, Fn>
where Fn: MutFn<Cfg, Val, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        cfg.step();
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                let Some(val) = ctx.ref_(s) else {
                    return Val::default();
                };
                val.clone()
            }
            SYMBOL_EVAL_CHAR => {
                let Some(val) = ctx.ref_(s) else {
                    return Val::default();
                };
                let val = val.clone();
                self.f.mut_call(cfg, ctx, val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

pub(crate) struct CallEval<'a, Func> {
    pub(crate) func: &'a Func,
}

pub(crate) const CFG_ADAPTER: &str = "adapter";

pub(crate) fn import_adapter(
    cfg: &mut Cfg, id: Symbol,
) -> Option<Box<dyn MutFn<Cfg, Val, Val, Val>>> {
    let id = format!("{CFG_ADAPTER}@{}", &*id);
    if let Some(adapter) = cfg.import(Symbol::from_string_unchecked(id)) {
        let Val::Func(adapter) = adapter else {
            error!("adapter should be valid");
            return None;
        };
        Some(Box::new(adapter))
    } else {
        Some(Box::new(Eval))
    }
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
        let Some(adapter) = import_adapter(cfg, func.id()) else {
            return Val::default();
        };
        let input = adapter.free_call(cfg, call.input);
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
        let Some(adapter) = import_adapter(cfg, func.id()) else {
            return Val::default();
        };
        let input = adapter.const_call(cfg, ctx.reborrow(), call.input);
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
        let Some(adapter) = import_adapter(cfg, func.id()) else {
            return Val::default();
        };
        let input = adapter.mut_call(cfg, ctx, call.input);
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
            v => Form.free_call(cfg, v),
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
            v => Form.const_call(cfg, ctx, v),
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
            v => Form.mut_call(cfg, ctx, v),
        }
    }
}

impl FreeFn<Cfg, Symbol, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Symbol, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Symbol, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, PairVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.mut_call(cfg, ctx, input)
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
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.mut_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, MapVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.mut_call(cfg, ctx, input)
    }
}
