use crate::{
    ctx::ref1::CtxMeta,
    symbol::Symbol,
    transform::{
        eval::Eval,
        id::Id,
    },
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
pub(crate) struct Lazy;

impl Transformer<Val, Val> for Lazy {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Lazy {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform_symbol(ctx, s)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_pair(self, ctx, pair)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_list(self, ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_map(self, ctx, map)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if call.func.is_unit() {
            return Eval.transform(ctx, call.unwrap().input);
        }
        DefaultByVal::transform_call(self, ctx, call)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        DefaultByVal::transform_ask(self, ctx, ask)
    }
}
