use crate::{
    AbstractVal,
    AskVal,
    CallVal,
    ListVal,
    MapVal,
    PairVal,
    Symbol,
    SymbolMode,
    UniMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::{
        eval::EvalMode,
        form::FormMode,
        id::Id,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimMode {
    pub symbol: SymbolMode,
    pub pair: FormMode,
    pub call: EvalMode,
    pub abstract1: EvalMode,
    pub ask: EvalMode,
    pub list: FormMode,
    pub map: FormMode,
}

impl Transformer<Val, Val> for PrimMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for PrimMode {
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
        match self.pair {
            FormMode::Id => Id.transform_pair(ctx, pair),
            FormMode::Form => FormCore::transform_pair(self, self, ctx, pair),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.call {
            EvalMode::Id => Id.transform_call(ctx, call),
            EvalMode::Form => FormCore::transform_call(self, self, ctx, call),
            EvalMode::Eval => EvalCore::transform_call(self, self, ctx, call),
        }
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.abstract1 {
            EvalMode::Id => Id.transform_abstract(ctx, abstract1),
            EvalMode::Form => FormCore::transform_abstract(self, self, ctx, abstract1),
            EvalMode::Eval => EvalCore::transform_abstract(self, self, ctx, abstract1),
        }
    }

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, ask: AskVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self.ask {
            EvalMode::Id => Id.transform_ask(ctx, ask),
            EvalMode::Form => FormCore::transform_ask(self, self, ctx, ask),
            EvalMode::Eval => EvalCore::transform_ask(self, self, ctx, ask),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match &self.list {
            FormMode::Id => Id.transform_list(ctx, list),
            FormMode::Form => FormCore::transform_list(self, ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match &self.map {
            FormMode::Id => Id.transform_map(ctx, map),
            FormMode::Form => FormCore::transform_map(self, self, ctx, map),
        }
    }
}

impl From<UniMode> for PrimMode {
    fn from(mode: UniMode) -> Self {
        Self {
            symbol: SymbolMode::from(mode),
            pair: FormMode::from(mode),
            call: EvalMode::from(mode),
            abstract1: EvalMode::from(mode),
            ask: EvalMode::from(mode),
            list: FormMode::from(mode),
            map: FormMode::from(mode),
        }
    }
}
