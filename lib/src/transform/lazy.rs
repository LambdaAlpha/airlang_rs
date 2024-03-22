use crate::{
    ctx_access::CtxAccessor,
    symbol::Symbol,
    transform::{
        eval::{
            Eval,
            EvalByRef,
        },
        id::{
            Id,
            IdByRef,
        },
    },
    transformer::{
        input::{
            ByRef,
            ByVal,
        },
        DefaultByRef,
        DefaultByVal,
        ValBuilder,
    },
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
    Transformer,
};

#[derive(Copy, Clone)]
pub(crate) struct Lazy;

impl<Ctx> Transformer<Ctx, Val, Val> for Lazy
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Lazy
where
    Ctx: CtxAccessor,
{
    fn transform_atoms(&self, ctx: &mut Ctx, input: Val) -> Val {
        Id.transform_atoms(ctx, input)
    }

    fn transform_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        Id.transform_symbol(ctx, s)
    }

    fn transform_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        if func.is_unit() {
            return Eval.transform(ctx, input);
        }
        DefaultByVal::transform_call(self, ctx, func, input, ValBuilder)
    }

    fn transform_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultByVal::transform_reverse(self, ctx, func, output, ValBuilder)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct LazyByRef;

impl<'a, Ctx> Transformer<Ctx, &'a Val, Val> for LazyByRef
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::transform_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for LazyByRef
where
    Ctx: CtxAccessor,
{
    fn transform_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        IdByRef.transform_atoms(ctx, input)
    }

    fn transform_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        IdByRef.transform_symbol(ctx, s)
    }

    fn transform_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        DefaultByRef::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        if func.is_unit() {
            return EvalByRef.transform(ctx, input);
        }
        DefaultByRef::transform_call(self, ctx, func, input, ValBuilder)
    }

    fn transform_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        DefaultByRef::transform_reverse(self, ctx, func, output, ValBuilder)
    }
}
