use log::error;

use super::ListForm;
use super::MapForm;
use super::PairForm;
use super::const_ctx_ref;
use super::mut_ctx_ref;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FuncSetup;
use crate::semantics::func::MutFn;
use crate::semantics::solve::Solve;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::Action;
use crate::type_::ConstRef;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;
use crate::type_::Task;

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
    fn free_call(&self, _cfg: &mut Cfg, input: Symbol) -> Val {
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

pub(crate) struct TaskEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> FreeFn<Cfg, TaskVal, Val> for TaskEval<'a, Func>
where Func: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, task: TaskVal) -> Val {
        let task = Task::from(task);
        let func = self.func.free_call(cfg, task.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let input = func.call().free_call(cfg, task.input);
                func.free_call(cfg, input)
            }
            Action::Solve => {
                let input = func.solve().free_call(cfg, task.input);
                Solve { func }.free_call(cfg, input)
            }
        }
    }
}

impl<'a, Func> ConstFn<Cfg, Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: ConstFn<Cfg, Val, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut c: ConstRef<Val>, task: TaskVal) -> Val {
        let task = Task::from(task);
        let func = self.func.const_call(cfg, c.reborrow(), task.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let input = func.call().const_call(cfg, c.reborrow(), task.input);
                let Some(c) = const_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                func.const_call(cfg, c, input)
            }
            Action::Solve => {
                let input = func.solve().const_call(cfg, c.reborrow(), task.input);
                let Some(c) = const_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                Solve { func }.const_call(cfg, c, input)
            }
        }
    }
}

impl<'a, Func> MutFn<Cfg, Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: MutFn<Cfg, Val, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, c: &mut Val, task: TaskVal) -> Val {
        let task = Task::from(task);
        let func = self.func.mut_call(cfg, c, task.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let input = func.call().mut_call(cfg, c, task.input);
                let Some(c) = mut_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                func.dyn_call(cfg, c, input)
            }
            Action::Solve => {
                let input = func.solve().mut_call(cfg, c, task.input);
                let Some(c) = mut_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                Solve { func }.dyn_call(cfg, c, input)
            }
        }
    }
}

pub(crate) struct TaskApply;

impl FreeFn<Cfg, TaskVal, Val> for TaskApply {
    fn free_call(&self, cfg: &mut Cfg, task: TaskVal) -> Val {
        let task = Task::from(task);
        let Val::Func(func) = task.func else {
            error!("func {:?} should be a func", task.func);
            return Val::default();
        };
        match task.action {
            Action::Call => func.free_call(cfg, task.input),
            Action::Solve => Solve { func }.free_call(cfg, task.input),
        }
    }
}

impl ConstFn<Cfg, Val, TaskVal, Val> for TaskApply {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, task: TaskVal) -> Val {
        let task = Task::from(task);
        let Val::Func(func) = task.func else {
            error!("func {:?} should be a func", task.func);
            return Val::default();
        };
        let Some(ctx) = const_ctx_ref(ctx, task.ctx) else {
            return Val::default();
        };
        match task.action {
            Action::Call => func.const_call(cfg, ctx, task.input),
            Action::Solve => Solve { func }.const_call(cfg, ctx, task.input),
        }
    }
}

impl MutFn<Cfg, Val, TaskVal, Val> for TaskApply {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, task: TaskVal) -> Val {
        let task = Task::from(task);
        let Val::Func(func) = task.func else {
            error!("func {:?} should be a func", task.func);
            return Val::default();
        };
        let Some(ctx) = mut_ctx_ref(ctx, task.ctx) else {
            return Val::default();
        };
        match task.action {
            Action::Call => func.dyn_call(cfg, ctx, task.input),
            Action::Solve => Solve { func }.dyn_call(cfg, ctx, task.input),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Eval;

impl FreeFn<Cfg, Val, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_call(cfg, symbol),
            Val::Pair(pair) => self.free_call(cfg, pair),
            Val::Task(task) => self.free_call(cfg, task),
            Val::List(list) => self.free_call(cfg, list),
            Val::Map(map) => self.free_call(cfg, map),
            v => v,
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.const_call(cfg, ctx, pair),
            Val::Task(task) => self.const_call(cfg, ctx, task),
            Val::List(list) => self.const_call(cfg, ctx, list),
            Val::Map(map) => self.const_call(cfg, ctx, map),
            v => v,
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_call(cfg, ctx, symbol),
            Val::Pair(pair) => self.mut_call(cfg, ctx, pair),
            Val::Task(task) => self.mut_call(cfg, ctx, task),
            Val::List(list) => self.mut_call(cfg, ctx, list),
            Val::Map(map) => self.mut_call(cfg, ctx, map),
            v => v,
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

impl FreeFn<Cfg, TaskVal, Val> for Eval {
    fn free_call(&self, cfg: &mut Cfg, input: TaskVal) -> Val {
        TaskEval { func: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, TaskVal, Val> for Eval {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        TaskEval { func: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, TaskVal, Val> for Eval {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: TaskVal) -> Val {
        TaskEval { func: self }.mut_call(cfg, ctx, input)
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
