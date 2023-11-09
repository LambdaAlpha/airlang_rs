use crate::{
    semantics::{
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
        eval_mode::value::{
            Value,
            ValueByRef,
        },
        val::{
            FuncVal,
            ListVal,
            MapVal,
            Val,
        },
        Evaluator,
    },
    types::Symbol,
};

#[derive(Copy, Clone)]
pub(crate) struct Eval;

impl<Ctx> Evaluator<Ctx, Val, Val> for Eval
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Eval
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
        let func = self.eval(ctx, func);
        if let Val::Func(FuncVal(func)) = func {
            let input = func.input_mode.eval(ctx, input);
            func.eval(ctx, input)
        } else {
            let input = self.eval(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultByVal::eval_reverse(self, ctx, func, output, ValBuilder)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct EvalByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for EvalByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for EvalByRef
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
        let func = self.eval(ctx, func);
        if let Val::Func(FuncVal(func)) = func {
            let input = func.input_mode.eval(ctx, input);
            func.eval(ctx, input)
        } else {
            let input = self.eval(ctx, input);
            ValBuilder.from_call(func, input)
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        DefaultByRef::eval_reverse(self, ctx, func, output, ValBuilder)
    }
}
