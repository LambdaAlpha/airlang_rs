use crate::semantics::cfg::Cfg;
use crate::semantics::core::abort_by_bug_with_msg;
use crate::semantics::core::form::CellForm;
use crate::semantics::core::form::ListForm;
use crate::semantics::core::form::MapForm;
use crate::semantics::core::form::PairForm;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::DynFunc;
use crate::semantics::func::PrimInput;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::QuoteVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::Key;
use crate::type_::Quote;

pub(crate) struct QuoteEval;

impl DynFunc<Cfg, Val, QuoteVal, Val> for QuoteEval {
    fn call(&self, _cfg: &mut Cfg, _ctx: &mut Val, quote: QuoteVal) -> Val {
        let quote = Quote::from(quote);
        quote.source
    }
}

pub(crate) struct CallEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> DynFunc<Cfg, Val, CallVal, Val> for CallEval<'a, Func>
where Func: DynFunc<Cfg, Val, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.call(cfg, ctx, call.func);
        let Val::Func(func) = func else {
            let msg = format!("eval: expected a function, but got {func}");
            return abort_by_bug_with_msg(cfg, msg.into());
        };
        let input = if matches!(func.input(), PrimInput::Eval) {
            Eval.call(cfg, ctx, call.input)
        } else {
            call.input
        };
        if !cfg.step() {
            return Val::default();
        }
        func.call(cfg, ctx, input)
    }
}

#[derive(Default, Copy, Clone)]
pub struct Eval;

impl DynFunc<Cfg, Val, Val, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, val: Val) -> Val {
        if !cfg.step() {
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
            v => v,
        }
    }
}

impl DynFunc<Cfg, Val, Key, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.call(cfg, ctx, key)
    }
}

impl DynFunc<Cfg, Val, CellVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.call(cfg, ctx, cell))
    }
}

impl DynFunc<Cfg, Val, PairVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.call(cfg, ctx, pair))
    }
}

impl DynFunc<Cfg, Val, ListVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.call(cfg, ctx, list))
    }
}

impl DynFunc<Cfg, Val, MapVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.call(cfg, ctx, map))
    }
}

impl DynFunc<Cfg, Val, QuoteVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, quote: QuoteVal) -> Val {
        QuoteEval.call(cfg, ctx, quote)
    }
}

impl DynFunc<Cfg, Val, CallVal, Val> for Eval {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        CallEval { func: self }.call(cfg, ctx, call)
    }
}
