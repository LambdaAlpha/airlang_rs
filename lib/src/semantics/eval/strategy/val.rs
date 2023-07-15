use crate::{
    semantics::{
        eval::strategy::{
            ByRefStrategy,
            EvalStrategy,
            FreeByRefStrategy,
            FreeStrategy,
        },
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
        Ctx,
    },
    types::{
        Call,
        Pair,
        Reverse,
        Symbol,
    },
};

pub(crate) struct ValFreeStrategy;

impl FreeStrategy for ValFreeStrategy {
    fn eval(input: Val) -> Val {
        input
    }

    fn eval_symbol(s: Symbol) -> Val {
        Val::Symbol(s)
    }

    fn eval_ref(ref_val: RefVal) -> Val {
        Val::Ref(ref_val)
    }

    fn eval_pair(first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn eval_list(list: ListVal) -> Val {
        Val::List(list)
    }

    fn eval_map(map: MapVal) -> Val {
        Val::Map(map)
    }

    fn eval_call(func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn eval_reverse(func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

pub(crate) struct ValFreeByRefStrategy;

impl FreeByRefStrategy for ValFreeByRefStrategy {
    fn eval(input: &Val) -> Val {
        input.clone()
    }

    fn eval_symbol(s: &Symbol) -> Val {
        Val::Symbol(s.clone())
    }

    fn eval_ref(ref_val: &RefVal) -> Val {
        Val::Ref(ref_val.clone())
    }

    fn eval_pair(first: &Val, second: &Val) -> Val {
        Val::Pair(Box::new(Pair::new(first.clone(), second.clone())))
    }

    fn eval_list(list: &ListVal) -> Val {
        Val::List(list.clone())
    }

    fn eval_map(map: &MapVal) -> Val {
        Val::Map(map.clone())
    }

    fn eval_call(func: &Val, input: &Val) -> Val {
        Val::Call(Box::new(Call::new(func.clone(), input.clone())))
    }

    fn eval_reverse(func: &Val, output: &Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func.clone(), output.clone())))
    }
}

pub(crate) struct ValStrategy;

impl EvalStrategy for ValStrategy {
    fn eval(_: &mut Ctx, input: Val) -> Val {
        ValFreeStrategy::eval(input)
    }

    fn eval_symbol(_: &mut Ctx, s: Symbol) -> Val {
        ValFreeStrategy::eval_symbol(s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: RefVal) -> Val {
        ValFreeStrategy::eval_ref(ref_val)
    }

    fn eval_pair(_: &mut Ctx, first: Val, second: Val) -> Val {
        ValFreeStrategy::eval_pair(first, second)
    }

    fn eval_list(_: &mut Ctx, list: ListVal) -> Val {
        ValFreeStrategy::eval_list(list)
    }

    fn eval_map(_: &mut Ctx, map: MapVal) -> Val {
        ValFreeStrategy::eval_map(map)
    }

    fn eval_call(_: &mut Ctx, func: Val, input: Val) -> Val {
        ValFreeStrategy::eval_call(func, input)
    }

    fn eval_reverse(_: &mut Ctx, func: Val, output: Val) -> Val {
        ValFreeStrategy::eval_reverse(func, output)
    }
}

pub(crate) struct ValByRefStrategy;

impl ByRefStrategy for ValByRefStrategy {
    fn eval(_: &mut Ctx, input: &Val) -> Val {
        ValFreeByRefStrategy::eval(input)
    }

    fn eval_symbol(_: &mut Ctx, s: &Symbol) -> Val {
        ValFreeByRefStrategy::eval_symbol(s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: &RefVal) -> Val {
        ValFreeByRefStrategy::eval_ref(ref_val)
    }

    fn eval_pair(_: &mut Ctx, first: &Val, second: &Val) -> Val {
        ValFreeByRefStrategy::eval_pair(first, second)
    }

    fn eval_list(_: &mut Ctx, list: &ListVal) -> Val {
        ValFreeByRefStrategy::eval_list(list)
    }

    fn eval_map(_: &mut Ctx, map: &MapVal) -> Val {
        ValFreeByRefStrategy::eval_map(map)
    }

    fn eval_call(_: &mut Ctx, func: &Val, input: &Val) -> Val {
        ValFreeByRefStrategy::eval_call(func, input)
    }

    fn eval_reverse(_: &mut Ctx, func: &Val, output: &Val) -> Val {
        ValFreeByRefStrategy::eval_reverse(func, output)
    }
}
