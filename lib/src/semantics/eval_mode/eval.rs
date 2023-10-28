use crate::{
    semantics::{
        ctx_access::CtxAccessor,
        eval::{
            input::{
                ByRef,
                ByVal,
            },
            output::OutputBuilder,
            BoolAndBuilder,
            DefaultByRef,
            DefaultByVal,
            OpValBuilder,
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
        ctx.get(&s)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Val {
        DefaultByVal::eval_pair(self, ctx, first, second, &ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Val {
        DefaultByVal::eval_list(self, ctx, list, &ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Val {
        DefaultByVal::eval_map(self, ctx, map, &ValBuilder)
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
        DefaultByVal::eval_reverse(self, ctx, func, output, &ValBuilder)
    }
}

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
        ctx.get(s)
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Val {
        DefaultByRef::eval_pair(self, ctx, first, second, &ValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Val {
        DefaultByRef::eval_list(self, ctx, list, &ValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Val {
        DefaultByRef::eval_map(self, ctx, map, &ValBuilder)
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
        DefaultByRef::eval_reverse(self, ctx, func, output, &ValBuilder)
    }
}

pub(crate) struct EvalFree;

impl<Ctx> Evaluator<Ctx, Val, Option<Val>> for EvalFree
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Option<Val> {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Option<Val>> for EvalFree
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Option<Val> {
        Some(Eval.eval_atoms(ctx, input))
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: Symbol) -> Option<Val> {
        None
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Option<Val> {
        DefaultByVal::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Option<Val> {
        DefaultByVal::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Option<Val> {
        DefaultByVal::eval_map(self, ctx, map, &OpValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Option<Val> {
        let func = self.eval(ctx, func)?;
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_free() {
                return None;
            }
            let input = func.input_mode.eval_free(ctx, input)?;
            Some(func.eval(ctx, input))
        } else {
            let input = self.eval(ctx, input)?;
            Some(ValBuilder.from_call(func, input))
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Option<Val> {
        DefaultByVal::eval_reverse(self, ctx, func, output, &OpValBuilder)
    }
}

pub(crate) struct EvalFreeByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Option<Val>> for EvalFreeByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Option<Val> {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Option<Val>> for EvalFreeByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Option<Val> {
        Some(EvalByRef.eval_atoms(ctx, input))
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: &'a Symbol) -> Option<Val> {
        None
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Option<Val> {
        DefaultByRef::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Option<Val> {
        DefaultByRef::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Option<Val> {
        DefaultByRef::eval_map(self, ctx, map, &OpValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Option<Val> {
        let func = self.eval(ctx, func)?;
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_free() {
                return None;
            }
            let input = func.input_mode.eval_free_by_ref(ctx, input)?;
            Some(func.eval(ctx, input))
        } else {
            let input = self.eval(ctx, input)?;
            Some(ValBuilder.from_call(func, input))
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Option<Val> {
        DefaultByRef::eval_reverse(self, ctx, func, output, &OpValBuilder)
    }
}

pub(crate) struct EvalConst;

impl<Ctx> Evaluator<Ctx, Val, Option<Val>> for EvalConst
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Option<Val> {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, Option<Val>> for EvalConst
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: Val) -> Option<Val> {
        Some(Eval.eval_atoms(ctx, input))
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: Symbol) -> Option<Val> {
        Some(Eval.eval_symbol(ctx, s))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Option<Val> {
        DefaultByVal::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Option<Val> {
        DefaultByVal::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Option<Val> {
        DefaultByVal::eval_map(self, ctx, map, &OpValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Option<Val> {
        let func = self.eval(ctx, func)?;
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_const() {
                return None;
            }
            let input = func.input_mode.eval_const(ctx, input)?;
            Some(func.eval(ctx, input))
        } else {
            let input = self.eval(ctx, input)?;
            Some(ValBuilder.from_call(func, input))
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Option<Val> {
        DefaultByVal::eval_reverse(self, ctx, func, output, &OpValBuilder)
    }
}

pub(crate) struct EvalConstByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Option<Val>> for EvalConstByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Option<Val> {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, Option<Val>> for EvalConstByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, ctx: &mut Ctx, input: &'a Val) -> Option<Val> {
        Some(EvalByRef.eval_atoms(ctx, input))
    }

    fn eval_symbol(&self, ctx: &mut Ctx, s: &'a Symbol) -> Option<Val> {
        Some(EvalByRef.eval_symbol(ctx, s))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Option<Val> {
        DefaultByRef::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Option<Val> {
        DefaultByRef::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Option<Val> {
        DefaultByRef::eval_map(self, ctx, map, &OpValBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Option<Val> {
        let func = self.eval(ctx, func)?;
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_const() {
                return None;
            }
            let input = func.input_mode.eval_const_by_ref(ctx, input)?;
            Some(func.eval(ctx, input))
        } else {
            let input = self.eval(ctx, input)?;
            Some(ValBuilder.from_call(func, input))
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Option<Val> {
        DefaultByRef::eval_reverse(self, ctx, func, output, &OpValBuilder)
    }
}

pub(crate) struct EvalFreeChecker;

impl<Ctx> Evaluator<Ctx, Val, bool> for EvalFreeChecker
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> bool {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, bool> for EvalFreeChecker
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, _ctx: &mut Ctx, _input: Val) -> bool {
        true
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: Symbol) -> bool {
        false
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> bool {
        DefaultByVal::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> bool {
        DefaultByVal::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> bool {
        DefaultByVal::eval_map(self, ctx, map, &BoolAndBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> bool {
        let Some(func) = EvalFree.eval(ctx, func) else {
            return false;
        };
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_free() {
                return false;
            }
            func.input_mode.is_free(ctx, input)
        } else {
            self.eval(ctx, input)
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> bool {
        DefaultByVal::eval_reverse(self, ctx, func, output, &BoolAndBuilder)
    }
}

pub(crate) struct EvalFreeCheckerByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, bool> for EvalFreeCheckerByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> bool {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, bool> for EvalFreeCheckerByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, _ctx: &mut Ctx, _input: &'a Val) -> bool {
        true
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: &'a Symbol) -> bool {
        false
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> bool {
        DefaultByRef::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> bool {
        DefaultByRef::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> bool {
        DefaultByRef::eval_map(self, ctx, map, &BoolAndBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> bool {
        let Some(func) = EvalFreeByRef.eval(ctx, func) else {
            return false;
        };
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_free() {
                return false;
            }
            func.input_mode.is_free_by_ref(ctx, input)
        } else {
            self.eval(ctx, input)
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> bool {
        DefaultByRef::eval_reverse(self, ctx, func, output, &BoolAndBuilder)
    }
}

pub(crate) struct EvalConstChecker;

impl<Ctx> Evaluator<Ctx, Val, bool> for EvalConstChecker
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> bool {
        DefaultByVal::eval_val(self, ctx, input)
    }
}

impl<Ctx> ByVal<Ctx, bool> for EvalConstChecker
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, _ctx: &mut Ctx, _input: Val) -> bool {
        true
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: Symbol) -> bool {
        true
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> bool {
        DefaultByVal::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> bool {
        DefaultByVal::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> bool {
        DefaultByVal::eval_map(self, ctx, map, &BoolAndBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> bool {
        let Some(func) = EvalConst.eval(ctx, func) else {
            return false;
        };
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_const() {
                return false;
            }
            func.input_mode.is_const(ctx, input)
        } else {
            self.eval(ctx, input)
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> bool {
        DefaultByVal::eval_reverse(self, ctx, func, output, &BoolAndBuilder)
    }
}

pub(crate) struct EvalConstCheckerByRef;

impl<'a, Ctx> Evaluator<Ctx, &'a Val, bool> for EvalConstCheckerByRef
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> bool {
        DefaultByRef::eval_val(self, ctx, input)
    }
}

impl<'a, Ctx> ByRef<'a, Ctx, bool> for EvalConstCheckerByRef
where
    Ctx: CtxAccessor,
{
    fn eval_atoms(&self, _ctx: &mut Ctx, _input: &'a Val) -> bool {
        true
    }

    fn eval_symbol(&self, _ctx: &mut Ctx, _s: &'a Symbol) -> bool {
        true
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> bool {
        DefaultByRef::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> bool {
        DefaultByRef::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> bool {
        DefaultByRef::eval_map(self, ctx, map, &BoolAndBuilder)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> bool {
        let Some(func) = EvalConstByRef.eval(ctx, func) else {
            return false;
        };
        if let Val::Func(FuncVal(func)) = func {
            if !func.is_ctx_const() {
                return false;
            }
            func.input_mode.is_const_by_ref(ctx, input)
        } else {
            self.eval(ctx, input)
        }
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> bool {
        DefaultByRef::eval_reverse(self, ctx, func, output, &BoolAndBuilder)
    }
}
