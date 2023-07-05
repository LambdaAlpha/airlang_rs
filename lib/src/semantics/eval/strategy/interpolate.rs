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
    types::{
        Call,
        Symbol,
    },
};

pub(crate) struct InterpolateStrategy;

impl EvalStrategy for InterpolateStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        ValStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: RefVal) -> Val {
        ValStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: Val, input: Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultStrategy::eval(ctx, input);
        }

        let func = Self::eval(ctx, func);
        let input = Self::eval(ctx, input);
        let call = Box::new(Call::new(func, input));
        Val::Call(call)
    }

    fn eval_reverse(ctx: &mut Ctx, func: Val, output: Val) -> Val {
        ValStrategy::eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InterpolateByRefStrategy;

impl ByRefStrategy for InterpolateByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        ValByRefStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: &RefVal) -> Val {
        ValByRefStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultByRefStrategy::eval(ctx, input);
        }

        let func = Self::eval(ctx, func);
        let input = Self::eval(ctx, input);
        let call = Box::new(Call::new(func, input));
        Val::Call(call)
    }

    fn eval_reverse(ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
        ValByRefStrategy::eval_reverse(ctx, func, output)
    }
}
