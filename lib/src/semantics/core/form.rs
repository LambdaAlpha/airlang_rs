use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::id::Id;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::CallVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Key;

pub(crate) struct PairForm<'a, First, Second> {
    pub(crate) first: &'a First,
    pub(crate) second: &'a Second,
}

impl<'a, First, Second> FreeFn<Cfg, PairVal, Val> for PairForm<'a, First, Second>
where
    First: FreeFn<Cfg, Val, Val>,
    Second: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut input: PairVal) -> Val {
        cfg.step();
        input.first = self.first.free_call(cfg, take(&mut input.first));
        input.second = self.second.free_call(cfg, take(&mut input.second));
        Val::Pair(input)
    }
}

impl<'a, First, Second, Ctx> ConstFn<Cfg, Ctx, PairVal, Val> for PairForm<'a, First, Second>
where
    First: ConstFn<Cfg, Ctx, Val, Val>,
    Second: ConstFn<Cfg, Ctx, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut input: PairVal) -> Val {
        cfg.step();
        input.first = self.first.const_call(cfg, ctx.reborrow(), take(&mut input.first));
        input.second = self.second.const_call(cfg, ctx, take(&mut input.second));
        Val::Pair(input)
    }
}

impl<'a, First, Second, Ctx> MutFn<Cfg, Ctx, PairVal, Val> for PairForm<'a, First, Second>
where
    First: MutFn<Cfg, Ctx, Val, Val>,
    Second: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut input: PairVal) -> Val {
        cfg.step();
        input.first = self.first.mut_call(cfg, ctx, take(&mut input.first));
        input.second = self.second.mut_call(cfg, ctx, take(&mut input.second));
        Val::Pair(input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut input: PairVal) -> Val {
        cfg.step();
        input.first = self.first.dyn_call(cfg, ctx.reborrow(), take(&mut input.first));
        input.second = self.second.dyn_call(cfg, ctx, take(&mut input.second));
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

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut call: CallVal) -> Val {
        cfg.step();
        call.func = self.func.dyn_call(cfg, ctx.reborrow(), take(&mut call.func));
        call.input = self.input.dyn_call(cfg, ctx, take(&mut call.input));
        Val::Call(call)
    }
}

pub(crate) struct ListForm<'a, Item> {
    pub(crate) item: &'a Item,
}

impl<'a, Item> FreeFn<Cfg, ListVal, Val> for ListForm<'a, Item>
where Item: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, mut input: ListVal) -> Val {
        cfg.step();
        for v in input.iter_mut() {
            *v = self.item.free_call(cfg, take(v));
        }
        Val::List(input)
    }
}

impl<'a, Item, Ctx> ConstFn<Cfg, Ctx, ListVal, Val> for ListForm<'a, Item>
where Item: ConstFn<Cfg, Ctx, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut input: ListVal) -> Val {
        cfg.step();
        for v in input.iter_mut() {
            *v = self.item.const_call(cfg, ctx.reborrow(), take(v));
        }
        Val::List(input)
    }
}

impl<'a, Item, Ctx> MutFn<Cfg, Ctx, ListVal, Val> for ListForm<'a, Item>
where Item: MutFn<Cfg, Ctx, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut input: ListVal) -> Val {
        cfg.step();
        for v in input.iter_mut() {
            *v = self.item.mut_call(cfg, ctx, take(v));
        }
        Val::List(input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut input: ListVal) -> Val {
        cfg.step();
        for v in input.iter_mut() {
            *v = self.item.dyn_call(cfg, ctx.reborrow(), take(v));
        }
        Val::List(input)
    }
}

pub(crate) struct MapForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value> FreeFn<Cfg, MapVal, Val> for MapForm<'a, Value>
where Value: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, mut input: MapVal) -> Val {
        cfg.step();
        for v in input.values_mut() {
            *v = self.value.free_call(cfg, take(v));
        }
        Val::Map(input)
    }
}

impl<'a, Value, Ctx> ConstFn<Cfg, Ctx, MapVal, Val> for MapForm<'a, Value>
where Value: ConstFn<Cfg, Ctx, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut input: MapVal) -> Val {
        cfg.step();
        for v in input.values_mut() {
            *v = self.value.const_call(cfg, ctx.reborrow(), take(v));
        }
        Val::Map(input)
    }
}

impl<'a, Value, Ctx> MutFn<Cfg, Ctx, MapVal, Val> for MapForm<'a, Value>
where Value: MutFn<Cfg, Ctx, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut input: MapVal) -> Val {
        cfg.step();
        for v in input.values_mut() {
            *v = self.value.mut_call(cfg, ctx, take(v));
        }
        Val::Map(input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut input: MapVal) -> Val {
        cfg.step();
        for v in input.values_mut() {
            *v = self.value.dyn_call(cfg, ctx.reborrow(), take(v));
        }
        Val::Map(input)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Form;

impl FreeFn<Cfg, Val, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: Val) -> Val {
        match input {
            Val::Key(key) => self.free_call(cfg, key),
            Val::Pair(pair) => self.free_call(cfg, pair),
            Val::Call(call) => self.free_call(cfg, call),
            Val::List(list) => self.free_call(cfg, list),
            Val::Map(map) => self.free_call(cfg, map),
            v => Id.free_call(cfg, v),
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Key(key) => self.const_call(cfg, ctx, key),
            Val::Pair(pair) => self.const_call(cfg, ctx, pair),
            Val::Call(call) => self.const_call(cfg, ctx, call),
            Val::List(list) => self.const_call(cfg, ctx, list),
            Val::Map(map) => self.const_call(cfg, ctx, map),
            v => Id.const_call(cfg, ctx, v),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Key(key) => self.mut_call(cfg, ctx, key),
            Val::Pair(pair) => self.mut_call(cfg, ctx, pair),
            Val::Call(call) => self.mut_call(cfg, ctx, call),
            Val::List(list) => self.mut_call(cfg, ctx, list),
            Val::Map(map) => self.mut_call(cfg, ctx, map),
            v => Id.mut_call(cfg, ctx, v),
        }
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
        match input {
            Val::Key(key) => self.dyn_call(cfg, ctx, key),
            Val::Pair(pair) => self.dyn_call(cfg, ctx, pair),
            Val::Call(call) => self.dyn_call(cfg, ctx, call),
            Val::List(list) => self.dyn_call(cfg, ctx, list),
            Val::Map(map) => self.dyn_call(cfg, ctx, map),
            v => Id.dyn_call(cfg, ctx, v),
        }
    }
}

impl FreeFn<Cfg, Key, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: Key) -> Val {
        KeyEval.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, Key, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: Key) -> Val {
        KeyEval.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, Key, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: Key) -> Val {
        KeyEval.mut_call(cfg, ctx, input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: Key) -> Val {
        KeyEval.dyn_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, PairVal, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: PairVal) -> Val {
        PairForm { first: self, second: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, PairVal, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: PairVal) -> Val {
        PairForm { first: self, second: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, PairVal, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: PairVal) -> Val {
        PairForm { first: self, second: self }.mut_call(cfg, ctx, input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: PairVal) -> Val {
        PairForm { first: self, second: self }.dyn_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, CallVal, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: CallVal) -> Val {
        CallForm { func: self, input: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, CallVal, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: CallVal) -> Val {
        CallForm { func: self, input: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, CallVal, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: CallVal) -> Val {
        CallForm { func: self, input: self }.mut_call(cfg, ctx, input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: CallVal) -> Val {
        CallForm { func: self, input: self }.dyn_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, ListVal, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: ListVal) -> Val {
        ListForm { item: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, ListVal, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: ListVal) -> Val {
        ListForm { item: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, ListVal, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: ListVal) -> Val {
        ListForm { item: self }.mut_call(cfg, ctx, input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: ListVal) -> Val {
        ListForm { item: self }.dyn_call(cfg, ctx, input)
    }
}

impl FreeFn<Cfg, MapVal, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, input: MapVal) -> Val {
        MapForm { value: self }.free_call(cfg, input)
    }
}

impl ConstFn<Cfg, Val, MapVal, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, input: MapVal) -> Val {
        MapForm { value: self }.const_call(cfg, ctx, input)
    }
}

impl MutFn<Cfg, Val, MapVal, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, input: MapVal) -> Val {
        MapForm { value: self }.mut_call(cfg, ctx, input)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, input: MapVal) -> Val {
        MapForm { value: self }.dyn_call(cfg, ctx, input)
    }
}
