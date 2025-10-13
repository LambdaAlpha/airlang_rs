use std::borrow::Borrow;
use std::hash::Hash;
use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::List;
use crate::type_::Map;

pub(crate) struct Form;

impl FreeFn<Cfg, Val, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        cfg.step();
        input
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, _ctx: ConstRef<Val>, input: Val) -> Val {
        cfg.step();
        input
    }
}

impl MutFn<Cfg, Val, Val, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, _ctx: &mut Val, input: Val) -> Val {
        cfg.step();
        input
    }
}

pub(crate) struct PairForm<'a, Some, First, Second> {
    pub(crate) some: &'a Map<Val, Some>,
    pub(crate) first: &'a First,
    pub(crate) second: &'a Second,
}

impl<'a, Some, First, Second> FreeFn<Cfg, PairVal, Val> for PairForm<'a, Some, First, Second>
where
    Some: FreeFn<Cfg, Val, Val>,
    First: FreeFn<Cfg, Val, Val>,
    Second: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut input: PairVal) -> Val {
        cfg.step();
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.free_call(cfg, take(&mut input.second));
        } else {
            input.first = self.first.free_call(cfg, take(&mut input.first));
            input.second = self.second.free_call(cfg, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

impl<'a, Some, First, Second, Ctx> ConstFn<Cfg, Ctx, PairVal, Val>
    for PairForm<'a, Some, First, Second>
where
    Some: ConstFn<Cfg, Ctx, Val, Val>,
    First: ConstFn<Cfg, Ctx, Val, Val>,
    Second: ConstFn<Cfg, Ctx, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut input: PairVal) -> Val {
        cfg.step();
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.const_call(cfg, ctx, take(&mut input.second));
        } else {
            input.first = self.first.const_call(cfg, ctx.reborrow(), take(&mut input.first));
            input.second = self.second.const_call(cfg, ctx, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

impl<'a, Some, First, Second, Ctx> MutFn<Cfg, Ctx, PairVal, Val>
    for PairForm<'a, Some, First, Second>
where
    Some: MutFn<Cfg, Ctx, Val, Val>,
    First: MutFn<Cfg, Ctx, Val, Val>,
    Second: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut input: PairVal) -> Val {
        cfg.step();
        if let Some(second) = self.some.get(&input.first) {
            input.second = second.mut_call(cfg, ctx, take(&mut input.second));
        } else {
            input.first = self.first.mut_call(cfg, ctx, take(&mut input.first));
            input.second = self.second.mut_call(cfg, ctx, take(&mut input.second));
        }
        Val::Pair(input)
    }
}

pub(crate) struct CallForm<'a, Func, Input> {
    pub(crate) func: &'a Func,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Input> FreeFn<Cfg, CallVal, Val> for CallForm<'a, Func, Input>
where
    Func: FreeFn<Cfg, Val, Val>,
    Input: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut call: CallVal) -> Val {
        cfg.step();
        call.func = self.func.free_call(cfg, take(&mut call.func));
        call.input = self.input.free_call(cfg, take(&mut call.input));
        Val::Call(call)
    }
}

impl<'a, Func, Input, C> ConstFn<Cfg, C, CallVal, Val> for CallForm<'a, Func, Input>
where
    Func: ConstFn<Cfg, C, Val, Val>,
    Input: ConstFn<Cfg, C, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<C>, mut call: CallVal) -> Val {
        cfg.step();
        call.func = self.func.const_call(cfg, ctx.reborrow(), take(&mut call.func));
        call.input = self.input.const_call(cfg, ctx, take(&mut call.input));
        Val::Call(call)
    }
}

impl<'a, Func, Input, Ctx> MutFn<Cfg, Ctx, CallVal, Val> for CallForm<'a, Func, Input>
where
    Func: MutFn<Cfg, Ctx, Val, Val>,
    Input: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut call: CallVal) -> Val {
        cfg.step();
        call.func = self.func.mut_call(cfg, ctx, take(&mut call.func));
        call.input = self.input.mut_call(cfg, ctx, take(&mut call.input));
        Val::Call(call)
    }
}

pub(crate) struct ListForm<'a, Head, Tail> {
    pub(crate) head: &'a List<Head>,
    pub(crate) tail: &'a Tail,
}

impl<'a, Head, Tail> FreeFn<Cfg, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: FreeFn<Cfg, Val, Val>,
    Tail: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut input: ListVal) -> Val {
        cfg.step();
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.free_call(cfg, take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            list.push(f.free_call(cfg, val));
        }
        for val in iter {
            list.push(self.tail.free_call(cfg, val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> ConstFn<Cfg, Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: ConstFn<Cfg, Ctx, Val, Val>,
    Tail: ConstFn<Cfg, Ctx, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut input: ListVal) -> Val {
        cfg.step();
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.const_call(cfg, ctx.reborrow(), take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            list.push(f.const_call(cfg, ctx.reborrow(), val));
        }
        for val in iter {
            list.push(self.tail.const_call(cfg, ctx.reborrow(), val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> MutFn<Cfg, Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: MutFn<Cfg, Ctx, Val, Val>,
    Tail: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut input: ListVal) -> Val {
        cfg.step();
        if self.head.is_empty() {
            for v in input.iter_mut() {
                *v = self.tail.mut_call(cfg, ctx, take(v));
            }
            return Val::List(input);
        }
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            list.push(f.mut_call(cfg, ctx, val));
        }
        for val in iter {
            list.push(self.tail.mut_call(cfg, ctx, val));
        }
        Val::List(List::from(list).into())
    }
}

pub(crate) struct MapForm<'a, SomeKey, SomeValue, ElseValue> {
    pub(crate) some: &'a Map<SomeKey, SomeValue>,
    pub(crate) else_: &'a ElseValue,
}

impl<'a, SomeKey, SomeValue, ElseValue> FreeFn<Cfg, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: FreeFn<Cfg, Val, Val>,
    ElseValue: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut input: MapVal) -> Val {
        cfg.step();
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.free_call(cfg, take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.free_call(cfg, take(v));
                } else {
                    *v = self.else_.free_call(cfg, take(v));
                }
            }
        }
        Val::Map(input)
    }
}

impl<'a, SomeKey, SomeValue, ElseValue, Ctx> ConstFn<Cfg, Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: ConstFn<Cfg, Ctx, Val, Val>,
    ElseValue: ConstFn<Cfg, Ctx, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut input: MapVal) -> Val {
        cfg.step();
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.const_call(cfg, ctx.reborrow(), take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.const_call(cfg, ctx.reborrow(), take(v));
                } else {
                    *v = self.else_.const_call(cfg, ctx.reborrow(), take(v));
                }
            }
        }
        Val::Map(input)
    }
}

impl<'a, SomeKey, SomeValue, ElseValue, Ctx> MutFn<Cfg, Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: MutFn<Cfg, Ctx, Val, Val>,
    ElseValue: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut input: MapVal) -> Val {
        cfg.step();
        if self.some.is_empty() {
            for v in input.values_mut() {
                *v = self.else_.mut_call(cfg, ctx, take(v));
            }
        } else {
            for (k, v) in input.iter_mut() {
                if let Some(f) = self.some.get(k) {
                    *v = f.mut_call(cfg, ctx, take(v));
                } else {
                    *v = self.else_.mut_call(cfg, ctx, take(v));
                }
            }
        }
        Val::Map(input)
    }
}
