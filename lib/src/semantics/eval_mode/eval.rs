use crate::{
    semantics::{
        ctx::{
            free::FreeCtx,
            CtxTrait,
        },
        eval::{
            input::{
                ByRef,
                ByVal,
            },
            output::OutputBuilder,
            DefaultByRef,
            DefaultByVal,
            ValBuilder,
        },
        eval_mode::{
            value::{
                Value,
                ValueByRef,
            },
            INLINE,
            INLINE_BY_REF,
        },
        val::{
            FuncVal,
            ListVal,
            MapVal,
            RefVal,
            Val,
        },
        Evaluator,
    },
    types::Symbol,
};

#[derive(Default)]
pub(crate) struct Eval;

impl<Ctx> Evaluator<Ctx, Val, Val> for Eval
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Eval
where
    Ctx: CtxTrait,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Val {
        Value.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        ctx.get(&s)
    }

    fn eval_ref(&self, _ctx: &mut Ctx, ref_val: RefVal) -> Val {
        FreeCtx::get_val_ref(&ref_val)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::eval_pair(self, ctx, first, second, &ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::eval_list(self, ctx, list, &ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        ValBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        let Val::Func(FuncVal(func)) = self.eval(ctx, func) else {
            return Val::default();
        };
        func.eval(ctx, input)
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: Val, _output: Val) -> Val {
        Val::default()
    }
}

#[derive(Default)]
pub(crate) struct EvalByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalByRef
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for EvalByRef
where
    Ctx: CtxTrait,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        ValueByRef.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        ctx.get(s)
    }

    fn eval_ref(&self, _ctx: &mut Ctx, ref_val: &'a RefVal) -> Val {
        FreeCtx::get_val_ref(ref_val)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::eval_pair(self, ctx, first, second, &ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::eval_list(self, ctx, list, &ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_BY_REF.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        ValBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        let Val::Func(FuncVal(func)) = self.eval(ctx, func) else {
            return Val::default();
        };
        func.eval(ctx, input.clone())
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: &'a Val, _output: &'a Val) -> Val {
        Val::default()
    }
}
