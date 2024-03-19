use crate::{
    ctx_access::CtxAccessor,
    eval::{
        input::{
            ByRef,
            ByVal,
        },
        DefaultByRef,
        DefaultByVal,
        ValBuilder,
    },
    eval_mode::{
        eager::{
            Eager,
            EagerByRef,
        },
        id::{
            Id,
            IdByRef,
        },
    },
    symbol::Symbol,
    val::{
        list::ListVal,
        map::MapVal,
        Val,
    },
    Evaluator,
};

#[derive(Copy, Clone)]
pub(crate) struct Lazy;

impl<Ctx> Evaluator<Ctx, Val, Val> for Lazy
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Lazy
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Val {
        Id.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        Id.eval_symbol(ctx, s)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::eval_pair(self, ctx, first, second, ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::eval_list(self, ctx, list, ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::eval_map(self, ctx, map, ValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        if func.is_unit() {
            return Eager.eval(ctx, input);
        }
        DefaultByVal::eval_call(self, ctx, func, input, ValBuilder)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultByVal::eval_reverse(self, ctx, func, output, ValBuilder)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct LazyByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for LazyByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for LazyByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        IdByRef.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        IdByRef.eval_symbol(ctx, s)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::eval_pair(self, ctx, first, second, ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::eval_list(self, ctx, list, ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        DefaultByRef::eval_map(self, ctx, map, ValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        if func.is_unit() {
            return EagerByRef.eval(ctx, input);
        }
        DefaultByRef::eval_call(self, ctx, func, input, ValBuilder)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        DefaultByRef::eval_reverse(self, ctx, func, output, ValBuilder)
    }
}
