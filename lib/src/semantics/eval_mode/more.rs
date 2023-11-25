use crate::{
    semantics::{
        ctx::names::SOLVER,
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
                }
            }
            _ => {}
        }
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
        let reverse = DefaultByVal::eval_reverse(self, ctx, func, output, ValBuilder);
        let Ok(meta) = ctx.get_meta() else {
            return reverse;
        };
        let Ok(solver) = meta.get(SOLVER) else {
            return reverse;
        };
        let Val::Func(FuncVal(solver)) = solver else {
            return Val::default();
        };
        solver.eval(ctx, reverse)
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
                }
            }
            _ => {}
        }
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
        let reverse = DefaultByRef::eval_reverse(self, ctx, func, output, ValBuilder);
        let Ok(meta) = ctx.get_meta() else {
            return reverse;
        };
        let Ok(solver) = meta.get(SOLVER) else {
            return reverse;
        };
        let Val::Func(FuncVal(solver)) = solver else {
            return Val::default();
        };
        solver.eval(ctx, reverse)
    }
}
