use crate::{
    semantics::{
        eval::{
            ctx::Ctx,
            ctx_free::CtxFree,
            strategy::{
                inline::{
                    InlineByRefStrategy,
                    InlineConstByRefStrategy,
                    InlineConstStrategy,
                    InlineFreeByRefStrategy,
                    InlineFreeStrategy,
                    InlineStrategy,
                },
                ByRefStrategy,
                EvalStrategy,
                FreeByRefStrategy,
                FreeStrategy,
            },
        },
        val::{
            FuncVal,
            MapVal,
            RefVal,
            Val,
        },
    },
    types::Symbol,
};

pub(crate) struct DefaultFreeStrategy;

impl FreeStrategy for DefaultFreeStrategy {
    fn eval_symbol(_: Symbol) -> Val {
        Val::default()
    }

    fn eval_ref(ref_val: RefVal) -> Val {
        CtxFree::get(&ref_val)
    }

    fn eval_map(map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineFreeStrategy::eval(k);
                let value = Self::eval(v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(func: Val, input: Val) -> Val {
        let Val::Func(FuncVal(func)) = Self::eval(func) else {
            return Val::default();
        };
        func.eval_free(input)
    }

    fn eval_reverse(_: Val, _: Val) -> Val {
        Val::default()
    }
}

pub(crate) struct DefaultFreeByRefStrategy;

impl FreeByRefStrategy for DefaultFreeByRefStrategy {
    fn eval_symbol(_: &Symbol) -> Val {
        Val::default()
    }

    fn eval_ref(ref_val: &RefVal) -> Val {
        CtxFree::get(ref_val)
    }

    fn eval_map(map: &MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineFreeByRefStrategy::eval(k);
                let value = Self::eval(v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(func: &Val, input: &Val) -> Val {
        let Val::Func(FuncVal(func)) = Self::eval(func) else {
            return Val::default();
        };
        func.eval_free(input.clone())
    }

    fn eval_reverse(_: &Val, _: &Val) -> Val {
        Val::default()
    }
}

pub(crate) struct DefaultStrategy;

impl EvalStrategy for DefaultStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        ctx.get(&s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: RefVal) -> Val {
        CtxFree::get(&ref_val)
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

    fn eval_reverse(_: &mut Ctx, _: Val, _: Val) -> Val {
        Val::default()
    }
}

pub(crate) struct DefaultByRefStrategy;

impl ByRefStrategy for DefaultByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        ctx.get(s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: &RefVal) -> Val {
        CtxFree::get(ref_val)
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

    fn eval_reverse(_: &mut Ctx, _: &Val, _: &Val) -> Val {
        Val::default()
    }
}

pub(crate) struct DefaultConstStrategy;

impl EvalStrategy for DefaultConstStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        ctx.get(&s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: RefVal) -> Val {
        CtxFree::get(&ref_val)
    }

    fn eval_map(ctx: &mut Ctx, map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineConstStrategy::eval(ctx, k);
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
        func.eval_const(ctx, input)
    }

    fn eval_reverse(_: &mut Ctx, _: Val, _: Val) -> Val {
        Val::default()
    }
}

pub(crate) struct DefaultConstByRefStrategy;

impl ByRefStrategy for DefaultConstByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        ctx.get(s)
    }

    fn eval_ref(_: &mut Ctx, ref_val: &RefVal) -> Val {
        CtxFree::get(ref_val)
    }

    fn eval_map(ctx: &mut Ctx, map: &MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineConstByRefStrategy::eval(ctx, k);
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

    fn eval_reverse(_: &mut Ctx, _: &Val, _: &Val) -> Val {
        Val::default()
    }
}
