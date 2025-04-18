use std::ops::Deref;

use crate::{
    AbstractVal,
    CallVal,
    ChangeVal,
    EitherVal,
    EquivVal,
    GenerateVal,
    InverseVal,
    ListVal,
    MapVal,
    PairVal,
    ReifyVal,
    Symbol,
    Val,
    ctx::ref1::CtxMeta,
};

pub(crate) trait Transformer<Input, Output> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Input) -> Output
    where Ctx: CtxMeta<'a>;
}

impl<I, O, T> Transformer<I, O> for Box<T>
where T: Transformer<I, O>
{
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: I) -> O
    where Ctx: CtxMeta<'a> {
        self.deref().transform(ctx, input)
    }
}

impl<I, O, T> Transformer<I, O> for Option<T>
where
    I: Into<O>,
    T: Transformer<I, O>,
{
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: I) -> O
    where Ctx: CtxMeta<'a> {
        match self {
            None => input.into(),
            Some(t) => t.transform(ctx, input),
        }
    }
}

pub(crate) trait ByVal<Output>: Transformer<Val, Output> {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_either<'a, Ctx>(&self, ctx: Ctx, either: EitherVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_reify<'a, Ctx>(&self, ctx: Ctx, reify: ReifyVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_equiv<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_inverse<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_generate<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Output
    where Ctx: CtxMeta<'a>;

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Output
    where Ctx: CtxMeta<'a>;
}
