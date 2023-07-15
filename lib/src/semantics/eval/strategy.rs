use crate::{
    semantics::{
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
        Ctx,
    },
    types::{
        Pair,
        Symbol,
    },
};

pub(crate) trait FreeStrategy {
    fn eval(input: Val) -> Val {
        match input {
            Val::Symbol(s) => Self::eval_symbol(s),
            Val::Pair(p) => Self::eval_pair(p.first, p.second),
            Val::List(l) => Self::eval_list(l),
            Val::Map(m) => Self::eval_map(m),
            Val::Call(c) => Self::eval_call(c.func, c.input),
            Val::Reverse(r) => Self::eval_reverse(r.func, r.output),
            Val::Ref(k) => Self::eval_ref(k),
            v => Self::eval_atoms(v),
        }
    }

    fn eval_atoms(input: Val) -> Val {
        input
    }

    fn eval_symbol(s: Symbol) -> Val;

    fn eval_ref(ref_val: RefVal) -> Val;

    fn eval_pair(first: Val, second: Val) -> Val {
        let first = Self::eval(first);
        let second = Self::eval(second);
        let pair = Pair::new(first, second);
        Val::Pair(Box::new(pair))
    }

    fn eval_list(list: ListVal) -> Val {
        let list = list.into_iter().map(|v| Self::eval(v)).collect();
        Val::List(list)
    }

    fn eval_map(map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| (Self::eval(k), Self::eval(v)))
            .collect();
        Val::Map(map)
    }

    fn eval_call(func: Val, input: Val) -> Val;

    fn eval_reverse(func: Val, output: Val) -> Val;
}

pub(crate) trait FreeByRefStrategy {
    fn eval(input: &Val) -> Val {
        match input {
            Val::Symbol(s) => Self::eval_symbol(s),
            Val::Pair(p) => Self::eval_pair(&p.first, &p.second),
            Val::List(l) => Self::eval_list(l),
            Val::Map(m) => Self::eval_map(m),
            Val::Call(c) => Self::eval_call(&c.func, &c.input),
            Val::Reverse(r) => Self::eval_reverse(&r.func, &r.output),
            Val::Ref(k) => Self::eval_ref(k),
            v => Self::eval_atoms(v),
        }
    }

    fn eval_atoms(input: &Val) -> Val {
        input.clone()
    }

    fn eval_symbol(s: &Symbol) -> Val;

    fn eval_ref(ref_val: &RefVal) -> Val;

    fn eval_pair(first: &Val, second: &Val) -> Val {
        let first = Self::eval(first);
        let second = Self::eval(second);
        let pair = Pair::new(first, second);
        Val::Pair(Box::new(pair))
    }

    fn eval_list(list: &ListVal) -> Val {
        let list = list.into_iter().map(|v| Self::eval(v)).collect();
        Val::List(list)
    }

    fn eval_map(map: &MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| (Self::eval(k), Self::eval(v)))
            .collect();
        Val::Map(map)
    }

    fn eval_call(func: &Val, input: &Val) -> Val;

    fn eval_reverse(func: &Val, output: &Val) -> Val;
}

pub(crate) trait EvalStrategy {
    fn eval(ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(s) => Self::eval_symbol(ctx, s),
            Val::Pair(p) => Self::eval_pair(ctx, p.first, p.second),
            Val::List(l) => Self::eval_list(ctx, l),
            Val::Map(m) => Self::eval_map(ctx, m),
            Val::Call(c) => Self::eval_call(ctx, c.func, c.input),
            Val::Reverse(r) => Self::eval_reverse(ctx, r.func, r.output),
            Val::Ref(k) => Self::eval_ref(ctx, k),
            v => Self::eval_atoms(ctx, v),
        }
    }

    fn eval_atoms(_: &mut Ctx, input: Val) -> Val {
        input
    }

    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val;

    fn eval_ref(_: &mut Ctx, ref_val: RefVal) -> Val;

    fn eval_pair(ctx: &mut Ctx, first: Val, second: Val) -> Val {
        let first = Self::eval(ctx, first);
        let second = Self::eval(ctx, second);
        let pair = Pair::new(first, second);
        Val::Pair(Box::new(pair))
    }

    fn eval_list(ctx: &mut Ctx, list: ListVal) -> Val {
        let list = list.into_iter().map(|v| Self::eval(ctx, v)).collect();
        Val::List(list)
    }

    fn eval_map(ctx: &mut Ctx, map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| (Self::eval(ctx, k), Self::eval(ctx, v)))
            .collect();
        Val::Map(map)
    }

    fn eval_call(ctx: &mut Ctx, func: Val, input: Val) -> Val;

    fn eval_reverse(ctx: &mut Ctx, func: Val, output: Val) -> Val;
}

pub(crate) trait ByRefStrategy {
    fn eval(ctx: &mut Ctx, input: &Val) -> Val {
        match input {
            Val::Symbol(s) => Self::eval_symbol(ctx, s),
            Val::Pair(p) => Self::eval_pair(ctx, &p.first, &p.second),
            Val::List(l) => Self::eval_list(ctx, l),
            Val::Map(m) => Self::eval_map(ctx, m),
            Val::Call(c) => Self::eval_call(ctx, &c.func, &c.input),
            Val::Reverse(r) => Self::eval_reverse(ctx, &r.func, &r.output),
            Val::Ref(k) => Self::eval_ref(ctx, k),
            v => Self::eval_atoms(ctx, v),
        }
    }

    fn eval_atoms(_: &mut Ctx, input: &Val) -> Val {
        input.clone()
    }

    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val;

    fn eval_ref(_: &mut Ctx, ref_val: &RefVal) -> Val;

    fn eval_pair(ctx: &mut Ctx, first: &Val, second: &Val) -> Val {
        let first = Self::eval(ctx, first);
        let second = Self::eval(ctx, second);
        let pair = Pair::new(first, second);
        Val::Pair(Box::new(pair))
    }

    fn eval_list(ctx: &mut Ctx, list: &ListVal) -> Val {
        let list = list.into_iter().map(|v| Self::eval(ctx, v)).collect();
        Val::List(list)
    }

    fn eval_map(ctx: &mut Ctx, map: &MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| (Self::eval(ctx, k), Self::eval(ctx, v)))
            .collect();
        Val::Map(map)
    }

    fn eval_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val;

    fn eval_reverse(ctx: &mut Ctx, func: &Val, output: &Val) -> Val;
}

pub(crate) mod val;

pub(crate) mod interpolate;

pub(crate) mod inline;

pub(crate) mod eval;
