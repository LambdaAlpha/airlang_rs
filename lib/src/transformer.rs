use std::ops::Deref;

use crate::{
    ctx::ref1::CtxMeta,
    transformer::input::ByVal,
    val::{
        list::ListVal,
        map::MapVal,
    },
    Ask,
    AskVal,
    Call,
    CallVal,
    List,
    Map,
    Pair,
    PairVal,
    Val,
};

pub(crate) trait Transformer<Input, Output> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Input) -> Output
    where
        Ctx: CtxMeta<'a>;
}

pub(crate) struct DefaultByVal;

impl DefaultByVal {
    pub(crate) fn transform_val<'a, Ctx, T>(t: &T, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: ByVal<Val>,
    {
        match input {
            Val::Symbol(s) => t.transform_symbol(ctx, s),
            Val::Pair(p) => t.transform_pair(ctx, p),
            Val::List(l) => t.transform_list(ctx, l),
            Val::Map(m) => t.transform_map(ctx, m),
            Val::Call(c) => t.transform_call(ctx, c),
            Val::Ask(a) => t.transform_ask(ctx, a),
            v => t.transform_default(ctx, v),
        }
    }

    pub(crate) fn transform_pair<'a, Ctx, T>(t: &T, mut ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: Transformer<Val, Val>,
    {
        let pair = Pair::from(pair);
        let first = t.transform(ctx.reborrow(), pair.first);
        let second = t.transform(ctx, pair.second);
        let pair = Pair::new(first, second);
        Val::Pair(pair.into())
    }

    pub(crate) fn transform_list<'a, Ctx, T>(t: &T, mut ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: Transformer<Val, Val>,
    {
        let list = List::from(list);
        let list: List<Val> = list
            .into_iter()
            .map(|v| t.transform(ctx.reborrow(), v))
            .collect();
        Val::List(list.into())
    }

    pub(crate) fn transform_map<'a, Ctx, T>(t: &T, mut ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: Transformer<Val, Val>,
    {
        let map = Map::from(map);
        let map: Map<Val, Val> = map
            .into_iter()
            .map(|(k, v)| {
                let key = t.transform(ctx.reborrow(), k);
                let value = t.transform(ctx.reborrow(), v);
                (key, value)
            })
            .collect();
        Val::Map(map.into())
    }

    pub(crate) fn transform_call<'a, Ctx, T>(t: &T, mut ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: Transformer<Val, Val>,
    {
        let call = Call::from(call);
        let func = t.transform(ctx.reborrow(), call.func);
        let input = t.transform(ctx, call.input);
        let call = Call::new(func, input);
        Val::Call(call.into())
    }

    pub(crate) fn transform_ask<'a, Ctx, T>(t: &T, mut ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: Transformer<Val, Val>,
    {
        let ask = Ask::from(ask);
        let func = t.transform(ctx.reborrow(), ask.func);
        let output = t.transform(ctx, ask.output);
        let ask = Ask::new(func, output);
        Val::Ask(ask.into())
    }
}

impl<I, O, T> Transformer<I, O> for Box<T>
where
    T: Transformer<I, O>,
{
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: I) -> O
    where
        Ctx: CtxMeta<'a>,
    {
        self.deref().transform(ctx, input)
    }
}

pub(crate) mod input;
