use std::ops::Deref;

use crate::{
    AdaptVal,
    AskVal,
    CallVal,
    ListVal,
    MapVal,
    PairVal,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
};

pub(crate) trait Transformer<Input, Output> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Input) -> Output
    where
        Ctx: CtxMeta<'a>;
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

pub(crate) trait ByVal<Output>: Transformer<Val, Output> {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_adapt<'a, Ctx>(&self, ctx: Ctx, adapt: AdaptVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Output
    where
        Ctx: CtxMeta<'a>;

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Output
    where
        Ctx: CtxMeta<'a>;
}
