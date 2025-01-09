use crate::{
    AbstractVal,
    AskVal,
    CallVal,
    PairVal,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::id::Id,
    symbol::Symbol,
    transformer::{
        ByVal,
        Transformer,
    },
    val::{
        Val,
        list::ListVal,
        map::MapVal,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Form {
    Literal,
    #[default]
    Ref,
    Move,
}

// default instance
#[allow(unused)]
pub(crate) const FORM: Form = Form::Ref;

pub(crate) const LITERAL: char = '.';
pub(crate) const REF: char = '*';
pub(crate) const MOVE: char = '^';

impl Transformer<Symbol, Val> for Form {
    fn transform<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            Form::Literal => FormCore::transform_symbol::<LITERAL, _>(ctx, symbol),
            Form::Ref => FormCore::transform_symbol::<REF, _>(ctx, symbol),
            Form::Move => FormCore::transform_symbol::<MOVE, _>(ctx, symbol),
        }
    }
}

impl Transformer<Val, Val> for Form {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Form {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.transform(ctx, symbol)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_pair(self, self, ctx, pair)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_call(self, self, ctx, call)
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_abstract(self, self, ctx, abstract1)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_ask(self, self, ctx, ask)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_list(self, ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_map(self, self, ctx, map)
    }
}
