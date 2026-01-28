use crate::semantics::cfg::Cfg;
use crate::semantics::core::abort_by_bug_with_msg;
use crate::semantics::core::form::CellForm;
use crate::semantics::core::form::ListForm;
use crate::semantics::core::form::MapForm;
use crate::semantics::core::form::PairForm;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Key;

pub(crate) struct CallEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> FreeFn<Cfg, CallVal, Val> for CallEval<'a, Func>
where Func: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.free_call(cfg, call.func);
        let Val::Func(func) = func else {
            let msg = format!("eval: expected a function, but got {func}");
            return abort_by_bug_with_msg(cfg, msg.into());
        };
        let input = if func.raw_input() { call.input } else { Eval.free_call(cfg, call.input) };
        if !cfg.step() {
            return Val::default();
        }
        func.free_call(cfg, input)
    }
}

impl<'a, Func> ConstFn<Cfg, Val, CallVal, Val> for CallEval<'a, Func>
where Func: ConstFn<Cfg, Val, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Val>, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.const_call(cfg, ctx.reborrow(), call.func);
        let Val::Func(func) = func else {
            let msg = format!("eval: expected a function, but got {func}");
            return abort_by_bug_with_msg(cfg, msg.into());
        };
        let input = if func.raw_input() {
            call.input
        } else {
            Eval.const_call(cfg, ctx.reborrow(), call.input)
        };
        if !cfg.step() {
            return Val::default();
        }
        func.const_call(cfg, ctx, input)
    }
}

impl<'a, Func> MutFn<Cfg, Val, CallVal, Val> for CallEval<'a, Func>
where Func: MutFn<Cfg, Val, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.mut_call(cfg, ctx, call.func);
        let Val::Func(func) = func else {
            let msg = format!("eval: expected a function, but got {func}");
            return abort_by_bug_with_msg(cfg, msg.into());
        };
        let input = if func.raw_input() { call.input } else { Eval.mut_call(cfg, ctx, call.input) };
        if !cfg.step() {
            return Val::default();
        }
        func.mut_call(cfg, ctx, input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Val>, call: CallVal) -> Val {
        let call = Call::from(call);
        let func = self.func.dyn_call(cfg, ctx.reborrow(), call.func);
        let Val::Func(func) = func else {
            let msg = format!("eval: expected a function, but got {func}");
            return abort_by_bug_with_msg(cfg, msg.into());
        };
        let input = if func.raw_input() {
            call.input
        } else {
            Eval.dyn_call(cfg, ctx.reborrow(), call.input)
        };
        if !cfg.step() {
            return Val::default();
        }
        func.dyn_call(cfg, ctx, input)
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Eval;

impl FreeFn<Cfg, Val, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.free_call(cfg, key),
            Val::Cell(cell) => self.free_call(cfg, cell),
            Val::Pair(pair) => self.free_call(cfg, pair),
            Val::Call(call) => self.free_call(cfg, call),
            Val::List(list) => self.free_call(cfg, list),
            Val::Map(map) => self.free_call(cfg, map),
            v => v,
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.const_call(cfg, ctx, key),
            Val::Cell(cell) => self.const_call(cfg, ctx, cell),
            Val::Pair(pair) => self.const_call(cfg, ctx, pair),
            Val::Call(call) => self.const_call(cfg, ctx, call),
            Val::List(list) => self.const_call(cfg, ctx, list),
            Val::Map(map) => self.const_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.mut_call(cfg, ctx, key),
            Val::Cell(cell) => self.mut_call(cfg, ctx, cell),
            Val::Pair(pair) => self.mut_call(cfg, ctx, pair),
            Val::Call(call) => self.mut_call(cfg, ctx, call),
            Val::List(list) => self.mut_call(cfg, ctx, list),
            Val::Map(map) => self.mut_call(cfg, ctx, map),
            v => v,
        }
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.dyn_call(cfg, ctx, key),
            Val::Cell(cell) => self.dyn_call(cfg, ctx, cell),
            Val::Pair(pair) => self.dyn_call(cfg, ctx, pair),
            Val::Call(call) => self.dyn_call(cfg, ctx, call),
            Val::List(list) => self.dyn_call(cfg, ctx, list),
            Val::Map(map) => self.dyn_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl FreeFn<Cfg, Key, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, key: Key) -> Val {
        KeyEval.free_call(cfg, key)
    }
}

impl ConstFn<Cfg, Val, Key, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, key: Key) -> Val {
        KeyEval.const_call(cfg, ctx, key)
    }
}

impl MutFn<Cfg, Val, Key, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.mut_call(cfg, ctx, key)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, key: Key) -> Val {
        KeyEval.dyn_call(cfg, ctx, key)
    }
}

impl FreeFn<Cfg, CellVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.free_call(cfg, cell))
    }
}

impl ConstFn<Cfg, Val, CellVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.const_call(cfg, ctx, cell))
    }
}

impl MutFn<Cfg, Val, CellVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.mut_call(cfg, ctx, cell))
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, cell: CellVal) -> Val {
        Val::Cell(CellForm { value: self }.dyn_call(cfg, ctx, cell))
    }
}

impl FreeFn<Cfg, PairVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.free_call(cfg, pair))
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.const_call(cfg, ctx, pair))
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.mut_call(cfg, ctx, pair))
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, pair: PairVal) -> Val {
        Val::Pair(PairForm { left: self, right: self }.dyn_call(cfg, ctx, pair))
    }
}

impl FreeFn<Cfg, CallVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, call: CallVal) -> Val {
        CallEval { func: self }.free_call(cfg, call)
    }
}

impl ConstFn<Cfg, Val, CallVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, call: CallVal) -> Val {
        CallEval { func: self }.const_call(cfg, ctx, call)
    }
}

impl MutFn<Cfg, Val, CallVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> Val {
        CallEval { func: self }.mut_call(cfg, ctx, call)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, call: CallVal) -> Val {
        CallEval { func: self }.dyn_call(cfg, ctx, call)
    }
}

impl FreeFn<Cfg, ListVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.free_call(cfg, list))
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.const_call(cfg, ctx, list))
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.mut_call(cfg, ctx, list))
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, list: ListVal) -> Val {
        Val::List(ListForm { item: self }.dyn_call(cfg, ctx, list))
    }
}

impl FreeFn<Cfg, MapVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.free_call(cfg, map))
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.const_call(cfg, ctx, map))
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.mut_call(cfg, ctx, map))
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, map: MapVal) -> Val {
        Val::Map(MapForm { value: self }.dyn_call(cfg, ctx, map))
    }
}
