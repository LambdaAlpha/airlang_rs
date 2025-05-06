use crate::CallVal;
use crate::ListMode;
use crate::ListVal;
use crate::MapMode;
use crate::MapVal;
use crate::PairMode;
use crate::PairVal;
use crate::Symbol;
use crate::UniMode;
use crate::Val;
use crate::core::FormCore;
use crate::ctx::ref1::CtxMeta;
use crate::mode::call::CallMode;
use crate::mode::symbol::SymbolMode;
use crate::transformer::ByVal;
use crate::transformer::Transformer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompMode {
    pub symbol: Option<SymbolMode>,
    pub pair: Option<PairMode>,
    pub call: Option<CallMode>,
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

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, call: CallVal) -> Val
    where Ctx: CtxMeta<'a> {
        self.call.transform(ctx, call)
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
            None => Self { symbol: None, pair: None, call: None, list: None, map: None },
            Some(mode) => Self {
                symbol: Some(SymbolMode::from(mode)),
                pair: Some(PairMode::from(mode)),
                call: Some(CallMode::from(mode)),
                list: Some(ListMode::from(mode)),
                map: Some(MapMode::from(mode)),
            },
        }
    }
}
