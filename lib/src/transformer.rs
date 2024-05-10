use std::ops::Deref;

use crate::{
    ctx_access::CtxAccessor,
    transformer::{
        input::ByVal,
        output::OutputBuilder,
    },
    val::{
        list::ListVal,
        map::MapVal,
    },
    Ask,
    Call,
    Pair,
    Val,
};

pub(crate) trait Transformer<Input, Output> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Input) -> Output
    where
        Ctx: CtxAccessor<'a>;
}

pub(crate) struct DefaultByVal;

impl DefaultByVal {
    pub(crate) fn transform_val<'a, Ctx, Output, T>(t: &T, ctx: Ctx, input: Val) -> Output
    where
        Ctx: CtxAccessor<'a>,
        T: ByVal<Output>,
    {
        match input {
            Val::Symbol(s) => t.transform_symbol(ctx, s),
            Val::Pair(p) => t.transform_pair(ctx, p.first, p.second),
            Val::List(l) => t.transform_list(ctx, l),
            Val::Map(m) => t.transform_map(ctx, m),
            Val::Call(c) => t.transform_call(ctx, c.func, c.input),
            Val::Ask(a) => t.transform_ask(ctx, a.func, a.output),
            v => t.transform_default(ctx, v),
        }
    }

    pub(crate) fn transform_pair<'a, Ctx, Output, T, Builder>(
        t: &T,
        mut ctx: Ctx,
        first: Val,
        second: Val,
        builder: Builder,
    ) -> Output
    where
        Ctx: CtxAccessor<'a>,
        T: Transformer<Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let first = t.transform(ctx.reborrow(), first);
        let second = t.transform(ctx, second);
        builder.from_pair(first, second)
    }

    pub(crate) fn transform_list<'a, Ctx, Output, T, Builder>(
        t: &T,
        mut ctx: Ctx,
        list: ListVal,
        builder: Builder,
    ) -> Output
    where
        Ctx: CtxAccessor<'a>,
        T: Transformer<Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let list = list.into_iter().map(|v| t.transform(ctx.reborrow(), v));
        builder.from_list(list)
    }

    pub(crate) fn transform_map<'a, Ctx, Output, T, Builder>(
        t: &T,
        mut ctx: Ctx,
        map: MapVal,
        builder: Builder,
    ) -> Output
    where
        Ctx: CtxAccessor<'a>,
        T: Transformer<Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let map = map.into_iter().map(|(k, v)| {
            let key = t.transform(ctx.reborrow(), k);
            let value = t.transform(ctx.reborrow(), v);
            (key, value)
        });
        builder.from_map(map)
    }

    pub(crate) fn transform_call<'a, Ctx, Output, T, Builder>(
        t: &T,
        mut ctx: Ctx,
        func: Val,
        input: Val,
        builder: Builder,
    ) -> Output
    where
        Ctx: CtxAccessor<'a>,
        T: Transformer<Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = t.transform(ctx.reborrow(), func);
        let input = t.transform(ctx, input);
        builder.from_call(func, input)
    }

    pub(crate) fn transform_ask<'a, Ctx, Output, T, Builder>(
        t: &T,
        mut ctx: Ctx,
        func: Val,
        output: Val,
        builder: Builder,
    ) -> Output
    where
        Ctx: CtxAccessor<'a>,
        T: Transformer<Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = t.transform(ctx.reborrow(), func);
        let output = t.transform(ctx, output);
        builder.from_ask(func, output)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ValBuilder;

impl OutputBuilder<Val> for ValBuilder {
    fn from_pair(&self, first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn from_list<Iter>(&self, iter: Iter) -> Val
    where
        Iter: Iterator<Item = Val>,
    {
        Val::List(iter.collect())
    }

    fn from_map<Iter>(&self, kv_iter: Iter) -> Val
    where
        Iter: Iterator<Item = (Val, Val)>,
    {
        Val::Map(kv_iter.collect())
    }

    fn from_call(&self, func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn from_ask(&self, func: Val, output: Val) -> Val {
        Val::Ask(Box::new(Ask::new(func, output)))
    }
}

impl<I, O, T> Transformer<I, O> for Box<T>
where
    T: Transformer<I, O>,
{
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: I) -> O
    where
        Ctx: CtxAccessor<'a>,
    {
        self.deref().transform(ctx, input)
    }
}

pub(crate) mod input;

pub(crate) mod output;
