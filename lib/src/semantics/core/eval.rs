use crate::semantics::cfg::Cfg;
use crate::semantics::core::abort_by_bug_with_msg;
use crate::semantics::core::form::CellForm;
use crate::semantics::core::form::ListForm;
use crate::semantics::core::form::MapForm;
use crate::semantics::core::form::PairForm;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::CtxFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::Key;

pub(crate) struct CallEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> CtxFn<Cfg, Val, CallVal, Val> for CallEval<'a, Func>
where Func: CtxFn<Cfg, Val, Val, Val>
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.ctx_call(cfg, ctx, call.func);
        let Val::Func(func) = func else {
            let msg = format!("eval: expected a function, but got {func}");
            return abort_by_bug_with_msg(cfg, msg.into());
        };
        let input = if func.raw_input() { call.input } else { Eval.ctx_call(cfg, ctx, call.input) };
        if !cfg.step() {
            return Val::default();
        }
        func.ctx_call(cfg, ctx, input)
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Eval;

impl CtxFn<Cfg, Val, Val, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.ctx_call(cfg, ctx, key),
            Val::Cell(cell) => self.ctx_call(cfg, ctx, cell),
            Val::Pair(pair) => self.ctx_call(cfg, ctx, pair),
            Val::Call(call) => self.ctx_call(cfg, ctx, call),
            Val::List(list) => self.ctx_call(cfg, ctx, list),
            Val::Map(map) => self.ctx_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl CtxFn<Cfg, Val, Key, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.ctx_call(cfg, ctx, key)
    }
}

impl CtxFn<Cfg, Val, CellVal, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.ctx_call(cfg, ctx, cell))
    }
}

impl CtxFn<Cfg, Val, PairVal, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.ctx_call(cfg, ctx, pair))
    }
}

impl CtxFn<Cfg, Val, CallVal, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        CallEval { func: self }.ctx_call(cfg, ctx, call)
    }
}

impl CtxFn<Cfg, Val, ListVal, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.ctx_call(cfg, ctx, list))
    }
}

impl CtxFn<Cfg, Val, MapVal, Val> for Eval {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.ctx_call(cfg, ctx, map))
    }
}
