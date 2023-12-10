use crate::{
    eval::Evaluator,
    symbol::Symbol,
    val::{
        list::ListVal,
        map::MapVal,
    },
    Val,
};

pub(crate) trait ByVal<Ctx, Output>: Evaluator<Ctx, Val, Output> {
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Output;

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Output;

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Output;

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Output;

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Output;

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Output;

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Output;
}

pub(crate) trait ByRef<'a, Ctx, Output>: Evaluator<Ctx, &'a Val, Output> {
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Output;

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Output;

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Output;

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Output;

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Output;

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Output;

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Output;
}
