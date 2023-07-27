use crate::{
    semantics::{
        eval::{
            ctx::{
                free::FreeCtx,
                CtxTrait,
            },
            strategy::{
                inline::{
                    InlineByRefStrategy,
                    InlineStrategy,
                },
                ByRef,
                ByVal,
            },
            Evaluator,
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

pub(crate) struct DefaultStrategy;

impl<Ctx> Evaluator<Ctx, Val, Val> for DefaultStrategy
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<Ctx> ByVal<Ctx> for DefaultStrategy
where
    Ctx: CtxTrait,
{
    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        ctx.get(&s)
    }

    fn eval_ref(&self, _: &mut Ctx, ref_val: RefVal) -> Val {
        FreeCtx::get_val_ref(&ref_val)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineStrategy.eval_val(ctx, k);
                let value = self.eval_val(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        let Val::Func(FuncVal(func)) = self.eval_val(ctx, func) else {
            return Val::default();
        };
        func.eval(ctx, input)
    }

    fn eval_reverse(&self, _: &mut Ctx, _: Val, _: Val) -> Val {
        Val::default()
    }
}

pub(crate) struct DefaultByRefStrategy;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for DefaultByRefStrategy
where
    Ctx: CtxTrait,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        self.eval_val(ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx> for DefaultByRefStrategy
where
    Ctx: CtxTrait,
{
    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        ctx.get(s)
    }

    fn eval_ref(&self, _: &mut Ctx, ref_val: &'a RefVal) -> Val {
        FreeCtx::get_val_ref(ref_val)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        let map = map
            .into_iter()
            .map(|(k, v)| {
                let key = InlineByRefStrategy.eval_val(ctx, k);
                let value = self.eval_val(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        let Val::Func(FuncVal(func)) = self.eval_val(ctx, func) else {
            return Val::default();
        };
        func.eval(ctx, input.clone())
    }

    fn eval_reverse(&self, _: &mut Ctx, _: &'a Val, _: &'a Val) -> Val {
        Val::default()
    }
}
