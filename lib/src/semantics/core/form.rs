use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::id::Id;
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
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Key;

pub(crate) struct CellForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value> FreeFn<Cfg, CellVal, CellVal> for CellForm<'a, Value>
where Value: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, mut cell: CellVal) -> CellVal {
        cfg.step();
        cell.value = self.value.free_call(cfg, take(&mut cell.value));
        cell
    }
}

impl<'a, Value, Ctx> ConstFn<Cfg, Ctx, CellVal, CellVal> for CellForm<'a, Value>
where Value: ConstFn<Cfg, Ctx, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Ctx>, mut cell: CellVal) -> CellVal {
        cfg.step();
        cell.value = self.value.const_call(cfg, ctx, take(&mut cell.value));
        cell
    }
}

impl<'a, Value, Ctx> MutFn<Cfg, Ctx, CellVal, CellVal> for CellForm<'a, Value>
where Value: MutFn<Cfg, Ctx, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut cell: CellVal) -> CellVal {
        cfg.step();
        cell.value = self.value.mut_call(cfg, ctx, take(&mut cell.value));
        cell
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Ctx>, mut cell: CellVal) -> CellVal {
        cfg.step();
        cell.value = self.value.dyn_call(cfg, ctx, take(&mut cell.value));
        cell
    }
}

pub(crate) struct PairForm<'a, First, Second> {
    pub(crate) first: &'a First,
    pub(crate) second: &'a Second,
}

impl<'a, First, Second> FreeFn<Cfg, PairVal, PairVal> for PairForm<'a, First, Second>
where
    First: FreeFn<Cfg, Val, Val>,
    Second: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut pair: PairVal) -> PairVal {
        cfg.step();
        pair.first = self.first.free_call(cfg, take(&mut pair.first));
        pair.second = self.second.free_call(cfg, take(&mut pair.second));
        pair
    }
}

impl<'a, First, Second, Ctx> ConstFn<Cfg, Ctx, PairVal, PairVal> for PairForm<'a, First, Second>
where
    First: ConstFn<Cfg, Ctx, Val, Val>,
    Second: ConstFn<Cfg, Ctx, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut pair: PairVal) -> PairVal {
        cfg.step();
        pair.first = self.first.const_call(cfg, ctx.reborrow(), take(&mut pair.first));
        pair.second = self.second.const_call(cfg, ctx, take(&mut pair.second));
        pair
    }
}

impl<'a, First, Second, Ctx> MutFn<Cfg, Ctx, PairVal, PairVal> for PairForm<'a, First, Second>
where
    First: MutFn<Cfg, Ctx, Val, Val>,
    Second: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut pair: PairVal) -> PairVal {
        cfg.step();
        pair.first = self.first.mut_call(cfg, ctx, take(&mut pair.first));
        pair.second = self.second.mut_call(cfg, ctx, take(&mut pair.second));
        pair
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut pair: PairVal) -> PairVal {
        cfg.step();
        pair.first = self.first.dyn_call(cfg, ctx.reborrow(), take(&mut pair.first));
        pair.second = self.second.dyn_call(cfg, ctx, take(&mut pair.second));
        pair
    }
}

pub(crate) struct CallForm<'a, Func, Input> {
    pub(crate) func: &'a Func,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Input> FreeFn<Cfg, CallVal, CallVal> for CallForm<'a, Func, Input>
where
    Func: FreeFn<Cfg, Val, Val>,
    Input: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut call: CallVal) -> CallVal {
        cfg.step();
        call.func = self.func.free_call(cfg, take(&mut call.func));
        call.input = self.input.free_call(cfg, take(&mut call.input));
        call
    }
}

impl<'a, Func, Input, C> ConstFn<Cfg, C, CallVal, CallVal> for CallForm<'a, Func, Input>
where
    Func: ConstFn<Cfg, C, Val, Val>,
    Input: ConstFn<Cfg, C, Val, Val>,
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<C>, mut call: CallVal) -> CallVal {
        cfg.step();
        call.func = self.func.const_call(cfg, ctx.reborrow(), take(&mut call.func));
        call.input = self.input.const_call(cfg, ctx, take(&mut call.input));
        call
    }
}

impl<'a, Func, Input, Ctx> MutFn<Cfg, Ctx, CallVal, CallVal> for CallForm<'a, Func, Input>
where
    Func: MutFn<Cfg, Ctx, Val, Val>,
    Input: MutFn<Cfg, Ctx, Val, Val>,
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut call: CallVal) -> CallVal {
        cfg.step();
        call.func = self.func.mut_call(cfg, ctx, take(&mut call.func));
        call.input = self.input.mut_call(cfg, ctx, take(&mut call.input));
        call
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut call: CallVal) -> CallVal {
        cfg.step();
        call.func = self.func.dyn_call(cfg, ctx.reborrow(), take(&mut call.func));
        call.input = self.input.dyn_call(cfg, ctx, take(&mut call.input));
        call
    }
}

pub(crate) struct ListForm<'a, Item> {
    pub(crate) item: &'a Item,
}

impl<'a, Item> FreeFn<Cfg, ListVal, ListVal> for ListForm<'a, Item>
where Item: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, mut list: ListVal) -> ListVal {
        cfg.step();
        for v in list.iter_mut() {
            *v = self.item.free_call(cfg, take(v));
        }
        list
    }
}

impl<'a, Item, Ctx> ConstFn<Cfg, Ctx, ListVal, ListVal> for ListForm<'a, Item>
where Item: ConstFn<Cfg, Ctx, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut list: ListVal) -> ListVal {
        cfg.step();
        for v in list.iter_mut() {
            *v = self.item.const_call(cfg, ctx.reborrow(), take(v));
        }
        list
    }
}

impl<'a, Item, Ctx> MutFn<Cfg, Ctx, ListVal, ListVal> for ListForm<'a, Item>
where Item: MutFn<Cfg, Ctx, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut list: ListVal) -> ListVal {
        cfg.step();
        for v in list.iter_mut() {
            *v = self.item.mut_call(cfg, ctx, take(v));
        }
        list
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut list: ListVal) -> ListVal {
        cfg.step();
        for v in list.iter_mut() {
            *v = self.item.dyn_call(cfg, ctx.reborrow(), take(v));
        }
        list
    }
}

pub(crate) struct MapForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value> FreeFn<Cfg, MapVal, MapVal> for MapForm<'a, Value>
where Value: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, mut map: MapVal) -> MapVal {
        cfg.step();
        for v in map.values_mut() {
            *v = self.value.free_call(cfg, take(v));
        }
        map
    }
}

impl<'a, Value, Ctx> ConstFn<Cfg, Ctx, MapVal, MapVal> for MapForm<'a, Value>
where Value: ConstFn<Cfg, Ctx, Val, Val>
{
    fn const_call(&self, cfg: &mut Cfg, mut ctx: ConstRef<Ctx>, mut map: MapVal) -> MapVal {
        cfg.step();
        for v in map.values_mut() {
            *v = self.value.const_call(cfg, ctx.reborrow(), take(v));
        }
        map
    }
}

impl<'a, Value, Ctx> MutFn<Cfg, Ctx, MapVal, MapVal> for MapForm<'a, Value>
where Value: MutFn<Cfg, Ctx, Val, Val>
{
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut map: MapVal) -> MapVal {
        cfg.step();
        for v in map.values_mut() {
            *v = self.value.mut_call(cfg, ctx, take(v));
        }
        map
    }

    fn dyn_call(&self, cfg: &mut Cfg, mut ctx: DynRef<Ctx>, mut map: MapVal) -> MapVal {
        cfg.step();
        for v in map.values_mut() {
            *v = self.value.dyn_call(cfg, ctx.reborrow(), take(v));
        }
        map
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) struct Form;

impl FreeFn<Cfg, Val, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, val: Val) -> Val {
        match val {
            Val::Key(key) => self.free_call(cfg, key),
            Val::Cell(cell) => Val::Cell(self.free_call(cfg, cell)),
            Val::Pair(pair) => Val::Pair(self.free_call(cfg, pair)),
            Val::Call(call) => Val::Call(self.free_call(cfg, call)),
            Val::List(list) => Val::List(self.free_call(cfg, list)),
            Val::Map(map) => Val::Map(self.free_call(cfg, map)),
            v => Id.free_call(cfg, v),
        }
    }
}

impl ConstFn<Cfg, Val, Val, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, val: Val) -> Val {
        match val {
            Val::Key(key) => self.const_call(cfg, ctx, key),
            Val::Cell(cell) => Val::Cell(self.const_call(cfg, ctx, cell)),
            Val::Pair(pair) => Val::Pair(self.const_call(cfg, ctx, pair)),
            Val::Call(call) => Val::Call(self.const_call(cfg, ctx, call)),
            Val::List(list) => Val::List(self.const_call(cfg, ctx, list)),
            Val::Map(map) => Val::Map(self.const_call(cfg, ctx, map)),
            v => Id.const_call(cfg, ctx, v),
        }
    }
}

impl MutFn<Cfg, Val, Val, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, val: Val) -> Val {
        match val {
            Val::Key(key) => self.mut_call(cfg, ctx, key),
            Val::Cell(cell) => Val::Cell(self.mut_call(cfg, ctx, cell)),
            Val::Pair(pair) => Val::Pair(self.mut_call(cfg, ctx, pair)),
            Val::Call(call) => Val::Call(self.mut_call(cfg, ctx, call)),
            Val::List(list) => Val::List(self.mut_call(cfg, ctx, list)),
            Val::Map(map) => Val::Map(self.mut_call(cfg, ctx, map)),
            v => Id.mut_call(cfg, ctx, v),
        }
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, val: Val) -> Val {
        match val {
            Val::Key(key) => self.dyn_call(cfg, ctx, key),
            Val::Cell(cell) => Val::Cell(self.dyn_call(cfg, ctx, cell)),
            Val::Pair(pair) => Val::Pair(self.dyn_call(cfg, ctx, pair)),
            Val::Call(call) => Val::Call(self.dyn_call(cfg, ctx, call)),
            Val::List(list) => Val::List(self.dyn_call(cfg, ctx, list)),
            Val::Map(map) => Val::Map(self.dyn_call(cfg, ctx, map)),
            v => Id.dyn_call(cfg, ctx, v),
        }
    }
}

impl FreeFn<Cfg, Key, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, key: Key) -> Val {
        KeyEval.free_call(cfg, key)
    }
}

impl ConstFn<Cfg, Val, Key, Val> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, key: Key) -> Val {
        KeyEval.const_call(cfg, ctx, key)
    }
}

impl MutFn<Cfg, Val, Key, Val> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.mut_call(cfg, ctx, key)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, key: Key) -> Val {
        KeyEval.dyn_call(cfg, ctx, key)
    }
}

impl FreeFn<Cfg, CellVal, CellVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, cell: CellVal) -> CellVal {
        CellForm { value: self }.free_call(cfg, cell)
    }
}

impl ConstFn<Cfg, Val, CellVal, CellVal> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, cell: CellVal) -> CellVal {
        CellForm { value: self }.const_call(cfg, ctx, cell)
    }
}

impl MutFn<Cfg, Val, CellVal, CellVal> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> CellVal {
        CellForm { value: self }.mut_call(cfg, ctx, cell)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, cell: CellVal) -> CellVal {
        CellForm { value: self }.dyn_call(cfg, ctx, cell)
    }
}

impl FreeFn<Cfg, PairVal, PairVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, pair: PairVal) -> PairVal {
        PairForm { first: self, second: self }.free_call(cfg, pair)
    }
}

impl ConstFn<Cfg, Val, PairVal, PairVal> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, pair: PairVal) -> PairVal {
        PairForm { first: self, second: self }.const_call(cfg, ctx, pair)
    }
}

impl MutFn<Cfg, Val, PairVal, PairVal> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> PairVal {
        PairForm { first: self, second: self }.mut_call(cfg, ctx, pair)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, pair: PairVal) -> PairVal {
        PairForm { first: self, second: self }.dyn_call(cfg, ctx, pair)
    }
}

impl FreeFn<Cfg, CallVal, CallVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.free_call(cfg, call)
    }
}

impl ConstFn<Cfg, Val, CallVal, CallVal> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.const_call(cfg, ctx, call)
    }
}

impl MutFn<Cfg, Val, CallVal, CallVal> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.mut_call(cfg, ctx, call)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.dyn_call(cfg, ctx, call)
    }
}

impl FreeFn<Cfg, ListVal, ListVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, list: ListVal) -> ListVal {
        ListForm { item: self }.free_call(cfg, list)
    }
}

impl ConstFn<Cfg, Val, ListVal, ListVal> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, list: ListVal) -> ListVal {
        ListForm { item: self }.const_call(cfg, ctx, list)
    }
}

impl MutFn<Cfg, Val, ListVal, ListVal> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> ListVal {
        ListForm { item: self }.mut_call(cfg, ctx, list)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, list: ListVal) -> ListVal {
        ListForm { item: self }.dyn_call(cfg, ctx, list)
    }
}

impl FreeFn<Cfg, MapVal, MapVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, map: MapVal) -> MapVal {
        MapForm { value: self }.free_call(cfg, map)
    }
}

impl ConstFn<Cfg, Val, MapVal, MapVal> for Form {
    fn const_call(&self, cfg: &mut Cfg, ctx: ConstRef<Val>, map: MapVal) -> MapVal {
        MapForm { value: self }.const_call(cfg, ctx, map)
    }
}

impl MutFn<Cfg, Val, MapVal, MapVal> for Form {
    fn mut_call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> MapVal {
        MapForm { value: self }.mut_call(cfg, ctx, map)
    }

    fn dyn_call(&self, cfg: &mut Cfg, ctx: DynRef<Val>, map: MapVal) -> MapVal {
        MapForm { value: self }.dyn_call(cfg, ctx, map)
    }
}
