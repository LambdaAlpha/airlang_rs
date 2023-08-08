use crate::{
    semantics::{
        eval::{
            input::{
                ByRef,
                ByVal,
            },
            output::OutputBuilder,
        },
        val::{
            ListVal,
            MapVal,
        },
        Val,
    },
    types::{
        Call,
        Pair,
        Reverse,
    },
};

pub(crate) trait Evaluator<Ctx, Input, Output> {
    fn eval(&self, ctx: &mut Ctx, input: Input) -> Output;
}

pub(crate) struct DefaultByVal;

impl DefaultByVal {
    pub(crate) fn eval_val<Ctx, Output, Eval>(eval: &Eval, ctx: &mut Ctx, input: Val) -> Output
    where
        Eval: ByVal<Ctx, Output>,
    {
        match input {
            Val::Symbol(s) => eval.eval_symbol(ctx, s),
            Val::Pair(p) => eval.eval_pair(ctx, p.first, p.second),
            Val::List(l) => eval.eval_list(ctx, l),
            Val::Map(m) => eval.eval_map(ctx, m),
            Val::Call(c) => eval.eval_call(ctx, c.func, c.input),
            Val::Reverse(r) => eval.eval_reverse(ctx, r.func, r.output),
            Val::Ref(k) => eval.eval_ref(ctx, k),
            v => eval.eval_atoms(ctx, v),
        }
    }

    pub(crate) fn eval_pair<Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        first: Val,
        second: Val,
    ) -> Output
    where
        Eval: ByVal<Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let first = eval.eval(ctx, first);
        let second = eval.eval(ctx, second);
        Builder::from_pair(first, second)
    }

    pub(crate) fn eval_list<Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        list: ListVal,
    ) -> Output
    where
        Eval: ByVal<Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let list = list.into_iter().map(|v| eval.eval(ctx, v));
        Builder::from_list(list)
    }

    pub(crate) fn eval_map<Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        map: MapVal,
    ) -> Output
    where
        Eval: ByVal<Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let map = map.into_iter().map(|(k, v)| {
            let key = eval.eval(ctx, k);
            let value = eval.eval(ctx, v);
            (key, value)
        });
        Builder::from_map(map)
    }

    pub(crate) fn eval_call<Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        func: Val,
        input: Val,
    ) -> Output
    where
        Eval: ByVal<Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = eval.eval(ctx, func);
        let input = eval.eval(ctx, input);
        Builder::from_call(func, input)
    }

    #[allow(unused)]
    pub(crate) fn eval_reverse<Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        func: Val,
        output: Val,
    ) -> Output
    where
        Eval: ByVal<Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = eval.eval(ctx, func);
        let output = eval.eval(ctx, output);
        Builder::from_reverse(func, output)
    }
}

pub(crate) struct DefaultByRef;

impl DefaultByRef {
    pub(crate) fn eval_val<'a, Ctx, Output, Eval>(
        eval: &Eval,
        ctx: &mut Ctx,
        input: &'a Val,
    ) -> Output
    where
        Eval: ByRef<'a, Ctx, Output>,
    {
        match input {
            Val::Symbol(s) => eval.eval_symbol(ctx, s),
            Val::Pair(p) => eval.eval_pair(ctx, &p.first, &p.second),
            Val::List(l) => eval.eval_list(ctx, l),
            Val::Map(m) => eval.eval_map(ctx, m),
            Val::Call(c) => eval.eval_call(ctx, &c.func, &c.input),
            Val::Reverse(r) => eval.eval_reverse(ctx, &r.func, &r.output),
            Val::Ref(k) => eval.eval_ref(ctx, k),
            v => eval.eval_atoms(ctx, v),
        }
    }

    pub(crate) fn eval_pair<'a, Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        first: &'a Val,
        second: &'a Val,
    ) -> Output
    where
        Eval: ByRef<'a, Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let first = eval.eval(ctx, first);
        let second = eval.eval(ctx, second);
        Builder::from_pair(first, second)
    }

    pub(crate) fn eval_list<'a, Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        list: &'a ListVal,
    ) -> Output
    where
        Eval: ByRef<'a, Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let list = list.into_iter().map(|v| eval.eval(ctx, v));
        Builder::from_list(list)
    }

    pub(crate) fn eval_map<'a, Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        map: &'a MapVal,
    ) -> Output
    where
        Eval: ByRef<'a, Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let map = map.into_iter().map(|(k, v)| {
            let key = eval.eval(ctx, k);
            let value = eval.eval(ctx, v);
            (key, value)
        });
        Builder::from_map(map)
    }

    pub(crate) fn eval_call<'a, Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        func: &'a Val,
        input: &'a Val,
    ) -> Output
    where
        Eval: ByRef<'a, Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = eval.eval(ctx, func);
        let input = eval.eval(ctx, input);
        Builder::from_call(func, input)
    }

    #[allow(unused)]
    pub(crate) fn eval_reverse<'a, Ctx, Output, Eval, Builder>(
        eval: &Eval,
        ctx: &mut Ctx,
        func: &'a Val,
        output: &'a Val,
    ) -> Output
    where
        Eval: ByRef<'a, Ctx, Output>,
        Builder: OutputBuilder<Output>,
    {
        let func = eval.eval(ctx, func);
        let output = eval.eval(ctx, output);
        Builder::from_reverse(func, output)
    }
}

pub(crate) struct ValBuilder;

impl OutputBuilder<Val> for ValBuilder {
    fn from_pair(first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn from_list<Iter>(iter: Iter) -> Val
    where
        Iter: Iterator<Item = Val>,
    {
        Val::List(iter.collect())
    }

    fn from_map<Iter>(kv_iter: Iter) -> Val
    where
        Iter: Iterator<Item = (Val, Val)>,
    {
        Val::Map(kv_iter.collect())
    }

    fn from_call(func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn from_reverse(func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

pub(crate) mod input;

pub(crate) mod output;
