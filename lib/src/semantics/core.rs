use std::borrow::Borrow;
use std::hash::Hash;
use std::mem::take;

use const_format::concatcp;

use super::func::ConstStaticFn;
use super::func::FreeStaticFn;
use super::func::FuncSetup;
use super::func::MutCellFn;
use super::func::MutStaticFn;
use super::solver::Solve;
use super::val::FuncVal;
use super::val::ListVal;
use super::val::MapVal;
use super::val::PairVal;
use super::val::TaskVal;
use super::val::Val;
use crate::type_::Action;
use crate::type_::ConstRef;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;
use crate::type_::Task;

pub(crate) struct SymbolForm<'a, Fn> {
    pub(crate) default: char,
    pub(crate) f: &'a Fn,
}

pub(crate) const SYMBOL_LITERAL_CHAR: char = '.';
pub(crate) const SYMBOL_LITERAL: &str = concatcp!(SYMBOL_LITERAL_CHAR);
pub(crate) const SYMBOL_REF_CHAR: char = '@';
pub(crate) const SYMBOL_REF: &str = concatcp!(SYMBOL_REF_CHAR);
pub(crate) const SYMBOL_MOVE_CHAR: char = '#';
pub(crate) const SYMBOL_MOVE: &str = concatcp!(SYMBOL_MOVE_CHAR);
pub(crate) const SYMBOL_EVAL_CHAR: char = '$';
pub(crate) const SYMBOL_EVAL: &str = concatcp!(SYMBOL_EVAL_CHAR);

impl<'a, Fn> SymbolForm<'a, Fn> {
    fn recognize(&self, input: Symbol) -> (char, Symbol) {
        match input.chars().next() {
            Some(SYMBOL_LITERAL_CHAR) => {
                (SYMBOL_LITERAL_CHAR, Symbol::from_str_unchecked(&input[1 ..]))
            }
            Some(SYMBOL_REF_CHAR) => (SYMBOL_REF_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            Some(SYMBOL_MOVE_CHAR) => (SYMBOL_MOVE_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            Some(SYMBOL_EVAL_CHAR) => (SYMBOL_EVAL_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            _ => (self.default, input),
        }
    }
}

impl<'a, Fn> FreeStaticFn<Symbol, Val> for SymbolForm<'a, Fn> {
    fn free_static_call(&self, input: Symbol) -> Val {
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => Val::default(),
            SYMBOL_MOVE_CHAR => Val::default(),
            SYMBOL_EVAL_CHAR => Val::default(),
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> ConstStaticFn<Val, Symbol, Val> for SymbolForm<'a, Fn>
where Fn: ConstStaticFn<Val, Val, Val>
{
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                let Val::Ctx(ctx) = &*ctx else {
                    return Val::default();
                };
                ctx.variables().get_ref(s).cloned().unwrap_or_default()
            }
            SYMBOL_MOVE_CHAR => Val::default(),
            SYMBOL_EVAL_CHAR => {
                let Val::Ctx(ctx1) = &*ctx else {
                    return Val::default();
                };
                let Ok(val) = ctx1.variables().get_ref(s) else {
                    return Val::default();
                };
                let val = val.clone();
                self.f.const_static_call(ctx, val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> MutStaticFn<Val, Symbol, Val> for SymbolForm<'a, Fn>
where Fn: MutStaticFn<Val, Val, Val>
{
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                let Val::Ctx(ctx) = &*ctx else {
                    return Val::default();
                };
                ctx.variables().get_ref(s).cloned().unwrap_or_default()
            }
            SYMBOL_MOVE_CHAR => {
                let Val::Ctx(ctx) = ctx else {
                    return Val::default();
                };
                ctx.variables_mut().remove(s).unwrap_or_default()
            }
            SYMBOL_EVAL_CHAR => {
                let Val::Ctx(ctx1) = &*ctx else {
                    return Val::default();
                };
                let Ok(val) = ctx1.variables().get_ref(s) else {
                    return Val::default();
                };
                self.f.mut_static_call(ctx, val.clone())
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

pub(crate) struct PairForm<'a, Some, First, Second> {
    pub(crate) some: &'a Map<Val, Some>,
    pub(crate) first: &'a First,
    pub(crate) second: &'a Second,
}

impl<'a, Some, First, Second> FreeStaticFn<PairVal, Val> for PairForm<'a, Some, First, Second>
where
    Some: FreeStaticFn<Val, Val>,
    First: FreeStaticFn<Val, Val>,
    Second: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, mut input: PairVal) -> Val {
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.free_static_call(take(&mut input.second));
        } else {
            input.first = self.first.free_static_call(take(&mut input.first));
            input.second = self.second.free_static_call(take(&mut input.second));
        }
        Val::Pair(input)
    }
}

impl<'a, Some, First, Second, Ctx> ConstStaticFn<Ctx, PairVal, Val>
    for PairForm<'a, Some, First, Second>
where
    Some: ConstStaticFn<Ctx, Val, Val>,
    First: ConstStaticFn<Ctx, Val, Val>,
    Second: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, mut input: PairVal) -> Val {
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.const_static_call(ctx, take(&mut input.second));
        } else {
            input.first = self.first.const_static_call(ctx.reborrow(), take(&mut input.first));
            input.second = self.second.const_static_call(ctx, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

impl<'a, Some, First, Second, Ctx> MutStaticFn<Ctx, PairVal, Val>
    for PairForm<'a, Some, First, Second>
where
    Some: MutStaticFn<Ctx, Val, Val>,
    First: MutStaticFn<Ctx, Val, Val>,
    Second: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, mut input: PairVal) -> Val {
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.mut_static_call(ctx, take(&mut input.second));
        } else {
            input.first = self.first.mut_static_call(ctx, take(&mut input.first));
            input.second = self.second.mut_static_call(ctx, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

pub(crate) struct TaskForm<'a, Func, Ctx, Input> {
    pub(crate) func: &'a Func,
    pub(crate) ctx: &'a Ctx,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Ctx, Input> FreeStaticFn<TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: FreeStaticFn<Val, Val>,
    Ctx: FreeStaticFn<Val, Val>,
    Input: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, mut input: TaskVal) -> Val {
        input.func = self.func.free_static_call(take(&mut input.func));
        input.ctx = self.ctx.free_static_call(take(&mut input.ctx));
        input.input = self.input.free_static_call(take(&mut input.input));
        Val::Task(input)
    }
}

impl<'a, Func, Ctx, Input, C> ConstStaticFn<C, TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: ConstStaticFn<C, Val, Val>,
    Ctx: ConstStaticFn<C, Val, Val>,
    Input: ConstStaticFn<C, Val, Val>,
{
    fn const_static_call(&self, mut c: ConstRef<C>, mut input: TaskVal) -> Val {
        input.func = self.func.const_static_call(c.reborrow(), take(&mut input.func));
        input.ctx = self.ctx.const_static_call(c.reborrow(), take(&mut input.ctx));
        input.input = self.input.const_static_call(c, take(&mut input.input));
        Val::Task(input)
    }
}

impl<'a, Func, Ctx, Input, C> MutStaticFn<C, TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: MutStaticFn<C, Val, Val>,
    Ctx: MutStaticFn<C, Val, Val>,
    Input: MutStaticFn<C, Val, Val>,
{
    fn mut_static_call(&self, c: &mut C, mut input: TaskVal) -> Val {
        input.func = self.func.mut_static_call(c, take(&mut input.func));
        input.ctx = self.ctx.mut_static_call(c, take(&mut input.ctx));
        input.input = self.input.mut_static_call(c, take(&mut input.input));
        Val::Task(input)
    }
}

pub(crate) struct ListForm<'a, Head, Tail> {
    pub(crate) head: &'a List<Head>,
    pub(crate) tail: &'a Tail,
}

impl<'a, Head, Tail> FreeStaticFn<ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: FreeStaticFn<Val, Val>,
    Tail: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, mut input: ListVal) -> Val {
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.free_static_call(take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = f.free_static_call(val);
            list.push(val);
        }
        for val in iter {
            list.push(self.tail.free_static_call(val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> ConstStaticFn<Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: ConstStaticFn<Ctx, Val, Val>,
    Tail: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, mut input: ListVal) -> Val {
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.const_static_call(ctx.reborrow(), take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = f.const_static_call(ctx.reborrow(), val);
            list.push(val);
        }
        for val in iter {
            list.push(self.tail.const_static_call(ctx.reborrow(), val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> MutStaticFn<Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: MutStaticFn<Ctx, Val, Val>,
    Tail: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, mut input: ListVal) -> Val {
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.mut_static_call(ctx, take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = f.mut_static_call(ctx, val);
            list.push(val);
        }
        for val in iter {
            list.push(self.tail.mut_static_call(ctx, val));
        }
        Val::List(List::from(list).into())
    }
}

pub(crate) struct MapForm<'a, SomeKey, SomeValue, ElseValue> {
    pub(crate) some: &'a Map<SomeKey, SomeValue>,
    pub(crate) else_: &'a ElseValue,
}

impl<'a, SomeKey, SomeValue, ElseValue> FreeStaticFn<MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: FreeStaticFn<Val, Val>,
    ElseValue: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, mut input: MapVal) -> Val {
        for (k, v) in input.iter_mut() {
            if let Some(f) = self.some.get(k) {
                *v = f.free_static_call(take(v));
            } else {
                *v = self.else_.free_static_call(take(v));
            }
        }
        Val::Map(input)
    }
}

impl<'a, SomeKey, SomeValue, ElseValue, Ctx> ConstStaticFn<Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: ConstStaticFn<Ctx, Val, Val>,
    ElseValue: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, mut input: MapVal) -> Val {
        for (k, v) in input.iter_mut() {
            if let Some(f) = self.some.get(k) {
                *v = f.const_static_call(ctx.reborrow(), take(v));
            } else {
                *v = self.else_.const_static_call(ctx.reborrow(), take(v));
            }
        }
        Val::Map(input)
    }
}

impl<'a, SomeKey, SomeValue, ElseValue, Ctx> MutStaticFn<Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: MutStaticFn<Ctx, Val, Val>,
    ElseValue: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, mut input: MapVal) -> Val {
        for (k, v) in input.iter_mut() {
            if let Some(f) = self.some.get(k) {
                *v = f.mut_static_call(ctx, take(v));
            } else {
                *v = self.else_.mut_static_call(ctx, take(v));
            }
        }
        Val::Map(input)
    }
}

pub(crate) struct TaskEval<'a, Func> {
    pub(crate) func: &'a Func,
}

impl<'a, Func> FreeStaticFn<TaskVal, Val> for TaskEval<'a, Func>
where Func: FreeStaticFn<Val, Val>
{
    fn free_static_call(&self, input: TaskVal) -> Val {
        let task = Task::from(input);
        match self.func.free_static_call(task.func) {
            Val::Func(func) => match task.action {
                Action::Call => {
                    let _ = func.call_ctx().free_static_call(task.ctx);
                    let input = func.call_input().free_static_call(task.input);
                    func.free_static_call(input)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().free_static_call(task.ctx);
                    let input = func.solve_input().free_static_call(task.input);
                    let task = Task { action: Action::Solve, func: Val::Func(func), ctx, input };
                    Solve.free_static_call(Val::Task(task.into()))
                }
            },
            Val::Symbol(func) => {
                let task = Task { action: task.action, func, ctx: task.ctx, input: task.input };
                TaskRefEval.free_static_call(task)
            }
            _ => Val::default(),
        }
    }
}

impl<'a, Func> ConstStaticFn<Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: ConstStaticFn<Val, Val, Val>
{
    fn const_static_call(&self, mut c: ConstRef<Val>, input: TaskVal) -> Val {
        let task = Task::from(input);
        match self.func.const_static_call(c.reborrow(), task.func) {
            Val::Func(func) => match task.action {
                Action::Call => {
                    let ctx = func.call_ctx().const_static_call(c.reborrow(), task.ctx);
                    let input = func.call_input().const_static_call(c.reborrow(), task.input);
                    const_static_func_call(c, &func, ctx, input)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().const_static_call(c.reborrow(), task.ctx);
                    let input = func.solve_input().const_static_call(c.reborrow(), task.input);
                    let task = Task { action: Action::Solve, func: Val::Func(func), ctx, input };
                    Solve.const_static_call(c, Val::Task(task.into()))
                }
            },
            Val::Symbol(func) => {
                let task = Task { action: task.action, func, ctx: task.ctx, input: task.input };
                TaskRefEval.const_static_call(c, task)
            }
            _ => Val::default(),
        }
    }
}

impl<'a, Func> MutStaticFn<Val, TaskVal, Val> for TaskEval<'a, Func>
where Func: MutStaticFn<Val, Val, Val>
{
    fn mut_static_call(&self, c: &mut Val, input: TaskVal) -> Val {
        let task = Task::from(input);
        match self.func.mut_static_call(c, task.func) {
            Val::Func(func) => match task.action {
                Action::Call => {
                    let ctx = func.call_ctx().mut_static_call(c, task.ctx);
                    let input = func.call_input().mut_static_call(c, task.input);
                    mut_static_func_call(c, &func, ctx, input)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().mut_static_call(c, task.ctx);
                    let input = func.solve_input().mut_static_call(c, task.input);
                    let task = Task { action: Action::Solve, func: Val::Func(func), ctx, input };
                    Solve.mut_static_call(c, Val::Task(task.into()))
                }
            },
            Val::Symbol(func) => {
                let task = Task { action: task.action, func, ctx: task.ctx, input: task.input };
                TaskRefEval.mut_static_call(c, task)
            }
            _ => Val::default(),
        }
    }
}

pub(crate) struct TaskRefEval;

impl FreeStaticFn<Task<Symbol, Val, Val>, Val> for TaskRefEval {
    fn free_static_call(&self, _input: Task<Symbol, Val, Val>) -> Val {
        Val::default()
    }
}

impl ConstStaticFn<Val, Task<Symbol, Val, Val>, Val> for TaskRefEval {
    fn const_static_call(&self, c: ConstRef<Val>, task: Task<Symbol, Val, Val>) -> Val {
        let c = c.unwrap();
        let Val::Ctx(ctx_val) = c else {
            return Val::default();
        };
        let Ok(ctx_value) = ctx_val.variables_mut().lock(task.func.clone()) else {
            return Val::default();
        };
        let Val::Func(func) = ctx_value.val else {
            ctx_val.variables_mut().unlock(task.func, ctx_value.val);
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let ctx = func.call_ctx().const_static_call(ConstRef::new(c), task.ctx);
                let input = func.call_input().const_static_call(ConstRef::new(c), task.input);
                let output = const_static_func_call(ConstRef::new(c), &func, ctx, input);
                let Val::Ctx(ctx_val) = c else {
                    unreachable!("TaskRefEval call ctx invariant is broken!!!");
                };
                ctx_val.variables_mut().unlock(task.func, Val::Func(func));
                output
            }
            Action::Solve => {
                let ctx = func.solve_ctx().const_static_call(ConstRef::new(c), task.ctx);
                let input = func.solve_input().const_static_call(ConstRef::new(c), task.input);
                let Val::Ctx(ctx_val) = c else {
                    unreachable!("TaskRefEval solve ctx invariant is broken!!!");
                };
                ctx_val.variables_mut().unlock(task.func.clone(), Val::Func(func));
                let task = Task { action: Action::Solve, func: Val::Symbol(task.func), ctx, input };
                Solve.const_static_call(ConstRef::new(c), Val::Task(task.into()))
            }
        }
    }
}

impl MutStaticFn<Val, Task<Symbol, Val, Val>, Val> for TaskRefEval {
    fn mut_static_call(&self, c: &mut Val, task: Task<Symbol, Val, Val>) -> Val {
        let Val::Ctx(ctx_val) = c else {
            return Val::default();
        };
        let Ok(ctx_value) = ctx_val.variables_mut().lock(task.func.clone()) else {
            return Val::default();
        };
        let Val::Func(mut func) = ctx_value.val else {
            ctx_val.variables_mut().unlock(task.func, ctx_value.val);
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let ctx = func.call_ctx().mut_static_call(c, task.ctx);
                let input = func.call_input().mut_static_call(c, task.input);
                let output = if ctx_value.contract.is_mutable() {
                    mut_cell_func_call(c, &mut func, ctx, input)
                } else {
                    mut_static_func_call(c, &func, ctx, input)
                };
                let Val::Ctx(ctx_val) = c else {
                    unreachable!("TaskRefEval call ctx invariant is broken!!!");
                };
                ctx_val.variables_mut().unlock(task.func, Val::Func(func));
                output
            }
            Action::Solve => {
                let ctx = func.solve_ctx().mut_static_call(c, task.ctx);
                let input = func.solve_input().mut_static_call(c, task.input);
                let Val::Ctx(ctx_val) = c else {
                    unreachable!("TaskRefEval solve ctx invariant is broken!!!");
                };
                ctx_val.variables_mut().unlock(task.func.clone(), Val::Func(func));
                let task = Task { action: Action::Solve, func: Val::Symbol(task.func), ctx, input };
                Solve.mut_static_call(c, Val::Task(task.into()))
            }
        }
    }
}

pub(crate) struct TaskApply;

impl FreeStaticFn<TaskVal, Val> for TaskApply {
    fn free_static_call(&self, input: TaskVal) -> Val {
        let task = Task::from(input);
        match task.func {
            Val::Func(func) => match task.action {
                Action::Call => func.free_static_call(task.input),
                Action::Solve => {
                    let task = Task {
                        action: Action::Solve,
                        func: Val::Func(func),
                        ctx: task.ctx,
                        input: task.input,
                    };
                    Solve.free_static_call(Val::Task(task.into()))
                }
            },
            Val::Symbol(func) => {
                let task = Task { action: task.action, func, ctx: task.ctx, input: task.input };
                TaskRefApply.free_static_call(task)
            }
            _ => Val::default(),
        }
    }
}

impl ConstStaticFn<Val, TaskVal, Val> for TaskApply {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: TaskVal) -> Val {
        let task = Task::from(input);
        match task.func {
            Val::Func(func) => match task.action {
                Action::Call => const_static_func_call(ctx, &func, task.ctx, task.input),
                Action::Solve => {
                    let task = Task {
                        action: Action::Solve,
                        func: Val::Func(func),
                        ctx: task.ctx,
                        input: task.input,
                    };
                    Solve.const_static_call(ctx, Val::Task(task.into()))
                }
            },
            Val::Symbol(func) => {
                let task = Task { action: task.action, func, ctx: task.ctx, input: task.input };
                TaskRefApply.const_static_call(ctx, task)
            }
            _ => Val::default(),
        }
    }
}

impl MutStaticFn<Val, TaskVal, Val> for TaskApply {
    fn mut_static_call(&self, ctx: &mut Val, input: TaskVal) -> Val {
        let task = Task::from(input);
        match task.func {
            Val::Func(func) => match task.action {
                Action::Call => mut_static_func_call(ctx, &func, task.ctx, task.input),
                Action::Solve => {
                    let task = Task {
                        action: Action::Solve,
                        func: Val::Func(func),
                        ctx: task.ctx,
                        input: task.input,
                    };
                    Solve.mut_static_call(ctx, Val::Task(task.into()))
                }
            },
            Val::Symbol(func) => {
                let task = Task { action: task.action, func, ctx: task.ctx, input: task.input };
                TaskRefApply.mut_static_call(ctx, task)
            }
            _ => Val::default(),
        }
    }
}

pub(crate) struct TaskRefApply;

impl FreeStaticFn<Task<Symbol, Val, Val>, Val> for TaskRefApply {
    fn free_static_call(&self, _input: Task<Symbol, Val, Val>) -> Val {
        Val::default()
    }
}

impl ConstStaticFn<Val, Task<Symbol, Val, Val>, Val> for TaskRefApply {
    fn const_static_call(&self, ctx: ConstRef<Val>, task: Task<Symbol, Val, Val>) -> Val {
        let ctx = ctx.unwrap();
        let Val::Ctx(ctx_val) = ctx else {
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let Ok(ctx_value) = ctx_val.variables_mut().lock(task.func.clone()) else {
                    return Val::default();
                };
                let Val::Func(func) = ctx_value.val else {
                    ctx_val.variables_mut().unlock(task.func, ctx_value.val);
                    return Val::default();
                };
                let output =
                    const_static_func_call(ConstRef::new(ctx), &func, task.ctx, task.input);
                let Val::Ctx(ctx_val) = ctx else {
                    unreachable!("TaskRefApply ctx invariant is broken!!!");
                };
                ctx_val.variables_mut().unlock(task.func, Val::Func(func));
                output
            }
            Action::Solve => {
                let Ok(val) = ctx_val.variables().get_ref(task.func.clone()) else {
                    return Val::default();
                };
                let Val::Func(_) = val else {
                    return Val::default();
                };
                let task = Task {
                    action: task.action,
                    func: Val::Symbol(task.func),
                    ctx: task.ctx,
                    input: task.input,
                };
                Solve.const_static_call(ConstRef::new(ctx), Val::Task(task.into()))
            }
        }
    }
}

impl MutStaticFn<Val, Task<Symbol, Val, Val>, Val> for TaskRefApply {
    fn mut_static_call(&self, ctx: &mut Val, task: Task<Symbol, Val, Val>) -> Val {
        let Val::Ctx(ctx_val) = ctx else {
            return Val::default();
        };
        match task.action {
            Action::Call => {
                let Ok(ctx_value) = ctx_val.variables_mut().lock(task.func.clone()) else {
                    return Val::default();
                };
                let Val::Func(mut func) = ctx_value.val else {
                    ctx_val.variables_mut().unlock(task.func, ctx_value.val);
                    return Val::default();
                };
                let output = if ctx_value.contract.is_mutable() {
                    mut_cell_func_call(ctx, &mut func, task.ctx, task.input)
                } else {
                    mut_static_func_call(ctx, &func, task.ctx, task.input)
                };
                let Val::Ctx(ctx_val) = ctx else {
                    unreachable!("TaskRefApply ctx invariant is broken!!!");
                };
                ctx_val.variables_mut().unlock(task.func, Val::Func(func));
                output
            }
            Action::Solve => {
                let Ok(val) = ctx_val.variables().get_ref(task.func.clone()) else {
                    return Val::default();
                };
                let Val::Func(_) = val else {
                    return Val::default();
                };
                let task = Task {
                    action: task.action,
                    func: Val::Symbol(task.func),
                    ctx: task.ctx,
                    input: task.input,
                };
                Solve.mut_static_call(ctx, Val::Task(task.into()))
            }
        }
    }
}

fn const_static_func_call(c: ConstRef<Val>, func: &FuncVal, ctx: Val, input: Val) -> Val {
    if ctx.is_unit() {
        return func.const_static_call(c, input);
    }
    let Val::Ctx(ctx_val) = c.unwrap() else {
        return Val::default();
    };
    let Val::Symbol(name) = ctx else {
        return Val::default();
    };
    let Ok(val_ref) = ctx_val.variables_mut().get_ref_dyn(name) else {
        return Val::default();
    };
    func.const_static_call(val_ref.into_const(), input)
}

fn mut_static_func_call(c: &mut Val, func: &FuncVal, ctx: Val, input: Val) -> Val {
    if ctx.is_unit() {
        return func.mut_static_call(c, input);
    }
    let Val::Ctx(ctx_val) = c else {
        return Val::default();
    };
    let Val::Symbol(name) = ctx else {
        return Val::default();
    };
    let Ok(val_ref) = ctx_val.variables_mut().get_ref_dyn(name) else {
        return Val::default();
    };
    func.dyn_static_call(val_ref, input)
}

fn mut_cell_func_call(c: &mut Val, func: &mut FuncVal, ctx: Val, input: Val) -> Val {
    if ctx.is_unit() {
        return func.mut_cell_call(c, input);
    }
    let Val::Ctx(ctx_val) = c else {
        return Val::default();
    };
    let Val::Symbol(name) = ctx else {
        return Val::default();
    };
    let Ok(val_ref) = ctx_val.variables_mut().get_ref_dyn(name) else {
        return Val::default();
    };
    func.dyn_cell_call(val_ref, input)
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
        SymbolForm { default: SYMBOL_REF_CHAR, f: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolForm { default: SYMBOL_REF_CHAR, f: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        SymbolForm { default: SYMBOL_REF_CHAR, f: self }.mut_static_call(ctx, input)
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
