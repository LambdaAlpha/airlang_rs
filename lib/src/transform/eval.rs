use crate::{
    ctx_access::CtxAccessor,
    problem::solve,
    symbol::Symbol,
    transform::id::{
        Id,
        IdByRef,
    },
    transformer::{
        input::{
            ByRef,
            ByVal,
        },
        output::OutputBuilder,
        DefaultByRef,
        DefaultByVal,
        ValBuilder,
    },
    val::{
        func::FuncVal,
        list::ListVal,
        map::MapVal,
        Val,
    },
    Transformer,
};

#[derive(Copy, Clone)]
pub(crate) struct Eval;

impl<Ctx> Transformer<Ctx, Val, Val> for Eval
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::transform_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Eval
where
    Ctx: CtxAccessor,
{
    fn transform_default(&self, ctx: &mut Ctx, input: Val) -> Val {
        Id.transform_default(ctx, input)
    }

    fn transform_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        ctx.get(&s).unwrap_or_default()
    }

    fn transform_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        let func = self.transform(ctx, func);
        self.eval_input_then_call(ctx, func, input)
    }

    fn transform_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        let func = self.transform(ctx, func);
        self.eval_output_then_solve(ctx, func, output)
    }
}

impl Eval {
    pub(crate) fn eval_input_then_call<Ctx>(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = &func {
            let input = func.input_mode.transform(ctx, input);
            func.transform(ctx, input)
        } else {
            let input = self.transform(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    pub(crate) fn eval_input<Ctx>(&self, ctx: &mut Ctx, func: &Val, input: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = func {
            func.input_mode.transform(ctx, input)
        } else {
            self.transform(ctx, input)
        }
    }

    pub(crate) fn call<Ctx>(ctx: &mut Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = &func {
            func.transform(ctx, input)
        } else {
            ValBuilder.from_call(func, input)
        }
    }

    pub(crate) fn eval_output_then_solve<Ctx>(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(func) = func {
            let output = func.output_mode.transform(ctx, output);
            solve(ctx, func, output)
        } else {
            let output = self.transform(ctx, output);
            ValBuilder.from_reverse(func, output)
        }
    }

    pub(crate) fn eval_output<Ctx>(&self, ctx: &mut Ctx, func: &Val, output: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(f)) = func {
            f.output_mode.transform(ctx, output)
        } else {
            self.transform(ctx, output)
        }
    }

    pub(crate) fn solve<Ctx>(ctx: &mut Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(func) = func {
            solve(ctx, func, output)
        } else {
            ValBuilder.from_reverse(func, output)
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EvalByRef;

impl<'a, Ctx> Transformer<Ctx, &'a Val, Val> for EvalByRef
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::transform_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for EvalByRef
where
    Ctx: CtxAccessor,
{
    fn transform_default(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        IdByRef.transform_default(ctx, input)
    }

    fn transform_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        ctx.get(s).unwrap_or_default()
    }

    fn transform_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::transform_pair(self, ctx, first, second, ValBuilder)
    }

    fn transform_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::transform_list(self, ctx, list, ValBuilder)
    }

    fn transform_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        DefaultByRef::transform_map(self, ctx, map, ValBuilder)
    }

    fn transform_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        let func = self.transform(ctx, func);
        self.eval_input_then_call(ctx, func, input)
    }

    fn transform_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        let func = self.transform(ctx, func);
        self.eval_output_then_solve(ctx, func, output)
    }
}

impl EvalByRef {
    pub(crate) fn eval_input_then_call<Ctx>(&self, ctx: &mut Ctx, func: Val, input: &Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = &func {
            let input = func.input_mode.transform(ctx, input);
            func.transform(ctx, input)
        } else {
            let input = self.transform(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    #[allow(unused)]
    pub(crate) fn eval_input<Ctx>(&self, ctx: &mut Ctx, func: &Val, input: &Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = func {
            func.input_mode.transform(ctx, input)
        } else {
            self.transform(ctx, input)
        }
    }

    pub(crate) fn eval_output_then_solve<Ctx: CtxAccessor>(
        &self,
        ctx: &mut Ctx,
        func: Val,
        output: &Val,
    ) -> Val {
        if let Val::Func(func) = func {
            let output = func.output_mode.transform(ctx, output);
            solve(ctx, func, output)
        } else {
            let output = self.transform(ctx, output);
            ValBuilder.from_reverse(func, output)
        }
    }

    #[allow(unused)]
    pub(crate) fn eval_output<Ctx>(&self, ctx: &mut Ctx, func: &Val, output: &Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(f)) = func {
            f.output_mode.transform(ctx, output)
        } else {
            self.transform(ctx, output)
        }
    }
}
