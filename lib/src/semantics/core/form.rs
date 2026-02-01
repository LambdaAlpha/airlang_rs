use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::CtxFn;
use crate::semantics::func::FreeFn;
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

impl<'a, Value> FreeFn<Cfg, CellVal, CellVal> for CellForm<'a, Value>
where Value: FreeFn<Cfg, Val, Val>
{
    fn free_call(&self, cfg: &mut Cfg, mut cell: CellVal) -> CellVal {
        cell.value = self.value.free_call(cfg, take(&mut cell.value));
        cell
    }
}

impl<'a, Value, Ctx> CtxFn<Cfg, Ctx, CellVal, CellVal> for CellForm<'a, Value>
where Value: CtxFn<Cfg, Ctx, Val, Val>
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut cell: CellVal) -> CellVal {
        cell.value = self.value.ctx_call(cfg, ctx, take(&mut cell.value));
        cell
    }
}

pub(crate) struct PairForm<'a, Left, Right> {
    pub(crate) left: &'a Left,
    pub(crate) right: &'a Right,
}

impl<'a, Left, Right> FreeFn<Cfg, PairVal, PairVal> for PairForm<'a, Left, Right>
where
    Left: FreeFn<Cfg, Val, Val>,
    Right: FreeFn<Cfg, Val, Val>,
{
    fn free_call(&self, cfg: &mut Cfg, mut pair: PairVal) -> PairVal {
        pair.left = self.left.free_call(cfg, take(&mut pair.left));
        pair.right = self.right.free_call(cfg, take(&mut pair.right));
        pair
    }
}

impl<'a, Left, Right, Ctx> CtxFn<Cfg, Ctx, PairVal, PairVal> for PairForm<'a, Left, Right>
where
    Left: CtxFn<Cfg, Ctx, Val, Val>,
    Right: CtxFn<Cfg, Ctx, Val, Val>,
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut pair: PairVal) -> PairVal {
        pair.left = self.left.ctx_call(cfg, ctx, take(&mut pair.left));
        pair.right = self.right.ctx_call(cfg, ctx, take(&mut pair.right));
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
        call.func = self.func.free_call(cfg, take(&mut call.func));
        call.input = self.input.free_call(cfg, take(&mut call.input));
        call
    }
}

impl<'a, Func, Input, Ctx> CtxFn<Cfg, Ctx, CallVal, CallVal> for CallForm<'a, Func, Input>
where
    Func: CtxFn<Cfg, Ctx, Val, Val>,
    Input: CtxFn<Cfg, Ctx, Val, Val>,
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut call: CallVal) -> CallVal {
        call.func = self.func.ctx_call(cfg, ctx, take(&mut call.func));
        call.input = self.input.ctx_call(cfg, ctx, take(&mut call.input));
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
        for v in list.iter_mut() {
            *v = self.item.free_call(cfg, take(v));
        }
        list
    }
}

impl<'a, Item, Ctx> CtxFn<Cfg, Ctx, ListVal, ListVal> for ListForm<'a, Item>
where Item: CtxFn<Cfg, Ctx, Val, Val>
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut list: ListVal) -> ListVal {
        for v in list.iter_mut() {
            *v = self.item.ctx_call(cfg, ctx, take(v));
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
        for v in map.values_mut() {
            *v = self.value.free_call(cfg, take(v));
        }
        map
    }
}

impl<'a, Value, Ctx> CtxFn<Cfg, Ctx, MapVal, MapVal> for MapForm<'a, Value>
where Value: CtxFn<Cfg, Ctx, Val, Val>
{
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Ctx, mut map: MapVal) -> MapVal {
        for v in map.values_mut() {
            *v = self.value.ctx_call(cfg, ctx, take(v));
        }
        map
    }
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Form;

impl FreeFn<Cfg, Val, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.free_call(cfg, key),
            Val::Cell(cell) => Val::Cell(self.free_call(cfg, cell)),
            Val::Pair(pair) => Val::Pair(self.free_call(cfg, pair)),
            Val::Call(call) => Val::Call(self.free_call(cfg, call)),
            Val::List(list) => Val::List(self.free_call(cfg, list)),
            Val::Map(map) => Val::Map(self.free_call(cfg, map)),
            v => v,
        }
    }
}

impl CtxFn<Cfg, Val, Val, Val> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, val: Val) -> Val {
        if !cfg.step() {
            return Val::default();
        }
        match val {
            Val::Key(key) => self.ctx_call(cfg, ctx, key),
            Val::Cell(cell) => Val::Cell(self.ctx_call(cfg, ctx, cell)),
            Val::Pair(pair) => Val::Pair(self.ctx_call(cfg, ctx, pair)),
            Val::Call(call) => Val::Call(self.ctx_call(cfg, ctx, call)),
            Val::List(list) => Val::List(self.ctx_call(cfg, ctx, list)),
            Val::Map(map) => Val::Map(self.ctx_call(cfg, ctx, map)),
            v => v,
        }
    }
}

impl FreeFn<Cfg, Key, Val> for Form {
    fn free_call(&self, cfg: &mut Cfg, key: Key) -> Val {
        KeyEval.free_call(cfg, key)
    }
}

impl CtxFn<Cfg, Val, Key, Val> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.ctx_call(cfg, ctx, key)
    }
}

impl FreeFn<Cfg, CellVal, CellVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, cell: CellVal) -> CellVal {
        CellForm { value: self }.free_call(cfg, cell)
    }
}

impl CtxFn<Cfg, Val, CellVal, CellVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> CellVal {
        CellForm { value: self }.ctx_call(cfg, ctx, cell)
    }
}

impl FreeFn<Cfg, PairVal, PairVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, pair: PairVal) -> PairVal {
        PairForm { left: self, right: self }.free_call(cfg, pair)
    }
}

impl CtxFn<Cfg, Val, PairVal, PairVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> PairVal {
        PairForm { left: self, right: self }.ctx_call(cfg, ctx, pair)
    }
}

impl FreeFn<Cfg, CallVal, CallVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.free_call(cfg, call)
    }
}

impl CtxFn<Cfg, Val, CallVal, CallVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.ctx_call(cfg, ctx, call)
    }
}

impl FreeFn<Cfg, ListVal, ListVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, list: ListVal) -> ListVal {
        ListForm { item: self }.free_call(cfg, list)
    }
}

impl CtxFn<Cfg, Val, ListVal, ListVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> ListVal {
        ListForm { item: self }.ctx_call(cfg, ctx, list)
    }
}

impl FreeFn<Cfg, MapVal, MapVal> for Form {
    fn free_call(&self, cfg: &mut Cfg, map: MapVal) -> MapVal {
        MapForm { value: self }.free_call(cfg, map)
    }
}

impl CtxFn<Cfg, Val, MapVal, MapVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> MapVal {
        MapForm { value: self }.ctx_call(cfg, ctx, map)
    }
}
