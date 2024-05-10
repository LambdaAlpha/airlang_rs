use crate::{
    ctx_access::CtxAccessor,
    symbol::Symbol,
    transform::{
        eval::Eval,
        id::Id,
    },
    transformer::{
        input::ByVal,
        DefaultByVal,
        Transformer,
        ValBuilder,
    },
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
};

#[derive(Copy, Clone)]
pub(crate) struct Lazy;

impl Transformer<Val, Val> for Lazy {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Lazy {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Id.transform_symbol(ctx, s)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, first: Val, second: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if func.is_unit() {
            return Eval.transform(ctx, input);
        }
        DefaultByVal::transform_call(self, ctx, func, input, ValBuilder)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_ask(self, ctx, func, output, ValBuilder)
    }
}
