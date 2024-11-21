use crate::{
    AdaptVal,
    AskVal,
    CallVal,
    PairVal,
    ctx::ref1::CtxMeta,
    symbol::Symbol,
    transformer::{
        ByVal,
        Transformer,
    },
    val::{
        Val,
        list::ListVal,
        map::MapVal,
    },
};

#[derive(Copy, Clone)]
pub(crate) struct Id;

impl Transformer<Val, Val> for Id {
    fn transform<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        input
    }
}

impl ByVal<Val> for Id {
    fn transform_default<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        input
    }

    fn transform_symbol<'a, Ctx>(&self, _ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::Symbol(symbol)
    }

    fn transform_pair<'a, Ctx>(&self, _ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::Pair(pair)
    }

    fn transform_adapt<'a, Ctx>(&self, _ctx: Ctx, adapt: AdaptVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::Adapt(adapt)
    }

    fn transform_call<'a, Ctx>(&self, _ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::Call(call)
    }

    fn transform_ask<'a, Ctx>(&self, _ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::Ask(ask)
    }

    fn transform_list<'a, Ctx>(&self, _ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::List(list)
    }

    fn transform_map<'a, Ctx>(&self, _ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Val::Map(map)
    }
}
