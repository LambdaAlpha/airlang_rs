use crate::{
    ctx_access::CtxAccessor,
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
        less::{
            Less,
            LessByRef,
        },
        value::{
            Value,
            ValueByRef,
        },
    },
    problem::solve,
    symbol::Symbol,
    val::{
        func::FuncVal,
        list::ListVal,
        map::MapVal,
        Val,
    },
    Evaluator,
};

#[derive(Copy, Clone)]
pub(crate) struct More;

impl<Ctx> Evaluator<Ctx, Val, Val> for More
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for More
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Val {
        Value.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        ctx.get(&s).unwrap_or_default()
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::eval_pair(self, ctx, first, second, ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::eval_list(self, ctx, list, ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::eval_map(self, ctx, map, ValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val {
        match &func {
            Val::Unit(_) => return Value.eval(ctx, input),
            Val::Bool(b) => {
                return if b.bool() {
                    self.eval(ctx, input)
                } else {
                    Less.eval(ctx, input)
                };
            }
            _ => {}
        }
        let func = self.eval(ctx, func);
        self.eval_input_then_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        let func = self.eval(ctx, func);
        self.eval_output_then_solve(ctx, func, output)
    }
}

impl More {
    pub(crate) fn eval_input_then_call<Ctx>(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = &func {
            let input = func.input_mode.eval(ctx, input);
            func.eval(ctx, input)
        } else {
            let input = self.eval(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    pub(crate) fn eval_input<Ctx>(&self, ctx: &mut Ctx, func: &Val, input: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = func {
            func.input_mode.eval(ctx, input)
        } else {
            self.eval(ctx, input)
        }
    }

    pub(crate) fn call<Ctx>(&self, ctx: &mut Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = &func {
            func.eval(ctx, input)
        } else {
            ValBuilder.from_call(func, input)
        }
    }

    pub(crate) fn eval_output_then_solve<Ctx>(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        let output = if let Val::Func(FuncVal(f)) = &func {
            f.output_mode.eval(ctx, output)
        } else {
            self.eval(ctx, output)
        };
        let reverse = ValBuilder.from_reverse(func, output);
        solve(ctx, reverse)
    }

    pub(crate) fn eval_output<Ctx>(&self, ctx: &mut Ctx, func: &Val, output: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(f)) = func {
            f.output_mode.eval(ctx, output)
        } else {
            self.eval(ctx, output)
        }
    }

    pub(crate) fn solve<Ctx>(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        let reverse = ValBuilder.from_reverse(func, output);
        solve(ctx, reverse)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct MoreByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for MoreByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for MoreByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        ValueByRef.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        ctx.get(s).unwrap_or_default()
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::eval_pair(self, ctx, first, second, ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::eval_list(self, ctx, list, ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        DefaultByRef::eval_map(self, ctx, map, ValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Val {
        match &func {
            Val::Unit(_) => return ValueByRef.eval(ctx, input),
            Val::Bool(b) => {
                return if b.bool() {
                    self.eval(ctx, input)
                } else {
                    LessByRef.eval(ctx, input)
                };
            }
            _ => {}
        }
        let func = self.eval(ctx, func);
        self.eval_input_then_call(ctx, func, input)
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        let func = self.eval(ctx, func);
        self.eval_func_reverse(ctx, func, output)
    }
}

impl MoreByRef {
    pub(crate) fn eval_input_then_call<Ctx>(&self, ctx: &mut Ctx, func: Val, input: &Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = &func {
            let input = func.input_mode.eval(ctx, input);
            func.eval(ctx, input)
        } else {
            let input = self.eval(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    #[allow(unused)]
    pub(crate) fn eval_input<Ctx>(&self, ctx: &mut Ctx, func: &Val, input: &Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(func)) = func {
            func.input_mode.eval(ctx, input)
        } else {
            self.eval(ctx, input)
        }
    }

    pub(crate) fn eval_func_reverse<Ctx: CtxAccessor>(
        &self,
        ctx: &mut Ctx,
        func: Val,
        output: &Val,
    ) -> Val {
        let output = if let Val::Func(FuncVal(f)) = &func {
            f.output_mode.eval(ctx, output)
        } else {
            self.eval(ctx, output)
        };
        let reverse = ValBuilder.from_reverse(func, output);
        solve(ctx, reverse)
    }

    #[allow(unused)]
    pub(crate) fn eval_output<Ctx>(&self, ctx: &mut Ctx, func: &Val, output: &Val) -> Val
    where
        Ctx: CtxAccessor,
    {
        if let Val::Func(FuncVal(f)) = func {
            f.output_mode.eval(ctx, output)
        } else {
            self.eval(ctx, output)
        }
    }
}
