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
    types::Symbol,
};

pub(crate) struct InlineStrategy;

impl<Ctx> Evaluator<Ctx, Val, Val> for InlineStrategy
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<Ctx> ByVal<Ctx> for InlineStrategy
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
        DefaultStrategy.eval_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultStrategy.eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InlineByRefStrategy;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for InlineByRefStrategy
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx> for InlineByRefStrategy
where
    Ctx: CtxTrait,
{
    fn eval_symbol(&self, ctx: &mut Ctx, s: &Symbol) -> Val {
        ValByRefStrategy.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &RefVal) -> Val {
        ValByRefStrategy.eval_ref(ctx, ref_val)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        DefaultByRefStrategy.eval_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
        DefaultByRefStrategy.eval_reverse(ctx, func, output)
    }
}
