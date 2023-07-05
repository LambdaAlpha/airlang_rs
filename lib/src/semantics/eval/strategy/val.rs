use crate::{
    semantics::{
        eval::strategy::{
            ByRefStrategy,
            EvalStrategy,
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

pub(crate) struct ValStrategy;

impl EvalStrategy for ValStrategy {
    fn eval(_: &mut Ctx, input: Val) -> Val {
        input
    }

    fn eval_symbol(_: &mut Ctx, s: Symbol) -> Val {
        Val::Symbol(s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: RefVal) -> Val {
        Val::Ref(ref_val)
    }

    fn eval_pair(_: &mut Ctx, first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn eval_list(_: &mut Ctx, list: ListVal) -> Val {
        Val::List(list)
    }

    fn eval_map(_: &mut Ctx, map: MapVal) -> Val {
        Val::Map(map)
    }

    fn eval_call(_: &mut Ctx, func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn eval_reverse(_: &mut Ctx, func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

pub(crate) struct ValByRefStrategy;

impl ByRefStrategy for ValByRefStrategy {
    fn eval(_: &mut Ctx, input: &Val) -> Val {
        input.clone()
    }

    fn eval_symbol(_: &mut Ctx, s: &Symbol) -> Val {
        Val::Symbol(s.clone())
    }

    fn eval_ref(_: &mut Ctx, ref_val: &RefVal) -> Val {
        Val::Ref(ref_val.clone())
    }

    fn eval_pair(_: &mut Ctx, first: &Val, second: &Val) -> Val {
        Val::Pair(Box::new(Pair::new(first.clone(), second.clone())))
    }

    fn eval_list(_: &mut Ctx, list: &ListVal) -> Val {
        Val::List(list.clone())
    }

    fn eval_map(_: &mut Ctx, map: &MapVal) -> Val {
        Val::Map(map.clone())
    }

    fn eval_call(_: &mut Ctx, func: &Val, input: &Val) -> Val {
        Val::Call(Box::new(Call::new(func.clone(), input.clone())))
    }

    fn eval_reverse(_: &mut Ctx, func: &Val, output: &Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func.clone(), output.clone())))
    }
}
