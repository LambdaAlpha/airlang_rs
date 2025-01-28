use crate::{
    AbstractVal,
    AskVal,
    CallVal,
    ChangeVal,
    ListMode,
    ListVal,
    MapMode,
    MapVal,
    PairMode,
    PairVal,
    Symbol,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::{
        abstract1::AbstractMode,
        ask::AskMode,
        call::CallMode,
        change::ChangeMode,
        id::Id,
        symbol::SymbolMode,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct CompMode {
    pub symbol: SymbolMode,
    pub pair: PairMode,
    pub call: CallMode,
    pub abstract1: AbstractMode,
    pub ask: AskMode,
    pub change: ChangeMode,
    pub list: ListMode,
    pub map: MapMode,
}

impl Transformer<Val, Val> for CompMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for CompMode {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        Id.transform(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.symbol.transform(ctx, symbol)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.pair.transform(ctx, pair)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.call.transform(ctx, call)
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.abstract1.transform(ctx, abstract1)
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.ask.transform(ctx, ask)
    }

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.change.transform(ctx, change)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.list.transform(ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        self.map.transform(ctx, map)
    }
}

impl From<UniMode> for CompMode {
    fn from(mode: UniMode) -> Self {
        Self {
            symbol: SymbolMode::from(mode),
            pair: PairMode::from(mode),
            call: CallMode::from(mode),
            abstract1: AbstractMode::from(mode),
            ask: AskMode::from(mode),
            change: ChangeMode::from(mode),
            list: ListMode::from(mode),
            map: MapMode::from(mode),
        }
    }
}
