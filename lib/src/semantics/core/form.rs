use std::borrow::Borrow;
use std::hash::Hash;
use std::mem::take;

use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
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
