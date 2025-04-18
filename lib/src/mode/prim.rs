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
    SymbolMode,
    UniMode,
    Val,
    core::{
        EvalCore,
        FormCore,
    },
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PrimMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<DataMode>,
    pub either: Option<DataMode>,
    pub change: Option<DataMode>,
    pub call: Option<CodeMode>,
    pub reify: Option<DataMode>,
    pub equiv: Option<DataMode>,
    pub inverse: Option<DataMode>,
    pub generate: Option<DataMode>,
    pub abstract1: Option<DataMode>,
    pub list: Option<DataMode>,
    pub map: Option<DataMode>,
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
pub struct DataMode;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CodeMode {
    Form,
    Eval,
}

impl Transformer<Val, Val> for PrimMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for PrimMode {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        Id.transform(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        match self.symbol {
            None => Id.transform_symbol(ctx, symbol),
            Some(mode) => mode.transform(ctx, symbol),
        }
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.pair {
            None => Id.transform_pair(ctx, pair),
            Some(_) => FormCore::transform_pair(self, self, ctx, pair),
        }
    }

    fn transform_either<'a, Ctx>(&self, ctx: Ctx, either: EitherVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.either {
            None => Id.transform_either(ctx, either),
            Some(_) => FormCore::transform_either(self, self, ctx, either),
        }
    }

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.change {
            None => Id.transform_change(ctx, change),
            Some(_) => FormCore::transform_change(self, self, ctx, change),
        }
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.call {
            None => Id.transform_call(ctx, call),
            Some(mode) => match mode {
                CodeMode::Form => FormCore::transform_call(self, self, ctx, call),
                CodeMode::Eval => EvalCore::transform_call(self, self, ctx, call),
            },
        }
    }

    fn transform_reify<'a, Ctx>(&self, ctx: Ctx, reify: ReifyVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.reify {
            None => Id.transform_reify(ctx, reify),
            Some(_) => FormCore::transform_reify(self, ctx, reify),
        }
    }

    fn transform_equiv<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.equiv {
            None => Id.transform_equiv(ctx, equiv),
            Some(_) => FormCore::transform_equiv(self, ctx, equiv),
        }
    }

    fn transform_inverse<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.inverse {
            None => Id.transform_inverse(ctx, inverse),
            Some(_) => FormCore::transform_inverse(self, ctx, inverse),
        }
    }

    fn transform_generate<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.generate {
            None => Id.transform_generate(ctx, generate),
            Some(_) => FormCore::transform_generate(self, ctx, generate),
        }
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.abstract1 {
            None => Id.transform_abstract(ctx, abstract1),
            Some(_) => FormCore::transform_abstract(self, ctx, abstract1),
        }
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.list {
            None => Id.transform_list(ctx, list),
            Some(_) => FormCore::transform_list(self, ctx, list),
        }
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        match self.map {
            None => Id.transform_map(ctx, map),
            Some(_) => FormCore::transform_map(self, self, ctx, map),
        }
    }
}

impl From<Option<UniMode>> for PrimMode {
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
                pair: Some(DataMode::from(mode)),
                either: Some(DataMode::from(mode)),
                call: Some(CodeMode::from(mode)),
                reify: Some(DataMode::from(mode)),
                equiv: Some(DataMode::from(mode)),
                inverse: Some(DataMode::from(mode)),
                generate: Some(DataMode::from(mode)),
                abstract1: Some(DataMode::from(mode)),
                change: Some(DataMode::from(mode)),
                list: Some(DataMode::from(mode)),
                map: Some(DataMode::from(mode)),
            },
        }
    }
}

impl From<UniMode> for DataMode {
    fn from(_mode: UniMode) -> Self {
        DataMode
    }
}

impl From<UniMode> for CodeMode {
    fn from(mode: UniMode) -> Self {
        mode.code
    }
}
