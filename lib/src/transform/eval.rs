use crate::{
    ctx::DefaultCtx,
    ctx_access::CtxAccessor,
    problem::solve,
    symbol::Symbol,
    transform::id::Id,
    transformer::{
        input::ByVal,
        output::OutputBuilder,
        DefaultByVal,
        Transformer,
        ValBuilder,
    },
    val::{
        func::FuncVal,
        list::ListVal,
        map::MapVal,
        Val,
    },
};

#[derive(Copy, Clone)]
pub(crate) struct Eval;

impl Transformer<Val, Val> for Eval {
    fn transform<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl ByVal<Val> for Eval {
    fn transform_default<'a, Ctx>(&self, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol<'a, Ctx>(&self, ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultCtx.get_or_default(ctx, s)
    }

    fn transform_pair<'a, Ctx>(&self, ctx: Ctx, first: Val, second: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list<'a, Ctx>(&self, ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        DefaultByVal::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call<'a, Ctx>(&self, mut ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let func = self.transform(ctx.reborrow(), func);
        self.eval_input_then_call(ctx, func, input)
    }

    fn transform_ask<'a, Ctx>(&self, mut ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        let func = self.transform(ctx.reborrow(), func);
        self.eval_output_then_solve(ctx, func, output)
    }
}

impl Eval {
    pub(crate) fn eval_input_then_call<'a, Ctx>(&self, mut ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if let Val::Func(FuncVal(func)) = &func {
            let input = func.input_mode.transform(ctx.reborrow(), input);
            func.transform(ctx, input)
        } else {
            let input = self.transform(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    pub(crate) fn eval_input<'a, Ctx>(&self, ctx: Ctx, func: &Val, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if let Val::Func(FuncVal(func)) = func {
            func.input_mode.transform(ctx, input)
        } else {
            self.transform(ctx, input)
        }
    }

    pub(crate) fn call<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if let Val::Func(FuncVal(func)) = &func {
            func.transform(ctx, input)
        } else {
            ValBuilder.from_call(func, input)
        }
    }

    pub(crate) fn eval_output_then_solve<'a, Ctx>(
        &self,
        mut ctx: Ctx,
        func: Val,
        output: Val,
    ) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if let Val::Func(func) = func {
            let output = func.output_mode.transform(ctx.reborrow(), output);
            solve(ctx, func, output)
        } else {
            let output = self.transform(ctx, output);
            ValBuilder.from_ask(func, output)
        }
    }

    pub(crate) fn eval_output<'a, Ctx>(&self, ctx: Ctx, func: &Val, output: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if let Val::Func(FuncVal(f)) = func {
            f.output_mode.transform(ctx, output)
        } else {
            self.transform(ctx, output)
        }
    }

    pub(crate) fn solve<'a, Ctx>(ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor<'a>,
    {
        if let Val::Func(func) = func {
            solve(ctx, func, output)
        } else {
            ValBuilder.from_ask(func, output)
        }
    }
}
