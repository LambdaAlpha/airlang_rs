use std::ops::Deref;

use crate::{
    transformer::{
        input::{
            ByRef,
            ByVal,
        },
        output::OutputBuilder,
    },
    val::{
        list::ListVal,
        map::MapVal,
    },
    Call,
    Pair,
    Reverse,
    Val,
};

pub(crate) trait Transformer<Ctx, Input, Output> {
    fn transform(&self, ctx: &mut Ctx, input: Input) -> Output;
}

pub(crate) struct DefaultByVal;

impl DefaultByVal {
    pub(crate) fn transform_val<Ctx, Output, T>(t: &T, ctx: &mut Ctx, input: Val) -> Output
    where
        T: ByVal<Ctx, Output>,
    {
        match input {
            Val::Symbol(s) => t.transform_symbol(ctx, s),
            Val::Pair(p) => t.transform_pair(ctx, p.first, p.second),
            Val::List(l) => t.transform_list(ctx, l),
            Val::Map(m) => t.transform_map(ctx, m),
            Val::Call(c) => t.transform_call(ctx, c.func, c.input),
            Val::Reverse(r) => t.transform_reverse(ctx, r.func, r.output),
            v => t.transform_default(ctx, v),
        }
    }

    pub(crate) fn transform_pair<Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        first: Val,
        second: Val,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let first = t.transform(ctx, first);
        let second = t.transform(ctx, second);
        builder.from_pair(first, second)
    }

    pub(crate) fn transform_list<Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        list: ListVal,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let list = list.into_iter().map(|v| t.transform(ctx, v));
        builder.from_list(list)
    }

    pub(crate) fn transform_map<Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        map: MapVal,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let map = map.into_iter().map(|(k, v)| {
            let key = t.transform(ctx, k);
            let value = t.transform(ctx, v);
            (key, value)
        });
        builder.from_map(map)
    }

    pub(crate) fn transform_call<Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        func: Val,
        input: Val,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = t.transform(ctx, func);
        let input = t.transform(ctx, input);
        builder.from_call(func, input)
    }

    pub(crate) fn transform_reverse<Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        func: Val,
        output: Val,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = t.transform(ctx, func);
        let output = t.transform(ctx, output);
        builder.from_reverse(func, output)
    }
}

pub(crate) struct DefaultByRef;

impl DefaultByRef {
    pub(crate) fn transform_val<'a, Ctx, Output, T>(t: &T, ctx: &mut Ctx, input: &'a Val) -> Output
    where
        T: ByRef<'a, Ctx, Output>,
    {
        match input {
            Val::Symbol(s) => t.transform_symbol(ctx, s),
            Val::Pair(p) => t.transform_pair(ctx, &p.first, &p.second),
            Val::List(l) => t.transform_list(ctx, l),
            Val::Map(m) => t.transform_map(ctx, m),
            Val::Call(c) => t.transform_call(ctx, &c.func, &c.input),
            Val::Reverse(r) => t.transform_reverse(ctx, &r.func, &r.output),
            v => t.transform_default(ctx, v),
        }
    }

    pub(crate) fn transform_pair<'a, Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        first: &'a Val,
        second: &'a Val,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let first = t.transform(ctx, first);
        let second = t.transform(ctx, second);
        builder.from_pair(first, second)
    }

    pub(crate) fn transform_list<'a, Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        list: &'a ListVal,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let list = list.into_iter().map(|v| t.transform(ctx, v));
        builder.from_list(list)
    }

    pub(crate) fn transform_map<'a, Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        map: &'a MapVal,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let map = map.into_iter().map(|(k, v)| {
            let key = t.transform(ctx, k);
            let value = t.transform(ctx, v);
            (key, value)
        });
        builder.from_map(map)
    }

    pub(crate) fn transform_call<'a, Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        func: &'a Val,
        input: &'a Val,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = t.transform(ctx, func);
        let input = t.transform(ctx, input);
        builder.from_call(func, input)
    }

    pub(crate) fn transform_reverse<'a, Ctx, Output, T, Builder>(
        t: &T,
        ctx: &mut Ctx,
        func: &'a Val,
        output: &'a Val,
        builder: Builder,
    ) -> Output
    where
        T: Transformer<Ctx, &'a Val, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = t.transform(ctx, func);
        let output = t.transform(ctx, output);
        builder.from_reverse(func, output)
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

    fn from_reverse(&self, func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

impl<Ctx, I, O, T> Transformer<Ctx, I, O> for Box<T>
where
    T: Transformer<Ctx, I, O>,
{
    fn transform(&self, ctx: &mut Ctx, input: I) -> O {
        self.deref().transform(ctx, input)
    }
}

pub(crate) mod input;

pub(crate) mod output;
