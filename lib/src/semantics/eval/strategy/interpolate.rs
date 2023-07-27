use crate::{
    semantics::{
        eval::{
            ctx::CtxTrait,
            strategy::{
                eval::{
                    DefaultByRefStrategy,
                    DefaultStrategy,
                },
                val::{
                    ValByRefStrategy,
                    ValStrategy,
                },
                ByRef,
                ByVal,
            },
            Evaluator,
        },
        val::{
            RefVal,
            Val,
        },
    },
    types::{
        Call,
        Symbol,
    },
};

pub(crate) struct InterpolateStrategy;

impl<Ctx> Evaluator<Ctx, Val, Val> for InterpolateStrategy
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<Ctx> ByVal<Ctx> for InterpolateStrategy
where
    Ctx: CtxTrait,
{
    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        ValStrategy.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Val {
        ValStrategy.eval_ref(ctx, ref_val)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultStrategy.eval_val(ctx, input);
        }
        let func = self.eval_val(ctx, func);
        let input = self.eval_val(ctx, input);
        let call = Box::new(Call::new(func, input));
        Val::Call(call)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        ValStrategy.eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InterpolateByRefStrategy;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for InterpolateByRefStrategy
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx> for InterpolateByRefStrategy
where
    Ctx: CtxTrait,
{
    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        ValByRefStrategy.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &'a RefVal) -> Val {
        ValByRefStrategy.eval_ref(ctx, ref_val)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        if let Val::Unit(_) = &func {
            return DefaultByRefStrategy.eval_val(ctx, input);
        }

        let func = self.eval_val(ctx, func);
        let input = self.eval_val(ctx, input);
        let call = Box::new(Call::new(func, input));
        Val::Call(call)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        ValByRefStrategy.eval_reverse(ctx, func, output)
    }
}
