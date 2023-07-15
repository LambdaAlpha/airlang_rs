use crate::{
    semantics::{
        eval::{
            ctx::Ctx,
            strategy::{
                eval::{
                    DefaultByRefStrategy,
                    DefaultConstByRefStrategy,
                    DefaultConstStrategy,
                    DefaultFreeByRefStrategy,
                    DefaultFreeStrategy,
                    DefaultStrategy,
                },
                val::{
                    ValByRefStrategy,
                    ValFreeByRefStrategy,
                    ValFreeStrategy,
                    ValStrategy,
                },
                ByRefStrategy,
                EvalStrategy,
                FreeByRefStrategy,
                FreeStrategy,
            },
        },
        val::{
            RefVal,
            Val,
        },
    },
    types::Symbol,
};

pub(crate) struct InlineFreeStrategy;

impl FreeStrategy for InlineFreeStrategy {
    fn eval_symbol(s: Symbol) -> Val {
        ValFreeStrategy::eval_symbol(s)
    }

    fn eval_ref(ref_val: RefVal) -> Val {
        ValFreeStrategy::eval_ref(ref_val)
    }

    fn eval_call(func: Val, input: Val) -> Val {
        DefaultFreeStrategy::eval_call(func, input)
    }

    fn eval_reverse(func: Val, output: Val) -> Val {
        DefaultFreeStrategy::eval_reverse(func, output)
    }
}

pub(crate) struct InlineFreeByRefStrategy;

impl FreeByRefStrategy for InlineFreeByRefStrategy {
    fn eval_symbol(s: &Symbol) -> Val {
        ValFreeByRefStrategy::eval_symbol(s)
    }

    fn eval_ref(ref_val: &RefVal) -> Val {
        ValFreeByRefStrategy::eval_ref(ref_val)
    }

    fn eval_call(func: &Val, input: &Val) -> Val {
        DefaultFreeByRefStrategy::eval_call(func, input)
    }

    fn eval_reverse(func: &Val, output: &Val) -> Val {
        DefaultFreeByRefStrategy::eval_reverse(func, output)
    }
}

pub(crate) struct InlineStrategy;

impl EvalStrategy for InlineStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        ValStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: RefVal) -> Val {
        ValStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: Val, input: Val) -> Val {
        DefaultStrategy::eval_call(ctx, func, input)
    }

    fn eval_reverse(ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultStrategy::eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InlineByRefStrategy;

impl ByRefStrategy for InlineByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        ValByRefStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: &RefVal) -> Val {
        ValByRefStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        DefaultByRefStrategy::eval_call(ctx, func, input)
    }

    fn eval_reverse(ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
        DefaultByRefStrategy::eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InlineConstStrategy;

impl EvalStrategy for InlineConstStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        ValStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: RefVal) -> Val {
        ValStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: Val, input: Val) -> Val {
        DefaultConstStrategy::eval_call(ctx, func, input)
    }

    fn eval_reverse(ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultConstStrategy::eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InlineConstByRefStrategy;

impl ByRefStrategy for InlineConstByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        ValByRefStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: &RefVal) -> Val {
        ValByRefStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        DefaultConstByRefStrategy::eval_call(ctx, func, input)
    }

    fn eval_reverse(ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
        DefaultConstByRefStrategy::eval_reverse(ctx, func, output)
    }
}
