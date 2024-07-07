use crate::{
    ctx::ref1::CtxMeta,
    mode::{
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
    CommentVal,
    ListVal,
    MapVal,
    PairVal,
    Symbol,
    Val,
};

pub(crate) const ID: &str = "id";
pub(crate) const FORM: &str = "form";
pub(crate) const EVAL: &str = "eval";

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BasicMode {
    Id,
    Form,
    #[default]
    Eval,
}

impl Transformer<Val, Val> for BasicMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform(ctx, input),
            BasicMode::Form => Form.transform(ctx, input),
            BasicMode::Eval => Eval.transform(ctx, input),
        }
    }
}

impl ByVal<Val> for BasicMode {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_default(ctx, input),
            BasicMode::Form => Form.transform_default(ctx, input),
            BasicMode::Eval => Eval.transform_default(ctx, input),
        }
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_symbol(ctx, s),
            BasicMode::Form => Form.transform_symbol(ctx, s),
            BasicMode::Eval => Eval.transform_symbol(ctx, s),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_pair(ctx, pair),
            BasicMode::Form => Form.transform_pair(ctx, pair),
            BasicMode::Eval => Eval.transform_pair(ctx, pair),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_list(ctx, list),
            BasicMode::Form => Form.transform_list(ctx, list),
            BasicMode::Eval => Eval.transform_list(ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_map(ctx, map),
            BasicMode::Form => Form.transform_map(ctx, map),
            BasicMode::Eval => Eval.transform_map(ctx, map),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_call(ctx, call),
            BasicMode::Form => Form.transform_call(ctx, call),
            BasicMode::Eval => Eval.transform_call(ctx, call),
        }
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_ask(ctx, ask),
            BasicMode::Form => Form.transform_ask(ctx, ask),
            BasicMode::Eval => Eval.transform_ask(ctx, ask),
        }
    }

    fn transform_comment<'a, Ctx>(&self, ctx: Ctx, comment: CommentVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            BasicMode::Id => Id.transform_comment(ctx, comment),
            BasicMode::Form => Form.transform_comment(ctx, comment),
            BasicMode::Eval => Eval.transform_comment(ctx, comment),
        }
    }
}
