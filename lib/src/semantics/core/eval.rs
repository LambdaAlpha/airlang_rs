use const_format::concatcp;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::core::form::CellForm;
use crate::semantics::core::form::ListForm;
use crate::semantics::core::form::MapForm;
use crate::semantics::core::form::PairForm;
use crate::semantics::core::key::KeyEval;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::DynFunc;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::QuoteVal;
use crate::semantics::val::SolveVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::Key;
use crate::type_::Pair;
use crate::type_::Quote;
use crate::type_::Solve;

pub(crate) struct QuoteEval;

impl DynFunc<Val, QuoteVal, Val> for QuoteEval {
    fn call(&self, _cfg: &mut Cfg, _ctx: Ctx<Val>, quote: QuoteVal) -> Val {
        let quote = Quote::from(quote);
        quote.value
    }
}

pub(crate) struct CallEval<'a, Func, Input> {
    pub(crate) func: &'a Func,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Input> DynFunc<Val, CallVal, Val> for CallEval<'a, Func, Input>
where
    Func: DynFunc<Val, Val, Val>,
    Input: DynFunc<Val, Val, Val>,
{
    fn call(&self, cfg: &mut Cfg, mut ctx: Ctx<Val>, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.call(cfg, ctx.reborrow(), call.func);
        let Val::Func(func) = func else {
            return cfg.abort();
        };
        let input = self.input.call(cfg, ctx.reborrow(), call.input);
        if cfg.is_aborted() {
            return Val::default();
        }
        func.call(cfg, ctx, input)
    }
}

pub const SOLVER: &str = concatcp!(PREFIX_CELL, "solver");

pub(crate) struct SolveEval<'a, Func, Output> {
    pub(crate) func: &'a Func,
    pub(crate) output: &'a Output,
}

impl<'a, Func, Output> DynFunc<Val, SolveVal, Val> for SolveEval<'a, Func, Output>
where
    Func: DynFunc<Val, Val, Val>,
    Output: DynFunc<Val, Val, Val>,
{
    fn call(&self, cfg: &mut Cfg, mut ctx: Ctx<Val>, solve: SolveVal) -> Val {
        let solve = Solve::from(solve);
        let func = self.func.call(cfg, ctx.reborrow(), solve.func);
        let Val::Func(func) = func else {
            return cfg.abort();
        };
        let output = self.output.call(cfg, ctx.reborrow(), solve.output);
        if cfg.is_aborted() {
            return Val::default();
        }
        let Some(solver) = cfg.map.get(&Key::from_str_unchecked(SOLVER)) else {
            return cfg.abort();
        };
        let Val::Func(solver) = solver else {
            return cfg.abort();
        };
        let solver = solver.clone();
        let problem = Val::Pair(Pair::new(Val::Func(func.clone()), output.clone()).into());
        let Val::Fact(fact) = solver.call(cfg, ctx.reborrow(), problem) else {
            return cfg.abort();
        };
        if cfg.is_aborted() {
            return Val::default();
        }
        if *fact.func() != func || *fact.output() != output {
            return cfg.abort();
        }
        fact.input().clone()
    }
}

#[derive(Default, Copy, Clone)]
pub struct Eval;

impl DynFunc<Val, Val, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, val: Val) -> Val {
        if cfg.is_aborted() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.call(cfg, ctx, key),
            Val::Cell(cell) => self.call(cfg, ctx, cell),
            Val::Pair(pair) => self.call(cfg, ctx, pair),
            Val::List(list) => self.call(cfg, ctx, list),
            Val::Map(map) => self.call(cfg, ctx, map),
            Val::Quote(quote) => self.call(cfg, ctx, quote),
            Val::Call(call) => self.call(cfg, ctx, call),
            Val::Solve(solve) => self.call(cfg, ctx, solve),
            v => v,
        }
    }
}

impl DynFunc<Val, Key, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, key: Key) -> Val {
        KeyEval.call(cfg, ctx, key)
    }
}

impl DynFunc<Val, CellVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.call(cfg, ctx, cell))
    }
}

impl DynFunc<Val, PairVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.call(cfg, ctx, pair))
    }
}

impl DynFunc<Val, ListVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.call(cfg, ctx, list))
    }
}

impl DynFunc<Val, MapVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.call(cfg, ctx, map))
    }
}

impl DynFunc<Val, QuoteVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, quote: QuoteVal) -> Val {
        QuoteEval.call(cfg, ctx, quote)
    }
}

impl DynFunc<Val, CallVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, call: CallVal) -> Val {
        CallEval { func: self, input: self }.call(cfg, ctx, call)
    }
}

impl DynFunc<Val, SolveVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<Val>, solve: SolveVal) -> Val {
        SolveEval { func: self, output: self }.call(cfg, ctx, solve)
    }
}
