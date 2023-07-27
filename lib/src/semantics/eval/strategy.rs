use crate::{
    semantics::{
        eval::Evaluator,
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
    },
    types::{
        Pair,
        Symbol,
    },
};

pub(crate) trait ByVal<Ctx>: Evaluator<Ctx, Val, Val> {
    fn eval_val(&self, ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(s) => self.eval_symbol(ctx, s),
            Val::Pair(p) => self.eval_pair(ctx, p.first, p.second),
            Val::List(l) => self.eval_list(ctx, l),
            Val::Map(m) => self.eval_map(ctx, m),
            Val::Call(c) => self.eval_call(ctx, c.func, c.input),
            Val::Reverse(r) => self.eval_reverse(ctx, r.func, r.output),
            Val::Ref(k) => self.eval_ref(ctx, k),
            v => self.eval_atoms(ctx, v),
        }
    }

    fn eval_atoms(&self, _ctx: &mut Ctx, input: Val) -> Val {
        input
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val;

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Val;

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        let first = <Self as ByVal<Ctx>>::eval_val(self, ctx, first);
        let second = self.eval_val(ctx, second);
        let pair = Pair::new(first, second);
        Val::Pair(Box::new(pair))
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        let list = list.into_iter().map(|v| self.eval_val(ctx, v)).collect();
        Val::List(list)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = self.eval_val(ctx, k);
                let value = self.eval_val(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val;

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val;
}

pub(crate) trait ByRef<'a, Ctx>: Evaluator<Ctx, &'a Val, Val> {
    fn eval_val(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match input {
            Val::Symbol(s) => self.eval_symbol(ctx, s),
            Val::Pair(p) => self.eval_pair(ctx, &p.first, &p.second),
            Val::List(l) => self.eval_list(ctx, l),
            Val::Map(m) => self.eval_map(ctx, m),
            Val::Call(c) => self.eval_call(ctx, &c.func, &c.input),
            Val::Reverse(r) => self.eval_reverse(ctx, &r.func, &r.output),
            Val::Ref(k) => self.eval_ref(ctx, k),
            v => self.eval_atoms(ctx, v),
        }
    }

    fn eval_atoms(&self, _ctx: &mut Ctx, input: &'a Val) -> Val {
        input.clone()
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val;

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &'a RefVal) -> Val;

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        let first = self.eval_val(ctx, first);
        let second = self.eval_val(ctx, second);
        let pair = Pair::new(first, second);
        Val::Pair(Box::new(pair))
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        let list = list.into_iter().map(|v| self.eval_val(ctx, v)).collect();
        Val::List(list)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = self.eval_val(ctx, k);
                let value = self.eval_val(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val;

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val;
}

pub(crate) mod val;

pub(crate) mod interpolate;

pub(crate) mod inline;

pub(crate) mod eval;
