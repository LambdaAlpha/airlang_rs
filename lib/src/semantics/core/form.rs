use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::func::DynFunc;
use crate::semantics::val::CellVal;
use crate::semantics::val::ListVal;
use crate::semantics::val::MapVal;
use crate::semantics::val::PairVal;
use crate::semantics::val::Val;

pub(crate) struct CellForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value, CtxVal> DynFunc<CtxVal, CellVal, CellVal> for CellForm<'a, Value>
where Value: DynFunc<CtxVal, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, ctx: Ctx<CtxVal>, mut cell: CellVal) -> CellVal {
        cell.value = self.value.call(cfg, ctx, take(&mut cell.value));
        cell
    }
}

pub(crate) struct PairForm<'a, Left, Right> {
    pub(crate) left: &'a Left,
    pub(crate) right: &'a Right,
}

impl<'a, Left, Right, CtxVal> DynFunc<CtxVal, PairVal, PairVal> for PairForm<'a, Left, Right>
where
    Left: DynFunc<CtxVal, Val, Val>,
    Right: DynFunc<CtxVal, Val, Val>,
{
    fn call(&self, cfg: &mut Cfg, mut ctx: Ctx<CtxVal>, mut pair: PairVal) -> PairVal {
        pair.left = self.left.call(cfg, ctx.reborrow(), take(&mut pair.left));
        pair.right = self.right.call(cfg, ctx, take(&mut pair.right));
        pair
    }
}

pub(crate) struct ListForm<'a, Item> {
    pub(crate) item: &'a Item,
}

impl<'a, Item, CtxVal> DynFunc<CtxVal, ListVal, ListVal> for ListForm<'a, Item>
where Item: DynFunc<CtxVal, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, mut ctx: Ctx<CtxVal>, mut list: ListVal) -> ListVal {
        for v in list.iter_mut() {
            *v = self.item.call(cfg, ctx.reborrow(), take(v));
        }
        list
    }
}

pub(crate) struct MapForm<'a, Value> {
    pub(crate) value: &'a Value,
}

impl<'a, Value, CtxVal> DynFunc<CtxVal, MapVal, MapVal> for MapForm<'a, Value>
where Value: DynFunc<CtxVal, Val, Val>
{
    fn call(&self, cfg: &mut Cfg, mut ctx: Ctx<CtxVal>, mut map: MapVal) -> MapVal {
        for v in map.values_mut() {
            *v = self.value.call(cfg, ctx.reborrow(), take(v));
        }
        map
    }
}
