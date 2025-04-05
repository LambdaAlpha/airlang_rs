use crate::{
    AbstractVal,
    CallVal,
    ChangeVal,
    OptimizeVal,
    PairVal,
    SolveVal,
    SymbolMode,
    core::{
        EvalCore,
        FormCore,
    },
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
pub(crate) struct Eval {
    pub(crate) symbol: SymbolMode,
}

// default instance
pub(crate) const EVAL: Eval = Eval::new(SymbolMode::Ref);

impl Transformer<Val, Val> for Eval {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Eval {
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

    fn transform_change<'a, Ctx>(&self, ctx: Ctx, change: ChangeVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_change(self, self, ctx, change)
    }

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        EvalCore::transform_call(self, self, ctx, call)
    }

    fn transform_optimize<'a, Ctx>(&self, ctx: Ctx, optimize: OptimizeVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_optimize(self, ctx, optimize)
    }

    fn transform_solve<'a, Ctx>(&self, ctx: Ctx, solve: SolveVal) -> Val
    where Ctx: CtxMeta<'a> {
        FormCore::transform_solve(self, ctx, solve)
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

impl Eval {
    pub(crate) const fn new(symbol: SymbolMode) -> Self {
        Eval { symbol }
    }
}
