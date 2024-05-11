use crate::{
    ctx_access::CtxAccessor,
    symbol::Symbol,
    transformer::{
        input::ByVal,
        DefaultByVal,
        Transformer,
    },
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
    AskVal,
    CallVal,
    PairVal,
};

#[derive(Copy, Clone)]
pub(crate) struct Id;

impl Transformer<Val, Val> for Id {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Id {
    fn transform_default<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        input
    }

    fn transform_symbol<'a, Ctx>(&self, _ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Symbol(s)
    }

    fn transform_pair<'a, Ctx>(&self, _ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Pair(pair)
    }

    fn transform_list<'a, Ctx>(&self, _ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::List(list)
    }

    fn transform_map<'a, Ctx>(&self, _ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Map(map)
    }

    fn transform_call<'a, Ctx>(&self, _ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Call(call)
    }

    fn transform_ask<'a, Ctx>(&self, _ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Ask(ask)
    }
}
