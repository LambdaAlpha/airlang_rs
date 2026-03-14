use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::func::DynFunc;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;

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
