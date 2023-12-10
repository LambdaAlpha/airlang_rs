use crate::{
    ctx_access::CtxAccessor,
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
        more::{
            More,
            MoreByRef,
        },
        value::{
            Value,
            ValueByRef,
        },
    },
    symbol::Symbol,
    val::{
        ListVal,
        MapVal,
        Val,
    },
    Evaluator,
};

#[derive(Copy, Clone)]
pub(crate) struct Less;

impl<Ctx> Evaluator<Ctx, Val, Val> for Less
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Val> for Less
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Val {
        Value.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Val {
        Value.eval_symbol(ctx, s)
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
            Val::Unit(_) => input,
            Val::Bool(b) => {
                if b.bool() {
                    More.eval(ctx, input)
                } else {
                    self.eval(ctx, input)
                }
            }
            _ => DefaultByVal::eval_call(self, ctx, func, input, ValBuilder),
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Val {
        DefaultByVal::eval_reverse(self, ctx, func, output, ValBuilder)
    }
}

#[derive(Copy, Clone)]
pub(crate) struct LessByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for LessByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Val> for LessByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        ValueByRef.eval_atoms(ctx, input)
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Val {
        ValueByRef.eval_symbol(ctx, s)
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
            Val::Unit(_) => ValueByRef.eval(ctx, input),
            Val::Bool(b) => {
                if b.bool() {
                    MoreByRef.eval(ctx, input)
                } else {
                    self.eval(ctx, input)
                }
            }
            _ => DefaultByRef::eval_call(self, ctx, func, input, ValBuilder),
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Val {
        DefaultByRef::eval_reverse(self, ctx, func, output, ValBuilder)
    }
}
