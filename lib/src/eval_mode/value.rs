use crate::{
    call::Call,
    eval::{
        input::{
            ByRef,
            ByVal,
        },
        DefaultByRef,
        DefaultByVal,
        Evaluator,
    },
    pair::Pair,
    reverse::Reverse,
    symbol::Symbol,
    val::{
        ListVal,
        MapVal,
        Val,
    },
};

#[derive(Copy, Clone)]
pub(crate) struct Value;

impl<Ctx> Evaluator<Ctx, Val, Val> for Value {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Value {
    fn eval_atoms(&self, _ctx: &mut Ctx, input: Val) -> Val {
        input
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, s: Symbol) -> Val {
        Val::Symbol(s)
    }

    fn eval_pair(&self, _ctx: &mut Ctx, first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn eval_list(&self, _ctx: &mut Ctx, list: ListVal) -> Val {
        Val::List(list)
    }

    fn eval_map(&self, _ctx: &mut Ctx, map: MapVal) -> Val {
        Val::Map(map)
    }

    fn eval_call(&self, _ctx: &mut Ctx, func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

#[derive(Copy, Clone)]
pub(crate) struct ValueByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for ValueByRef {
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for ValueByRef {
    fn eval_atoms(&self, _ctx: &mut Ctx, input: &'a Val) -> Val {
        input.clone()
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, s: &'a Symbol) -> Val {
        Val::Symbol(s.clone())
    }

    fn eval_pair(&self, _ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        Val::Pair(Box::new(Pair::new(first.clone(), second.clone())))
    }

    fn eval_list(&self, _ctx: &mut Ctx, list: &'a ListVal) -> Val {
        Val::List(list.clone())
    }

    fn eval_map(&self, _ctx: &mut Ctx, map: &'a MapVal) -> Val {
        Val::Map(map.clone())
    }

    fn eval_call(&self, _ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        Val::Call(Box::new(Call::new(func.clone(), input.clone())))
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func.clone(), output.clone())))
    }
}
