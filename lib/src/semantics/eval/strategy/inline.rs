use crate::{
    semantics::{
        eval::strategy::{
            eval::{
                DefaultByRefStrategy,
                DefaultStrategy,
            },
            val::{
                ValByRefStrategy,
                ValStrategy,
            },
            ByRefStrategy,
            EvalStrategy,
        },
        val::{
            RefVal,
            Val,
        },
        Ctx,
    },
    types::Symbol,
};

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
