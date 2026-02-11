use std::mem::take;

use crate::semantics::cfg::Cfg;
use crate::semantics::core::key::KeyEval;
use crate::semantics::func::CtxFn;
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

impl CtxFn<Cfg, Val, Key, Val> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, key: Key) -> Val {
        KeyEval.ctx_call(cfg, ctx, key)
    }
}

impl CtxFn<Cfg, Val, CellVal, CellVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, cell: CellVal) -> CellVal {
        CellForm { value: self }.ctx_call(cfg, ctx, cell)
    }
}

impl CtxFn<Cfg, Val, PairVal, PairVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, pair: PairVal) -> PairVal {
        PairForm { left: self, right: self }.ctx_call(cfg, ctx, pair)
    }
}

impl CtxFn<Cfg, Val, CallVal, CallVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, call: CallVal) -> CallVal {
        CallForm { func: self, input: self }.ctx_call(cfg, ctx, call)
    }
}

impl CtxFn<Cfg, Val, ListVal, ListVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, list: ListVal) -> ListVal {
        ListForm { item: self }.ctx_call(cfg, ctx, list)
    }
}

impl CtxFn<Cfg, Val, MapVal, MapVal> for Form {
    fn ctx_call(&self, cfg: &mut Cfg, ctx: &mut Val, map: MapVal) -> MapVal {
        MapForm { value: self }.ctx_call(cfg, ctx, map)
    }
}
