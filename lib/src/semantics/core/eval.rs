use log::error;

use super::ListForm;
use super::MapForm;
use super::PairForm;
use super::const_ctx_ref;
use super::mut_ctx_ref;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::FuncSetup;
use crate::semantics::func::MutFn;
use crate::semantics::solver::Solve;
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

impl<'a, Fn> FreeFn<Symbol, Val> for SymbolEval<'a, Fn> {
    fn free_call(&self, input: Symbol) -> Val {
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

impl<'a, Fn> ConstFn<Val, Symbol, Val> for SymbolEval<'a, Fn>
where Fn: ConstFn<Val, Val, Val>
{
    fn const_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
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
                self.f.const_call(ConstRef::new(ctx), val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> MutFn<Val, Symbol, Val> for SymbolEval<'a, Fn>
where Fn: MutFn<Val, Val, Val>
{
    fn mut_call(&self, ctx: &mut Val, input: Symbol) -> Val {
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
                self.f.mut_call(ctx, val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

pub(crate) struct TaskEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> FreeFn<TaskVal, Val> for TaskEval<'a, Func>
where Func: FreeFn<Val, Val>
{
    fn free_call(&self, task: TaskVal) -> Val {
        let task = Task::from(task);
        let func = self.func.free_call(task.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let input = func.call().free_call(task.input);
                func.free_call(input)
            }
            Action::Solve => {
                let input = func.solve().free_call(task.input);
                Solve { func }.free_call(input)
            }
        }
    }
}

impl<'a, Func> ConstFn<Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: ConstFn<Val, Val, Val>
{
    fn const_call(&self, mut c: ConstRef<Val>, task: TaskVal) -> Val {
        let task = Task::from(task);
        let func = self.func.const_call(c.reborrow(), task.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let input = func.call().const_call(c.reborrow(), task.input);
                let Some(c) = const_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                func.const_call(c, input)
            }
            Action::Solve => {
                let input = func.solve().const_call(c.reborrow(), task.input);
                let Some(c) = const_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                Solve { func }.const_call(c, input)
            }
        }
    }
}

impl<'a, Func> MutFn<Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: MutFn<Val, Val, Val>
{
    fn mut_call(&self, c: &mut Val, task: TaskVal) -> Val {
        let task = Task::from(task);
        let func = self.func.mut_call(c, task.func);
        let Val::Func(func) = func else {
            error!("func {func:?} should be a func");
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let input = func.call().mut_call(c, task.input);
                let Some(c) = mut_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                func.dyn_call(c, input)
            }
            Action::Solve => {
                let input = func.solve().mut_call(c, task.input);
                let Some(c) = mut_ctx_ref(c, task.ctx) else {
                    return Val::default();
                };
                Solve { func }.dyn_call(c, input)
            }
        }
    }
}

pub(crate) struct TaskApply;

impl FreeFn<TaskVal, Val> for TaskApply {
    fn free_call(&self, task: TaskVal) -> Val {
        let task = Task::from(task);
        let Val::Func(func) = task.func else {
            error!("func {:?} should be a func", task.func);
            return Val::default();
        };
        match task.action {
            Action::Call => func.free_call(task.input),
            Action::Solve => Solve { func }.free_call(task.input),
        }
    }
}

impl ConstFn<Val, TaskVal, Val> for TaskApply {
    fn const_call(&self, ctx: ConstRef<Val>, task: TaskVal) -> Val {
        let task = Task::from(task);
        let Val::Func(func) = task.func else {
            error!("func {:?} should be a func", task.func);
            return Val::default();
        };
        let Some(ctx) = const_ctx_ref(ctx, task.ctx) else {
            return Val::default();
        };
        match task.action {
            Action::Call => func.const_call(ctx, task.input),
            Action::Solve => Solve { func }.const_call(ctx, task.input),
        }
    }
}

impl MutFn<Val, TaskVal, Val> for TaskApply {
    fn mut_call(&self, ctx: &mut Val, task: TaskVal) -> Val {
        let task = Task::from(task);
        let Val::Func(func) = task.func else {
            error!("func {:?} should be a func", task.func);
            return Val::default();
        };
        let Some(ctx) = mut_ctx_ref(ctx, task.ctx) else {
            return Val::default();
        };
        match task.action {
            Action::Call => func.dyn_call(ctx, task.input),
            Action::Solve => Solve { func }.dyn_call(ctx, task.input),
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Eval;

impl FreeFn<Val, Val> for Eval {
    fn free_call(&self, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_call(symbol),
            Val::Pair(pair) => self.free_call(pair),
            Val::Task(task) => self.free_call(task),
            Val::List(list) => self.free_call(list),
            Val::Map(map) => self.free_call(map),
            v => v,
        }
    }
}

impl ConstFn<Val, Val, Val> for Eval {
    fn const_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_call(ctx, symbol),
            Val::Pair(pair) => self.const_call(ctx, pair),
            Val::Task(task) => self.const_call(ctx, task),
            Val::List(list) => self.const_call(ctx, list),
            Val::Map(map) => self.const_call(ctx, map),
            v => v,
        }
    }
}

impl MutFn<Val, Val, Val> for Eval {
    fn mut_call(&self, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_call(ctx, symbol),
            Val::Pair(pair) => self.mut_call(ctx, pair),
            Val::Task(task) => self.mut_call(ctx, task),
            Val::List(list) => self.mut_call(ctx, list),
            Val::Map(map) => self.mut_call(ctx, map),
            v => v,
        }
    }
}

impl FreeFn<Symbol, Val> for Eval {
    fn free_call(&self, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.free_call(input)
    }
}

impl ConstFn<Val, Symbol, Val> for Eval {
    fn const_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, Symbol, Val> for Eval {
    fn mut_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.mut_call(ctx, input)
    }
}

impl FreeFn<PairVal, Val> for Eval {
    fn free_call(&self, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.free_call(input)
    }
}

impl ConstFn<Val, PairVal, Val> for Eval {
    fn const_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, PairVal, Val> for Eval {
    fn mut_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.mut_call(ctx, input)
    }
}

impl FreeFn<TaskVal, Val> for Eval {
    fn free_call(&self, input: TaskVal) -> Val {
        TaskEval { func: self }.free_call(input)
    }
}

impl ConstFn<Val, TaskVal, Val> for Eval {
    fn const_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        TaskEval { func: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, TaskVal, Val> for Eval {
    fn mut_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        TaskEval { func: self }.mut_call(ctx, input)
    }
}

impl FreeFn<ListVal, Val> for Eval {
    fn free_call(&self, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.free_call(input)
    }
}

impl ConstFn<Val, ListVal, Val> for Eval {
    fn const_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, ListVal, Val> for Eval {
    fn mut_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.mut_call(ctx, input)
    }
}

impl FreeFn<MapVal, Val> for Eval {
    fn free_call(&self, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.free_call(input)
    }
}

impl ConstFn<Val, MapVal, Val> for Eval {
    fn const_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.const_call(ctx, input)
    }
}

impl MutFn<Val, MapVal, Val> for Eval {
    fn mut_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.mut_call(ctx, input)
    }
}
