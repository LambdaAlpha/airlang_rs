use crate::{
    AbstractVal,
    CallVal,
    ChangeVal,
    EitherVal,
    EquivVal,
    GenerateVal,
    InverseVal,
    PairVal,
    ReifyVal,
    SymbolMode,
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Form {
    pub(crate) symbol: SymbolMode,
}

// default instance
#[expect(dead_code)]
pub(crate) const FORM: Form = Form::new(SymbolMode::Ref);

impl Transformer<Val, Val> for Form {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Form {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, symbol: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        self.symbol.transform(ctx, symbol)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, pair: PairVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_pair(self, self, ctx, pair)
    }

    fn transform_either<'a, Ctx>(&self, ctx: Ctx, either: EitherVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_either(self, self, ctx, either)
    }

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_change(self, self, ctx, change)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_call(self, self, ctx, call)
    }

    fn transform_reify<'a, Ctx>(&self, ctx: Ctx, reify: ReifyVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_reify(self, ctx, reify)
    }

    fn transform_equiv<'a, Ctx>(&self, ctx: Ctx, equiv: EquivVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_equiv(self, ctx, equiv)
    }

    fn transform_inverse<'a, Ctx>(&self, ctx: Ctx, inverse: InverseVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_inverse(self, ctx, inverse)
    }

    fn transform_generate<'a, Ctx>(&self, ctx: Ctx, generate: GenerateVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_generate(self, ctx, generate)
    }

    fn transform_abstract<'a, Ctx>(&self, ctx: Ctx, abstract1: AbstractVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_abstract(self, ctx, abstract1)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_list(self, ctx, list)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_map(self, self, ctx, map)
    }
}

impl Form {
    pub(crate) const fn new(symbol: SymbolMode) -> Self {
        Form { symbol }
    }
}
