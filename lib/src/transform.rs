use crate::{
    ctx::ref1::CtxMeta,
    transform::{
        eval::Eval,
        form::Form,
        id::Id,
    },
    transformer::{
        input::ByVal,
        Transformer,
    },
    AskVal,
    CallVal,
    ListVal,
    MapVal,
    PairVal,
    Symbol,
    Val,
};

pub(crate) const ID: &str = "id";
pub(crate) const FORM: &str = "form";
pub(crate) const EVAL: &str = "eval";

pub(crate) const SYMBOL_READ_PREFIX: char = '$';
pub(crate) const SYMBOL_MOVE_PREFIX: char = '&';

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Transform {
    Id,
    Form,
    #[default]
    Eval,
}

impl Transformer<Val, Val> for Transform {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform(ctx, input),
            Transform::Form => Form.transform(ctx, input),
            Transform::Eval => Eval.transform(ctx, input),
        }
    }
}

impl ByVal<Val> for Transform {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_default(ctx, input),
            Transform::Form => Form.transform_default(ctx, input),
            Transform::Eval => Eval.transform_default(ctx, input),
        }
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_symbol(ctx, s),
            Transform::Form => Form.transform_symbol(ctx, s),
            Transform::Eval => Eval.transform_symbol(ctx, s),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_pair(ctx, pair),
            Transform::Form => Form.transform_pair(ctx, pair),
            Transform::Eval => Eval.transform_pair(ctx, pair),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_list(ctx, list),
            Transform::Form => Form.transform_list(ctx, list),
            Transform::Eval => Eval.transform_list(ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_map(ctx, map),
            Transform::Form => Form.transform_map(ctx, map),
            Transform::Eval => Eval.transform_map(ctx, map),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_call(ctx, call),
            Transform::Form => Form.transform_call(ctx, call),
            Transform::Eval => Eval.transform_call(ctx, call),
        }
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Transform::Id => Id.transform_ask(ctx, ask),
            Transform::Form => Form.transform_ask(ctx, ask),
            Transform::Eval => Eval.transform_ask(ctx, ask),
        }
    }
}

pub(crate) mod id;

pub(crate) mod form;

pub(crate) mod eval;
