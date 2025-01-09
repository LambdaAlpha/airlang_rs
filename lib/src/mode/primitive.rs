use const_format::concatcp;

use crate::{
    AbstractVal,
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
        form::{
            Form,
            LITERAL,
            MOVE,
            REF,
        },
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

pub(crate) const FORM_LITERAL: &str = concatcp!(FORM, LITERAL);
pub(crate) const FORM_REF: &str = concatcp!(FORM, REF);
pub(crate) const FORM_MOVE: &str = concatcp!(FORM, MOVE);
pub(crate) const EVAL_LITERAL: &str = concatcp!(EVAL, LITERAL);
pub(crate) const EVAL_REF: &str = concatcp!(EVAL, REF);
pub(crate) const EVAL_MOVE: &str = concatcp!(EVAL, MOVE);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveMode {
    Id,
    Form(Form),
    Eval(Eval),
}

impl Transformer<Val, Val> for PrimitiveMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform(ctx, input),
            PrimitiveMode::Form(mode) => mode.transform(ctx, input),
            PrimitiveMode::Eval(mode) => mode.transform(ctx, input),
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
            PrimitiveMode::Form(mode) => mode.transform_default(ctx, input),
            PrimitiveMode::Eval(mode) => mode.transform_default(ctx, input),
        }
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_symbol(ctx, symbol),
            PrimitiveMode::Form(mode) => mode.transform_symbol(ctx, symbol),
            PrimitiveMode::Eval(mode) => mode.transform_symbol(ctx, symbol),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_pair(ctx, pair),
            PrimitiveMode::Form(mode) => mode.transform_pair(ctx, pair),
            PrimitiveMode::Eval(mode) => mode.transform_pair(ctx, pair),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_call(ctx, call),
            PrimitiveMode::Form(mode) => mode.transform_call(ctx, call),
            PrimitiveMode::Eval(mode) => mode.transform_call(ctx, call),
        }
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_abstract(ctx, abstract1),
            PrimitiveMode::Form(mode) => mode.transform_abstract(ctx, abstract1),
            PrimitiveMode::Eval(mode) => mode.transform_abstract(ctx, abstract1),
        }
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_ask(ctx, ask),
            PrimitiveMode::Form(mode) => mode.transform_ask(ctx, ask),
            PrimitiveMode::Eval(mode) => mode.transform_ask(ctx, ask),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_list(ctx, list),
            PrimitiveMode::Form(mode) => mode.transform_list(ctx, list),
            PrimitiveMode::Eval(mode) => mode.transform_list(ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            PrimitiveMode::Id => Id.transform_map(ctx, map),
            PrimitiveMode::Form(mode) => mode.transform_map(ctx, map),
            PrimitiveMode::Eval(mode) => mode.transform_map(ctx, map),
        }
    }
}

impl Default for PrimitiveMode {
    fn default() -> Self {
        PrimitiveMode::Eval(Eval::default())
    }
}
