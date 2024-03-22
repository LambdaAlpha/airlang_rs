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
    fn transform_atoms(&self, ctx: &mut Ctx, input: Val) -> Output;

    fn transform_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Output;

    fn transform_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Output;

    fn transform_list(&self, ctx: &mut Ctx, list: ListVal) -> Output;

    fn transform_map(&self, ctx: &mut Ctx, map: MapVal) -> Output;

    fn transform_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Output;

    fn transform_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Output;
}

pub(crate) trait ByRef<'a, Ctx, Output>: Transformer<Ctx, &'a Val, Output> {
    fn transform_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Output;

    fn transform_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Output;

    fn transform_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Output;

    fn transform_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Output;

    fn transform_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Output;

    fn transform_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Output;

    fn transform_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Output;
}
