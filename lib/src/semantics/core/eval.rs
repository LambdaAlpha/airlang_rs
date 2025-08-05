use std::mem::take;

use log::error;

use super::ListForm;
use super::MapForm;
use super::PairForm;
use super::const_ctx_ref;
use super::mut_ctx_ref;
use super::with_lock;
use crate::semantics::ctx::DynCtx;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::FuncSetup;
use crate::semantics::func::MutCellFn;
use crate::semantics::func::MutStaticFn;
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

impl<'a, Fn> FreeStaticFn<Symbol, Val> for SymbolEval<'a, Fn> {
    fn free_static_call(&self, input: Symbol) -> Val {
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

impl<'a, Fn> ConstStaticFn<Val, Symbol, Val> for SymbolEval<'a, Fn>
where Fn: ConstStaticFn<Val, Val, Val>
{
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
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
                self.f.const_static_call(ConstRef::new(ctx), val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> MutStaticFn<Val, Symbol, Val> for SymbolEval<'a, Fn>
where Fn: MutStaticFn<Val, Val, Val>
{
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
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
                self.f.mut_static_call(ctx, val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

pub(crate) struct TaskEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> FreeStaticFn<TaskVal, Val> for TaskEval<'a, Func>
where Func: FreeStaticFn<Val, Val>
{
    fn free_static_call(&self, task: TaskVal) -> Val {
        let task = Task::from(task);
        match self.func.free_static_call(task.func) {
            Val::Func(func) => match task.action {
                Action::Call => {
                    let input = func.call().free_static_call(task.input);
                    func.free_static_call(input)
                }
                Action::Solve => {
                    let input = func.solve().free_static_call(task.input);
                    Solve { func }.free_static_call(input)
                }
            },
            Val::Symbol(func) => {
                error!("func ref {func:?} should be evaluated in a ctx");
                Val::default()
            }
            func => {
                error!("func {func:?} should be a func or a symbol");
                Val::default()
            }
        }
    }
}

impl<'a, Func> ConstStaticFn<Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: ConstStaticFn<Val, Val, Val>
{
    fn const_static_call(&self, mut c: ConstRef<Val>, task: TaskVal) -> Val {
        let task = Task::from(task);
        match self.func.const_static_call(c.reborrow(), task.func) {
            Val::Func(func) => match task.action {
                Action::Call => {
                    let input = func.call().const_static_call(c.reborrow(), task.input);
                    let Some(c) = const_ctx_ref(c, task.ctx) else {
                        return Val::default();
                    };
                    func.const_static_call(c, input)
                }
                Action::Solve => {
                    let input = func.solve().const_static_call(c.reborrow(), task.input);
                    let Some(c) = const_ctx_ref(c, task.ctx) else {
                        return Val::default();
                    };
                    Solve { func }.const_static_call(c, input)
                }
            },
            Val::Symbol(func) => with_lock(c.unwrap(), func, |c, func, _| match task.action {
                Action::Call => {
                    let input = func.call().const_static_call(ConstRef::new(c), task.input);
                    let Some(c) = const_ctx_ref(ConstRef::new(c), task.ctx) else {
                        return Val::default();
                    };
                    func.const_static_call(c, input)
                }
                Action::Solve => {
                    let input = func.solve().const_static_call(ConstRef::new(c), task.input);
                    let Some(c) = const_ctx_ref(ConstRef::new(c), task.ctx) else {
                        return Val::default();
                    };
                    let solve = Solve { func: take(func) };
                    let output = solve.const_static_call(c, input);
                    *func = solve.func;
                    output
                }
            }),
            func => {
                error!("func {func:?} should be a func or a symbol");
                Val::default()
            }
        }
    }
}

impl<'a, Func> MutStaticFn<Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: MutStaticFn<Val, Val, Val>
{
    fn mut_static_call(&self, c: &mut Val, task: TaskVal) -> Val {
        let task = Task::from(task);
        match self.func.mut_static_call(c, task.func) {
            Val::Func(func) => match task.action {
                Action::Call => {
                    let input = func.call().mut_static_call(c, task.input);
                    let Some(c) = mut_ctx_ref(c, task.ctx) else {
                        return Val::default();
                    };
                    func.dyn_static_call(c, input)
                }
                Action::Solve => {
                    let input = func.solve().mut_static_call(c, task.input);
                    let Some(c) = mut_ctx_ref(c, task.ctx) else {
                        return Val::default();
                    };
                    Solve { func }.dyn_static_call(c, input)
                }
            },
            Val::Symbol(func) => with_lock(c, func, |c, func, contract| match task.action {
                Action::Call => {
                    let input = func.call().mut_static_call(c, task.input);
                    let Some(c) = mut_ctx_ref(c, task.ctx) else {
                        return Val::default();
                    };
                    if contract.is_mutable() {
                        func.dyn_cell_call(c, input)
                    } else {
                        func.dyn_static_call(c, input)
                    }
                }
                Action::Solve => {
                    let input = func.solve().mut_static_call(c, task.input);
                    let Some(c) = mut_ctx_ref(c, task.ctx) else {
                        return Val::default();
                    };
                    let solve = Solve { func: take(func) };
                    let output = solve.dyn_static_call(c, input);
                    *func = solve.func;
                    output
                }
            }),

            func => {
                error!("func {func:?} should be a func or a symbol");
                Val::default()
            }
        }
    }
}

pub(crate) struct TaskApply;

impl FreeStaticFn<TaskVal, Val> for TaskApply {
    fn free_static_call(&self, task: TaskVal) -> Val {
        let task = Task::from(task);
        match task.func {
            Val::Func(func) => match task.action {
                Action::Call => func.free_static_call(task.input),
                Action::Solve => Solve { func }.free_static_call(task.input),
            },
            Val::Symbol(func) => {
                error!("func ref {func:?} should be evaluated in a ctx");
                Val::default()
            }
            func => {
                error!("func {func:?} should be a func or a symbol");
                Val::default()
            }
        }
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for TaskApply {
    fn const_static_call(&self, ctx: ConstRef<Val>, task: TaskVal) -> Val {
        let task = Task::from(task);
        match task.func {
            Val::Func(func) => {
                let Some(ctx) = const_ctx_ref(ctx, task.ctx) else {
                    return Val::default();
                };
                match task.action {
                    Action::Call => func.const_static_call(ctx, task.input),
                    Action::Solve => Solve { func }.const_static_call(ctx, task.input),
                }
            }
            Val::Symbol(func) => with_lock(ctx.unwrap(), func, |ctx, func, _| {
                let Some(ctx) = const_ctx_ref(ConstRef::new(ctx), task.ctx) else {
                    return Val::default();
                };
                match task.action {
                    Action::Call => func.const_static_call(ctx, task.input),
                    Action::Solve => {
                        let solve = Solve { func: take(func) };
                        let output = solve.const_static_call(ctx, task.input);
                        *func = solve.func;
                        output
                    }
                }
            }),
            func => {
                error!("func {func:?} should be a func or a symbol");
                Val::default()
            }
        }
    }
}

impl MutStaticFn<Val, TaskVal, Val> for TaskApply {
    fn mut_static_call(&self, ctx: &mut Val, task: TaskVal) -> Val {
        let task = Task::from(task);
        match task.func {
            Val::Func(func) => {
                let Some(ctx) = mut_ctx_ref(ctx, task.ctx) else {
                    return Val::default();
                };
                match task.action {
                    Action::Call => func.dyn_static_call(ctx, task.input),
                    Action::Solve => Solve { func }.dyn_static_call(ctx, task.input),
                }
            }
            Val::Symbol(func) => with_lock(ctx, func, |ctx, func, contract| {
                let Some(ctx) = mut_ctx_ref(ctx, task.ctx) else {
                    return Val::default();
                };
                match task.action {
                    Action::Call => {
                        if contract.is_mutable() {
                            func.dyn_cell_call(ctx, task.input)
                        } else {
                            func.dyn_static_call(ctx, task.input)
                        }
                    }
                    Action::Solve => {
                        let solve = Solve { func: take(func) };
                        let output = solve.dyn_static_call(ctx, task.input);
                        *func = solve.func;
                        output
                    }
                }
            }),
            func => {
                error!("func {func:?} should be a func or a symbol");
                Val::default()
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Eval;

impl FreeStaticFn<Val, Val> for Eval {
    fn free_static_call(&self, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_static_call(symbol),
            Val::Pair(pair) => self.free_static_call(pair),
            Val::Task(task) => self.free_static_call(task),
            Val::List(list) => self.free_static_call(list),
            Val::Map(map) => self.free_static_call(map),
            v => v,
        }
    }
}

impl ConstStaticFn<Val, Val, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_static_call(ctx, symbol),
            Val::Pair(pair) => self.const_static_call(ctx, pair),
            Val::Task(task) => self.const_static_call(ctx, task),
            Val::List(list) => self.const_static_call(ctx, list),
            Val::Map(map) => self.const_static_call(ctx, map),
            v => v,
        }
    }
}

impl MutStaticFn<Val, Val, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_static_call(ctx, symbol),
            Val::Pair(pair) => self.mut_static_call(ctx, pair),
            Val::Task(task) => self.mut_static_call(ctx, task),
            Val::List(list) => self.mut_static_call(ctx, list),
            Val::Map(map) => self.mut_static_call(ctx, map),
            v => v,
        }
    }
}

impl FreeStaticFn<Symbol, Val> for Eval {
    fn free_static_call(&self, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        SymbolEval { default: SYMBOL_REF_CHAR, f: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<PairVal, Val> for Eval {
    fn free_static_call(&self, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, PairVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, PairVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        PairForm { some, first: self, second: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<TaskVal, Val> for Eval {
    fn free_static_call(&self, input: TaskVal) -> Val {
        TaskEval { func: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        TaskEval { func: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, TaskVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        TaskEval { func: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<ListVal, Val> for Eval {
    fn free_static_call(&self, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, ListVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, ListVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        let head = &List::<Eval>::default();
        ListForm { head, tail: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<MapVal, Val> for Eval {
    fn free_static_call(&self, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, MapVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, MapVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        let some = &Map::<Val, Eval>::default();
        MapForm { some, else_: self }.mut_static_call(ctx, input)
    }
}
