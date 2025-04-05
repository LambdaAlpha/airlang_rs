use crate::{
    AbstractMode,
    AbstractVal,
    CallVal,
    ChangeVal,
    InverseVal,
    ListMode,
    ListVal,
    MapMode,
    MapVal,
    OptimizeVal,
    PairMode,
    PairVal,
    Symbol,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::{
        call::CallMode,
        change::ChangeMode,
        id::Id,
        inverse::InverseMode,
        optimize::OptimizeMode,
        symbol::SymbolMode,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<PairMode>,
    pub change: Option<ChangeMode>,
    pub call: Option<CallMode>,
    pub optimize: Option<OptimizeMode>,
    pub inverse: Option<InverseMode>,
    pub abstract1: Option<AbstractMode>,
    pub list: Option<ListMode>,
    pub map: Option<MapMode>,
}

impl Transformer<Val, Val> for CompMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for CompMode {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        Id.transform(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.symbol {
            None => Id.transform_symbol(ctx, symbol),
            Some(mode) => mode.transform(ctx, symbol),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.pair {
            None => Id.transform_pair(ctx, pair),
            Some(mode) => mode.transform(ctx, pair),
        }
    }

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.change {
            None => Id.transform_change(ctx, change),
            Some(mode) => mode.transform(ctx, change),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.call {
            None => Id.transform_call(ctx, call),
            Some(mode) => mode.transform(ctx, call),
        }
    }

    fn transform_optimize<'a, Ctx>(&self, ctx: Ctx, optimize: OptimizeVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.optimize {
            None => Id.transform_optimize(ctx, optimize),
            Some(mode) => mode.transform(ctx, optimize),
        }
    }

    fn transform_inverse<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.inverse {
            None => Id.transform_inverse(ctx, inverse),
            Some(mode) => mode.transform(ctx, inverse),
        }
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.abstract1 {
            None => Id.transform_abstract(ctx, abstract1),
            Some(mode) => mode.transform(ctx, abstract1),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.list {
            None => Id.transform_list(ctx, list),
            Some(mode) => mode.transform(ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        match &self.map {
            None => Id.transform_map(ctx, map),
            Some(mode) => mode.transform(ctx, map),
        }
    }
}

impl From<Option<UniMode>> for CompMode {
    fn from(mode: Option<UniMode>) -> Self {
        match mode {
            None => Self {
                symbol: None,
                pair: None,
                call: None,
                optimize: None,
                inverse: None,
                abstract1: None,
                change: None,
                list: None,
                map: None,
            },
            Some(mode) => Self {
                symbol: Some(SymbolMode::from(mode)),
                pair: Some(PairMode::from(mode)),
                call: Some(CallMode::from(mode)),
                optimize: Some(OptimizeMode::from(mode)),
                inverse: Some(InverseMode::from(mode)),
                abstract1: Some(AbstractMode::from(mode)),
                change: Some(ChangeMode::from(mode)),
                list: Some(ListMode::from(mode)),
                map: Some(MapMode::from(mode)),
            },
        }
    }
}
