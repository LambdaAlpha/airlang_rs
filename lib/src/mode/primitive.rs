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
    mode::{
        eval::Eval,
        form::Form,
        id::Id,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

pub(crate) const ID: &str = "id";
pub(crate) const FORM: &str = "form";
pub(crate) const EVAL: &str = "eval";

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveMode {
    Id,
    Form,
    #[default]
    Eval,
}

impl Transformer<Val, Val> for PrimitiveMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform(ctx, input),
            PrimitiveMode::Form => Form.transform(ctx, input),
            PrimitiveMode::Eval => Eval.transform(ctx, input),
        }
    }
}

impl ByVal<Val> for PrimitiveMode {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_default(ctx, input),
            PrimitiveMode::Form => Form.transform_default(ctx, input),
            PrimitiveMode::Eval => Eval.transform_default(ctx, input),
        }
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_symbol(ctx, symbol),
            PrimitiveMode::Form => Form.transform_symbol(ctx, symbol),
            PrimitiveMode::Eval => Eval.transform_symbol(ctx, symbol),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_pair(ctx, pair),
            PrimitiveMode::Form => Form.transform_pair(ctx, pair),
            PrimitiveMode::Eval => Eval.transform_pair(ctx, pair),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_call(ctx, call),
            PrimitiveMode::Form => Form.transform_call(ctx, call),
            PrimitiveMode::Eval => Eval.transform_call(ctx, call),
        }
    }

    fn transform_adapt<'a, Ctx>(&self, ctx: Ctx, adapt: AdaptVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_adapt(ctx, adapt),
            PrimitiveMode::Form => Form.transform_adapt(ctx, adapt),
            PrimitiveMode::Eval => Eval.transform_adapt(ctx, adapt),
        }
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_ask(ctx, ask),
            PrimitiveMode::Form => Form.transform_ask(ctx, ask),
            PrimitiveMode::Eval => Eval.transform_ask(ctx, ask),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_list(ctx, list),
            PrimitiveMode::Form => Form.transform_list(ctx, list),
            PrimitiveMode::Eval => Eval.transform_list(ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_map(ctx, map),
            PrimitiveMode::Form => Form.transform_map(ctx, map),
            PrimitiveMode::Eval => Eval.transform_map(ctx, map),
        }
    }
}
