use crate::{
    AbstractMode,
    AbstractVal,
    CallVal,
    ChangeVal,
    EitherVal,
    EquivVal,
    GenerateMode,
    GenerateVal,
    InverseVal,
    ListMode,
    ListVal,
    MapMode,
    MapVal,
    PairMode,
    PairVal,
    ReifyMode,
    ReifyVal,
    Symbol,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::{
        call::CallMode,
        change::ChangeMode,
        either::EitherMode,
        equiv::EquivMode,
        inverse::InverseMode,
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
    pub either: Option<EitherMode>,
    pub change: Option<ChangeMode>,
    pub call: Option<CallMode>,
    pub reify: Option<ReifyMode>,
    pub equiv: Option<EquivMode>,
    pub inverse: Option<InverseMode>,
    pub generate: Option<GenerateMode>,
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
    fn transform_default<'a, Ctx>(&self, _ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        input
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        self.symbol.transform(ctx, symbol)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.pair.transform(ctx, pair)
    }

    fn transform_either<'a, Ctx>(&self, ctx: Ctx, either: EitherVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.either.transform(ctx, either)
    }

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.change.transform(ctx, change)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.call.transform(ctx, call)
    }

    fn transform_reify<'a, Ctx>(&self, ctx: Ctx, reify: ReifyVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.reify.transform(ctx, reify)
    }

    fn transform_equiv<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.equiv.transform(ctx, equiv)
    }

    fn transform_inverse<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.inverse.transform(ctx, inverse)
    }

    fn transform_generate<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.generate.transform(ctx, generate)
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.abstract1.transform(ctx, abstract1)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.list.transform(ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.map.transform(ctx, map)
    }
}

impl From<Option<UniMode>> for CompMode {
    fn from(mode: Option<UniMode>) -> Self {
        match mode {
            None => Self {
                symbol: None,
                pair: None,
                either: None,
                call: None,
                reify: None,
                equiv: None,
                inverse: None,
                generate: None,
                abstract1: None,
                change: None,
                list: None,
                map: None,
            },
            Some(mode) => Self {
                symbol: Some(SymbolMode::from(mode)),
                pair: Some(PairMode::from(mode)),
                either: Some(EitherMode::from(mode)),
                call: Some(CallMode::from(mode)),
                reify: Some(ReifyMode::from(mode)),
                equiv: Some(EquivMode::from(mode)),
                inverse: Some(InverseMode::from(mode)),
                generate: Some(GenerateMode::from(mode)),
                abstract1: Some(AbstractMode::from(mode)),
                change: Some(ChangeMode::from(mode)),
                list: Some(ListMode::from(mode)),
                map: Some(MapMode::from(mode)),
            },
        }
    }
}
