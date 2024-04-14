use crate::{
    symbol::Symbol,
    transformer::Transformer,
    val::{
        list::ListVal,
        map::MapVal,
    },
    Val,
};

pub(crate) trait ByVal<Ctx, Output>: Transformer<Ctx, Val, Output> {
    fn transform_default(&self, ctx: &mut Ctx, input: Val) -> Output;

    fn transform_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Output;

    fn transform_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Output;

    fn transform_list(&self, ctx: &mut Ctx, list: ListVal) -> Output;

    fn transform_map(&self, ctx: &mut Ctx, map: MapVal) -> Output;

    fn transform_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Output;

    fn transform_ask(&self, ctx: &mut Ctx, func: Val, output: Val) -> Output;
}
