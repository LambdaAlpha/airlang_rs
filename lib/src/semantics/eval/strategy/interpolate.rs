use crate::{
    semantics::{
        eval::strategy::{
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

pub(crate) struct InterpolateFreeStrategy;

impl FreeStrategy for InterpolateFreeStrategy {
    fn eval_symbol(s: Symbol) -> Val {
        ValFreeStrategy::eval_symbol(s)
    }

    fn eval_ref(ref_val: RefVal) -> Val {
        ValFreeStrategy::eval_ref(ref_val)
    }

    fn eval_call(func: Val, input: Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultFreeStrategy::eval(input);
        }

        let func = Self::eval(func);
        let input = Self::eval(input);
        let call = Box::new(Call::new(func, input));
        Val::Call(call)
    }

    fn eval_reverse(func: Val, output: Val) -> Val {
        ValFreeStrategy::eval_reverse(func, output)
    }
}

pub(crate) struct InterpolateFreeByRefStrategy;

impl FreeByRefStrategy for InterpolateFreeByRefStrategy {
    fn eval_symbol(s: &Symbol) -> Val {
        ValFreeByRefStrategy::eval_symbol(s)
    }

    fn eval_ref(ref_val: &RefVal) -> Val {
        ValFreeByRefStrategy::eval_ref(ref_val)
    }

    fn eval_call(func: &Val, input: &Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultFreeByRefStrategy::eval(input);
        }

        let func = Self::eval(func);
        let input = Self::eval(input);
        let call = Box::new(Call::new(func, input));
        Val::Call(call)
    }

    fn eval_reverse(func: &Val, output: &Val) -> Val {
        ValFreeByRefStrategy::eval_reverse(func, output)
    }
}

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

pub(crate) struct InterpolateConstStrategy;

impl EvalStrategy for InterpolateConstStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: Symbol) -> Val {
        ValStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: RefVal) -> Val {
        ValStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: Val, input: Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultConstStrategy::eval(ctx, input);
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

pub(crate) struct InterpolateConstByRefStrategy;

impl ByRefStrategy for InterpolateConstByRefStrategy {
    fn eval_symbol(ctx: &mut Ctx, s: &Symbol) -> Val {
        ValByRefStrategy::eval_symbol(ctx, s)
    }

    fn eval_ref(ctx: &mut Ctx, ref_val: &RefVal) -> Val {
        ValByRefStrategy::eval_ref(ctx, ref_val)
    }

    fn eval_call(ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultConstByRefStrategy::eval(ctx, input);
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
