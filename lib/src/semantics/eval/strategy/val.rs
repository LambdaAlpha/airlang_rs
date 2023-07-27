use crate::{
    semantics::{
        eval::{
            strategy::{
                ByRef,
                ByVal,
            },
            Evaluator,
        },
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
    },
    types::{
        Call,
        Pair,
        Reverse,
        Symbol,
    },
};

pub(crate) struct ValStrategy;

impl<Ctx> Evaluator<Ctx, Val, Val> for ValStrategy {
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<Ctx> ByVal<Ctx> for ValStrategy {
    fn eval_val(&self, _: &mut Ctx, input: Val) -> Val {
        input
    }

    fn eval_symbol(&self, _: &mut Ctx, s: Symbol) -> Val {
        Val::Symbol(s)
    }

    fn eval_ref(&self, _: &mut Ctx, ref_val: RefVal) -> Val {
        Val::Ref(ref_val)
    }

    fn eval_pair(&self, _: &mut Ctx, first: Val, second: Val) -> Val {
        Val::Pair(Box::new(Pair::new(first, second)))
    }

    fn eval_list(&self, _: &mut Ctx, list: ListVal) -> Val {
        Val::List(list)
    }

    fn eval_map(&self, _: &mut Ctx, map: MapVal) -> Val {
        Val::Map(map)
    }

    fn eval_call(&self, _: &mut Ctx, func: Val, input: Val) -> Val {
        Val::Call(Box::new(Call::new(func, input)))
    }

    fn eval_reverse(&self, _: &mut Ctx, func: Val, output: Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func, output)))
    }
}

pub(crate) struct ValByRefStrategy;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for ValByRefStrategy {
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx> for ValByRefStrategy {
    fn eval_val(&self, _: &mut Ctx, input: &'a Val) -> Val {
        input.clone()
    }

    fn eval_symbol(&self, _: &mut Ctx, s: &'a Symbol) -> Val {
        Val::Symbol(s.clone())
    }

    fn eval_ref(&self, _: &mut Ctx, ref_val: &'a RefVal) -> Val {
        Val::Ref(ref_val.clone())
    }

    fn eval_pair(&self, _: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        Val::Pair(Box::new(Pair::new(first.clone(), second.clone())))
    }

    fn eval_list(&self, _: &mut Ctx, list: &'a ListVal) -> Val {
        Val::List(list.clone())
    }

    fn eval_map(&self, _: &mut Ctx, map: &'a MapVal) -> Val {
        Val::Map(map.clone())
    }

    fn eval_call(&self, _: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        Val::Call(Box::new(Call::new(func.clone(), input.clone())))
    }

    fn eval_reverse(&self, _: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        Val::Reverse(Box::new(Reverse::new(func.clone(), output.clone())))
    }
}
