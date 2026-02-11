use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::DynFunc;
use crate::semantics::val::CallVal;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;
use crate::type_::Key;

pub(crate) struct CellForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value, Ctx> DynFunc<Cfg, Ctx, CellVal, CellVal> for CellForm<'a, Value>
where Value: DynFunc<Cfg, Ctx, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut cell: CellVal) -> CellVal {
        cell.value = self.value.call(cfg, ctx, take(&mut cell.value));
        cell
    }
}

pub(crate) struct PairForm<'a, Left, Right> {
    pub(crate) left: &'a Left,
    pub(crate) right: &'a Right,
}

impl<'a, Left, Right, Ctx> DynFunc<Cfg, Ctx, PairVal, PairVal> for PairForm<'a, Left, Right>
where
    Left: DynFunc<Cfg, Ctx, Val, Val>,
    Right: DynFunc<Cfg, Ctx, Val, Val>,
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut pair: PairVal) -> PairVal {
        pair.left = self.left.call(cfg, ctx, take(&mut pair.left));
        pair.right = self.right.call(cfg, ctx, take(&mut pair.right));
        pair
    }
}

pub(crate) struct CallForm<'a, Func, Input> {
    pub(crate) func: &'a Func,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Input, Ctx> DynFunc<Cfg, Ctx, CallVal, CallVal> for CallForm<'a, Func, Input>
where
    Func: DynFunc<Cfg, Ctx, Val, Val>,
    Input: DynFunc<Cfg, Ctx, Val, Val>,
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut call: CallVal) -> CallVal {
        call.func = self.func.call(cfg, ctx, take(&mut call.func));
        call.input = self.input.call(cfg, ctx, take(&mut call.input));
        call
    }
}

pub(crate) struct ListForm<'a, Item> {
    pub(crate) item: &'a Item,
}

impl<'a, Item, Ctx> DynFunc<Cfg, Ctx, ListVal, ListVal> for ListForm<'a, Item>
where Item: DynFunc<Cfg, Ctx, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut list: ListVal) -> ListVal {
        for v in list.iter_mut() {
            *v = self.item.call(cfg, ctx, take(v));
        }
        list
    }
}

pub(crate) struct MapForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value, Ctx> DynFunc<Cfg, Ctx, MapVal, MapVal> for MapForm<'a, Value>
where Value: DynFunc<Cfg, Ctx, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut map: MapVal) -> MapVal {
        for v in map.values_mut() {
            *v = self.value.call(cfg, ctx, take(v));
        }
        map
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Form;

impl DynFunc<Cfg, Val, Val, Val> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.call(cfg, ctx, key),
            Val::Cell(cell) => Val::Cell(self.call(cfg, ctx, cell)),
            Val::Pair(pair) => Val::Pair(self.call(cfg, ctx, pair)),
            Val::Call(call) => Val::Call(self.call(cfg, ctx, call)),
            Val::List(list) => Val::List(self.call(cfg, ctx, list)),
            Val::Map(map) => Val::Map(self.call(cfg, ctx, map)),
            v => v,
        }
    }
}

impl DynFunc<Cfg, Val, Key, Val> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.call(cfg, ctx, key)
    }
}

impl DynFunc<Cfg, Val, CellVal, CellVal> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> CellVal {
        CellForm { value: self }.call(cfg, ctx, cell)
    }
}

impl DynFunc<Cfg, Val, PairVal, PairVal> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> PairVal {
        PairForm { left: self, right: self }.call(cfg, ctx, pair)
    }
}

impl DynFunc<Cfg, Val, CallVal, CallVal> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.call(cfg, ctx, call)
    }
}

impl DynFunc<Cfg, Val, ListVal, ListVal> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> ListVal {
        ListForm { item: self }.call(cfg, ctx, list)
    }
}

impl DynFunc<Cfg, Val, MapVal, MapVal> for Form {
    fn call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> MapVal {
        MapForm { value: self }.call(cfg, ctx, map)
    }
}
