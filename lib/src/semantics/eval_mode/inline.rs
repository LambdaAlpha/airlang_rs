use crate::{
    semantics::{
        ctx::CtxTrait,
        eval::{
            input::{
                ByRef,
                ByVal,
            },
            DefaultByRef,
            DefaultByVal,
            ValBuilder,
        },
        eval_mode::{
            eval::{
                Eval,
                EvalByRef,
            },
            value::{
                Value,
                ValueByRef,
            },
        },
        val::{
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
        Evaluator,
    },
    types::Symbol,
};

pub(crate) struct Inline;

impl<Ctx> Evaluator<Ctx, Val, Val> for Inline
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Inline
where
    Ctx: CtxTrait,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Val {
        Value.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        Value.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Val {
        Value.eval_ref(ctx, ref_val)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::eval_pair::<_, _, _, ValBuilder>(self, ctx, first, second)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::eval_list::<_, _, _, ValBuilder>(self, ctx, list)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::eval_map::<_, _, _, ValBuilder>(self, ctx, map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        Eval.eval_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        Eval.eval_reverse(ctx, func, output)
    }
}

pub(crate) struct InlineByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for InlineByRef
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for InlineByRef
where
    Ctx: CtxTrait,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        ValueByRef.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &Symbol) -> Val {
        ValueByRef.eval_symbol(ctx, s)
    }

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &RefVal) -> Val {
        ValueByRef.eval_ref(ctx, ref_val)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::eval_pair::<_, _, _, ValBuilder>(self, ctx, first, second)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::eval_list::<_, _, _, ValBuilder>(self, ctx, list)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        DefaultByRef::eval_map::<_, _, _, ValBuilder>(self, ctx, map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &Val, input: &Val) -> Val {
        EvalByRef.eval_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &Val, output: &Val) -> Val {
        EvalByRef.eval_reverse(ctx, func, output)
    }
}
