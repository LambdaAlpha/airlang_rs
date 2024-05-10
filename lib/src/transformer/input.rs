use crate::{
    ctx_access::CtxAccessor,
    symbol::Symbol,
    transformer::Transformer,
    val::{
        list::ListVal,
        map::MapVal,
    },
    Val,
};

pub(crate) trait ByVal<Output>: Transformer<Val, Output> {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Output
    where
        Ctx: CtxAccessor<'a>;

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Output
    where
        Ctx: CtxAccessor<'a>;

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, first: Val, second: Val) -> Output
    where
        Ctx: CtxAccessor<'a>;

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Output
    where
        Ctx: CtxAccessor<'a>;

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Output
    where
        Ctx: CtxAccessor<'a>;

    fn transform_call<'a, Ctx>(&self, ctx: Ctx, func: Val, input: Val) -> Output
    where
        Ctx: CtxAccessor<'a>;

    fn transform_ask<'a, Ctx>(&self, ctx: Ctx, func: Val, output: Val) -> Output
    where
        Ctx: CtxAccessor<'a>;
}
