use crate::{
    semantics::{
        eval::strategy::{
            inline::{
                InlineByRefStrategy,
                InlineStrategy,
            },
            ByRefStrategy,
            EvalStrategy,
        },
        val::{
            FuncVal,
            MapVal,
            RefVal,
            Val,
        },
        Ctx,
    },
    types::{
        Keeper,
        Symbol,
    },
};

pub(crate) struct Eval;

impl Eval {
    pub(crate) fn eval_symbol(ctx: &Ctx, s: &Symbol) -> Val {
        ctx.get(s)
    }

    pub(crate) fn eval_ref(ref_val: &RefVal) -> Val {
        let Ok(input) = Keeper::reader(&ref_val.0) else {
            return Val::default();
        };
        input.val.clone()
    }
}

pub(crate) struct DefaultStrategy;

impl EvalStrategy for DefaultStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        Eval::eval_symbol(ctx, &s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: RefVal) -> Val {
        Eval::eval_ref(&ref_val)
    }

    fn eval_map(ctx: &mut Ctx, map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineStrategy::eval(ctx, k);
                let value = Self::eval(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(ctx: &mut Ctx, func: Val, input: Val) -> Val {
        let Val::Func(FuncVal(func)) = Self::eval(ctx, func) else {
            return Val::default();
        };
        func.eval(ctx, input)
    }

    fn eval_reverse(ctx: &mut Ctx, func: Val, output: Val) -> Val {
        let reverse_interpreter = ctx.reverse_interpreter.clone();
        let Some(reverse_interpreter) = reverse_interpreter else {
            return Val::default();
        };
        let reverse_func = reverse_interpreter.eval(ctx, func);
        Self::eval_call(ctx, reverse_func, output)
    }
}

pub(crate) struct DefaultByRefStrategy;

impl ByRefStrategy for DefaultByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        Eval::eval_symbol(ctx, s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: &RefVal) -> Val {
        Eval::eval_ref(ref_val)
    }

    fn eval_map(ctx: &mut Ctx, map: &MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineByRefStrategy::eval(ctx, k);
                let value = Self::eval(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        let Val::Func(FuncVal(func)) = Self::eval(ctx, func) else {
            return Val::default();
        };
        func.eval(ctx, input.clone())
    }

    fn eval_reverse(ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
        DefaultStrategy::eval_reverse(ctx, func.clone(), output.clone())
    }
}
