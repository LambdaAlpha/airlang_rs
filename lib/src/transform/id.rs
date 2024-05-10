use crate::{
    ask::Ask,
    call::Call,
    ctx_access::CtxAccessor,
    pair::Pair,
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

    fn transform_pair<'a, Ctx>(&self, _ctx: Ctx, first: Val, second: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Pair(Box::new(Pair::new(first, second)))
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

    fn transform_call<'a, Ctx>(&self, _ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn transform_ask<'a, Ctx>(&self, _ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Val::Ask(Box::new(Ask::new(func, output)))
    }
}
