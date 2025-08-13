use std::borrow::Borrow;
use std::hash::Hash;
use std::mem::take;

use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::TaskVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::List;
use crate::type_::Map;

pub(crate) struct PairForm<'a, Some, First, Second> {
    pub(crate) some: &'a Map<Val, Some>,
    pub(crate) first: &'a First,
    pub(crate) second: &'a Second,
}

impl<'a, Some, First, Second> FreeFn<PairVal, Val> for PairForm<'a, Some, First, Second>
where
    Some: FreeFn<Val, Val>,
    First: FreeFn<Val, Val>,
    Second: FreeFn<Val, Val>,
{
    fn free_call(&self, mut input: PairVal) -> Val {
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.free_call(take(&mut input.second));
        } else {
            input.first = self.first.free_call(take(&mut input.first));
            input.second = self.second.free_call(take(&mut input.second));
        }
        Val::Pair(input)
    }
}

impl<'a, Some, First, Second, Ctx> ConstFn<Ctx, PairVal, Val> for PairForm<'a, Some, First, Second>
where
    Some: ConstFn<Ctx, Val, Val>,
    First: ConstFn<Ctx, Val, Val>,
    Second: ConstFn<Ctx, Val, Val>,
{
    fn const_call(&self, mut ctx: ConstRef<Ctx>, mut input: PairVal) -> Val {
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.const_call(ctx, take(&mut input.second));
        } else {
            input.first = self.first.const_call(ctx.reborrow(), take(&mut input.first));
            input.second = self.second.const_call(ctx, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

impl<'a, Some, First, Second, Ctx> MutFn<Ctx, PairVal, Val> for PairForm<'a, Some, First, Second>
where
    Some: MutFn<Ctx, Val, Val>,
    First: MutFn<Ctx, Val, Val>,
    Second: MutFn<Ctx, Val, Val>,
{
    fn mut_call(&self, ctx: &mut Ctx, mut input: PairVal) -> Val {
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.mut_call(ctx, take(&mut input.second));
        } else {
            input.first = self.first.mut_call(ctx, take(&mut input.first));
            input.second = self.second.mut_call(ctx, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

pub(crate) struct TaskForm<'a, Func, Ctx, Input> {
    pub(crate) func: &'a Func,
    pub(crate) ctx: &'a Ctx,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Ctx, Input> FreeFn<TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: FreeFn<Val, Val>,
    Ctx: FreeFn<Val, Val>,
    Input: FreeFn<Val, Val>,
{
    fn free_call(&self, mut task: TaskVal) -> Val {
        task.func = self.func.free_call(take(&mut task.func));
        task.ctx = self.ctx.free_call(take(&mut task.ctx));
        task.input = self.input.free_call(take(&mut task.input));
        Val::Task(task)
    }
}

impl<'a, Func, Ctx, Input, C> ConstFn<C, TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: ConstFn<C, Val, Val>,
    Ctx: ConstFn<C, Val, Val>,
    Input: ConstFn<C, Val, Val>,
{
    fn const_call(&self, mut ctx: ConstRef<C>, mut task: TaskVal) -> Val {
        task.func = self.func.const_call(ctx.reborrow(), take(&mut task.func));
        task.ctx = self.ctx.const_call(ctx.reborrow(), take(&mut task.ctx));
        task.input = self.input.const_call(ctx, take(&mut task.input));
        Val::Task(task)
    }
}

impl<'a, Func, Ctx, Input, C> MutFn<C, TaskVal, Val> for TaskForm<'a, Func, Ctx, Input>
where
    Func: MutFn<C, Val, Val>,
    Ctx: MutFn<C, Val, Val>,
    Input: MutFn<C, Val, Val>,
{
    fn mut_call(&self, ctx: &mut C, mut task: TaskVal) -> Val {
        task.func = self.func.mut_call(ctx, take(&mut task.func));
        task.ctx = self.ctx.mut_call(ctx, take(&mut task.ctx));
        task.input = self.input.mut_call(ctx, take(&mut task.input));
        Val::Task(task)
    }
}

pub(crate) struct ListForm<'a, Head, Tail> {
    pub(crate) head: &'a List<Head>,
    pub(crate) tail: &'a Tail,
}

impl<'a, Head, Tail> FreeFn<ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: FreeFn<Val, Val>,
    Tail: FreeFn<Val, Val>,
{
    fn free_call(&self, mut input: ListVal) -> Val {
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.free_call(take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            list.push(f.free_call(val));
        }
        for val in iter {
            list.push(self.tail.free_call(val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> ConstFn<Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: ConstFn<Ctx, Val, Val>,
    Tail: ConstFn<Ctx, Val, Val>,
{
    fn const_call(&self, mut ctx: ConstRef<Ctx>, mut input: ListVal) -> Val {
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.const_call(ctx.reborrow(), take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            list.push(f.const_call(ctx.reborrow(), val));
        }
        for val in iter {
            list.push(self.tail.const_call(ctx.reborrow(), val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> MutFn<Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: MutFn<Ctx, Val, Val>,
    Tail: MutFn<Ctx, Val, Val>,
{
    fn mut_call(&self, ctx: &mut Ctx, mut input: ListVal) -> Val {
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.mut_call(ctx, take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            list.push(f.mut_call(ctx, val));
        }
        for val in iter {
            list.push(self.tail.mut_call(ctx, val));
        }
        Val::List(List::from(list).into())
    }
}

pub(crate) struct MapForm<'a, SomeKey, SomeValue, ElseValue> {
    pub(crate) some: &'a Map<SomeKey, SomeValue>,
    pub(crate) else_: &'a ElseValue,
}

impl<'a, SomeKey, SomeValue, ElseValue> FreeFn<MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: FreeFn<Val, Val>,
    ElseValue: FreeFn<Val, Val>,
{
    fn free_call(&self, mut input: MapVal) -> Val {
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.free_call(take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.free_call(take(v));
                } else {
                    *v = self.else_.free_call(take(v));
                }
            }
        }
        Val::Map(input)
    }
}

impl<'a, SomeKey, SomeValue, ElseValue, Ctx> ConstFn<Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: ConstFn<Ctx, Val, Val>,
    ElseValue: ConstFn<Ctx, Val, Val>,
{
    fn const_call(&self, mut ctx: ConstRef<Ctx>, mut input: MapVal) -> Val {
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.const_call(ctx.reborrow(), take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.const_call(ctx.reborrow(), take(v));
                } else {
                    *v = self.else_.const_call(ctx.reborrow(), take(v));
                }
            }
        }
        Val::Map(input)
    }
}

impl<'a, SomeKey, SomeValue, ElseValue, Ctx> MutFn<Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: MutFn<Ctx, Val, Val>,
    ElseValue: MutFn<Ctx, Val, Val>,
{
    fn mut_call(&self, ctx: &mut Ctx, mut input: MapVal) -> Val {
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.mut_call(ctx, take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.mut_call(ctx, take(v));
                } else {
                    *v = self.else_.mut_call(ctx, take(v));
                }
            }
        }
        Val::Map(input)
    }
}
