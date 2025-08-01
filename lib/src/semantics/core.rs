use std::borrow::Borrow;
use std::hash::Hash;
use std::mem::take;

use const_format::concatcp;
use log::error;
use num_traits::ToPrimitive;

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
use crate::semantics::ctx::Contract;
use crate::type_::Action;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Symbol;
use crate::type_::Task;

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
    fn free_static_call(&self, mut task: TaskVal) -> Val {
        task.func = self.func.free_static_call(take(&mut task.func));
        task.ctx = self.ctx.free_static_call(take(&mut task.ctx));
        task.input = self.input.free_static_call(take(&mut task.input));
        Val::Task(task)
    }
}

impl<'a, Func, Ctx, Input, C> ConstStaticFn<C, TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: ConstStaticFn<C, Val, Val>,
    Ctx: ConstStaticFn<C, Val, Val>,
    Input: ConstStaticFn<C, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<C>, mut task: TaskVal) -> Val {
        task.func = self.func.const_static_call(ctx.reborrow(), take(&mut task.func));
        task.ctx = self.ctx.const_static_call(ctx.reborrow(), take(&mut task.ctx));
        task.input = self.input.const_static_call(ctx, take(&mut task.input));
        Val::Task(task)
    }
}

impl<'a, Func, Ctx, Input, C> MutStaticFn<C, TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: MutStaticFn<C, Val, Val>,
    Ctx: MutStaticFn<C, Val, Val>,
    Input: MutStaticFn<C, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut C, mut task: TaskVal) -> Val {
        task.func = self.func.mut_static_call(ctx, take(&mut task.func));
        task.ctx = self.ctx.mut_static_call(ctx, take(&mut task.ctx));
        task.input = self.input.mut_static_call(ctx, take(&mut task.input));
        Val::Task(task)
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
            list.push(f.free_static_call(val));
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
            list.push(f.const_static_call(ctx.reborrow(), val));
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
            list.push(f.mut_static_call(ctx, val));
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
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.free_static_call(take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.free_static_call(take(v));
                } else {
                    *v = self.else_.free_static_call(take(v));
                }
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
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.const_static_call(ctx.reborrow(), take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.const_static_call(ctx.reborrow(), take(v));
                } else {
                    *v = self.else_.const_static_call(ctx.reborrow(), take(v));
                }
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
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.mut_static_call(ctx, take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.mut_static_call(ctx, take(v));
                } else {
                    *v = self.else_.mut_static_call(ctx, take(v));
                }
            }
        }
        Val::Map(input)
    }
}

pub(crate) struct SymbolEval<'a, Fn> {
    pub(crate) default: char,
    pub(crate) f: &'a Fn,
}

pub(crate) const SYMBOL_LITERAL_CHAR: char = '.';
pub(crate) const SYMBOL_LITERAL: &str = concatcp!(SYMBOL_LITERAL_CHAR);
pub(crate) const SYMBOL_REF_CHAR: char = '@';
pub(crate) const SYMBOL_REF: &str = concatcp!(SYMBOL_REF_CHAR);
pub(crate) const SYMBOL_EVAL_CHAR: char = '$';
pub(crate) const SYMBOL_EVAL: &str = concatcp!(SYMBOL_EVAL_CHAR);

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
                let Some(val) = get_ref(&ctx, s) else {
                    return Val::default();
                };
                val.clone()
            }
            SYMBOL_EVAL_CHAR => {
                let Some(val) = get_ref(&ctx, s) else {
                    return Val::default();
                };
                let val = val.clone();
                self.f.const_static_call(ctx, val)
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
                let Some(val) = get_ref(ctx, s) else {
                    return Val::default();
                };
                val.clone()
            }
            SYMBOL_EVAL_CHAR => {
                let Some(val) = get_ref(ctx, s) else {
                    return Val::default();
                };
                self.f.mut_static_call(ctx, val.clone())
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

fn get_ref(ctx: &Val, name: Symbol) -> Option<&Val> {
    let Val::Ctx(ctx) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return None;
    };
    let Ok(val) = ctx.get_ref(name.clone()) else {
        error!("name {name:?} should exist");
        return None;
    };
    Some(val)
}

fn eval_const_ctx(c: ConstRef<Val>, ctx: Val) -> Option<ConstRef<Val>> {
    eval_mut_ctx(c.unwrap(), ctx).map(DynRef::into_const)
}

fn eval_mut_ctx(c: &mut Val, ctx: Val) -> Option<DynRef<'_, Val>> {
    if ctx.is_unit() {
        return Some(DynRef::new_mut(c));
    }
    match c {
        Val::Pair(pair_val) => {
            let Val::Symbol(name) = ctx else {
                error!("ctx {ctx:?} should be a symbol");
                return None;
            };
            match &*name {
                "first" => Some(DynRef::new_mut(&mut pair_val.first)),
                "second" => Some(DynRef::new_mut(&mut pair_val.second)),
                _ => None,
            }
        }
        Val::Task(task_val) => {
            let Val::Symbol(name) = ctx else {
                error!("ctx {ctx:?} should be a symbol");
                return None;
            };
            match &*name {
                "function" => Some(DynRef::new_mut(&mut task_val.func)),
                "context" => Some(DynRef::new_mut(&mut task_val.ctx)),
                "input" => Some(DynRef::new_mut(&mut task_val.input)),
                _ => None,
            }
        }
        Val::List(list_val) => {
            let Val::Int(index) = ctx else {
                error!("ctx {ctx:?} should be a int");
                return None;
            };
            let len = list_val.len();
            let Some(index) = index.to_usize() else {
                error!("index {index:?} should >= 0 and < list.len {len}");
                return None;
            };
            let Some(val) = list_val.get_mut(index) else {
                error!("index {index} should < list.len {len}");
                return None;
            };
            Some(DynRef::new_mut(val))
        }
        Val::Map(map_val) => {
            let Some(val) = map_val.get_mut(&ctx) else {
                error!("ctx {ctx:?} should exist in the map");
                return None;
            };
            Some(DynRef::new_mut(val))
        }
        Val::Ctx(ctx_val) => {
            let Val::Symbol(name) = ctx else {
                error!("ctx {ctx:?} should be a symbol");
                return None;
            };
            let Ok(val_ref) = ctx_val.get_ref_dyn(name.clone()) else {
                error!("name {name:?} should exist");
                return None;
            };
            Some(val_ref)
        }
        _ => {
            error!("ctx {c:?} should be a pair, a task, a list, a map or a ctx");
            None
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
                    let _ = func.call_ctx().free_static_call(task.ctx);
                    let input = func.call_input().free_static_call(task.input);
                    func.free_static_call(input)
                }
                Action::Solve => {
                    let _ = func.solve_ctx().free_static_call(task.ctx);
                    let input = func.solve_input().free_static_call(task.input);
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
                    let ctx = func.call_ctx().const_static_call(c.reborrow(), task.ctx);
                    let input = func.call_input().const_static_call(c.reborrow(), task.input);
                    let Some(c) = eval_const_ctx(c, ctx) else {
                        return Val::default();
                    };
                    func.const_static_call(c, input)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().const_static_call(c.reborrow(), task.ctx);
                    let input = func.solve_input().const_static_call(c.reborrow(), task.input);
                    let Some(c) = eval_const_ctx(c, ctx) else {
                        return Val::default();
                    };
                    Solve { func }.const_static_call(c, input)
                }
            },
            Val::Symbol(func) => with_lock(c.unwrap(), func, |c, func, _| match task.action {
                Action::Call => {
                    let ctx = func.call_ctx().const_static_call(ConstRef::new(c), task.ctx);
                    let input = func.call_input().const_static_call(ConstRef::new(c), task.input);
                    let Some(c1) = eval_const_ctx(ConstRef::new(c), ctx) else {
                        return (func, Val::default());
                    };
                    let output = func.const_static_call(c1, input);
                    (func, output)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().const_static_call(ConstRef::new(c), task.ctx);
                    let input = func.solve_input().const_static_call(ConstRef::new(c), task.input);
                    let Some(c1) = eval_const_ctx(ConstRef::new(c), ctx) else {
                        return (func, Val::default());
                    };
                    let solve = Solve { func };
                    let output = solve.const_static_call(c1, input);
                    (solve.func, output)
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
                    let ctx = func.call_ctx().mut_static_call(c, task.ctx);
                    let input = func.call_input().mut_static_call(c, task.input);
                    let Some(c) = eval_mut_ctx(c, ctx) else {
                        return Val::default();
                    };
                    func.dyn_static_call(c, input)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().mut_static_call(c, task.ctx);
                    let input = func.solve_input().mut_static_call(c, task.input);
                    let Some(c) = eval_mut_ctx(c, ctx) else {
                        return Val::default();
                    };
                    Solve { func }.dyn_static_call(c, input)
                }
            },
            Val::Symbol(func) => with_lock(c, func, |c, mut func, contract| match task.action {
                Action::Call => {
                    let ctx = func.call_ctx().mut_static_call(c, task.ctx);
                    let input = func.call_input().mut_static_call(c, task.input);
                    let Some(c1) = eval_mut_ctx(c, ctx) else {
                        return (func, Val::default());
                    };
                    let output = if contract.is_mutable() {
                        func.dyn_cell_call(c1, input)
                    } else {
                        func.dyn_static_call(c1, input)
                    };
                    (func, output)
                }
                Action::Solve => {
                    let ctx = func.solve_ctx().mut_static_call(c, task.ctx);
                    let input = func.solve_input().mut_static_call(c, task.input);
                    let Some(c) = eval_mut_ctx(c, ctx) else {
                        return (func, Val::default());
                    };
                    let solve = Solve { func };
                    let output = solve.dyn_static_call(c, input);
                    (solve.func, output)
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
                let Some(ctx) = eval_const_ctx(ctx, task.ctx) else {
                    return Val::default();
                };
                match task.action {
                    Action::Call => func.const_static_call(ctx, task.input),
                    Action::Solve => Solve { func }.const_static_call(ctx, task.input),
                }
            }
            Val::Symbol(func) => with_lock(ctx.unwrap(), func, |ctx, func, _| {
                let Some(ctx1) = eval_const_ctx(ConstRef::new(ctx), task.ctx) else {
                    return (func, Val::default());
                };
                match task.action {
                    Action::Call => {
                        let output = func.const_static_call(ctx1, task.input);
                        (func, output)
                    }
                    Action::Solve => {
                        let solve = Solve { func };
                        let output = solve.const_static_call(ctx1, task.input);
                        (solve.func, output)
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
                let Some(ctx) = eval_mut_ctx(ctx, task.ctx) else {
                    return Val::default();
                };
                match task.action {
                    Action::Call => func.dyn_static_call(ctx, task.input),
                    Action::Solve => Solve { func }.dyn_static_call(ctx, task.input),
                }
            }
            Val::Symbol(func) => with_lock(ctx, func, |ctx, mut func, contract| {
                let Some(ctx1) = eval_mut_ctx(ctx, task.ctx) else {
                    return (func, Val::default());
                };
                match task.action {
                    Action::Call => {
                        let output = if contract.is_mutable() {
                            func.dyn_cell_call(ctx1, task.input)
                        } else {
                            func.dyn_static_call(ctx1, task.input)
                        };
                        (func, output)
                    }
                    Action::Solve => {
                        let solve = Solve { func };
                        let output = solve.dyn_static_call(ctx1, task.input);
                        (solve.func, output)
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

fn with_lock<F>(c: &mut Val, func_name: Symbol, f: F) -> Val
where F: FnOnce(&mut Val, FuncVal, Contract) -> (FuncVal, Val) {
    let Val::Ctx(ctx_val) = c else {
        error!("ctx {c:?} should be a ctx");
        return Val::default();
    };
    let Ok(ctx_value) = ctx_val.lock(func_name.clone()) else {
        error!("func ref {func_name:?} should be lockable");
        return Val::default();
    };
    let Val::Func(func) = ctx_value.val else {
        error!("func ref {:?} should be a func", ctx_value.val);
        ctx_val.unlock(func_name, ctx_value.val);
        return Val::default();
    };
    let (func, output) = f(c, func, ctx_value.contract);
    let Val::Ctx(ctx_val) = c else {
        unreachable!("lock_unlock ctx invariant is broken!!!");
    };
    ctx_val.unlock(func_name, Val::Func(func));
    output
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
