use crate::{
    semantics::{
        ctx_access::{
            free::FreeCtx,
            CtxAccessor,
        },
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
        eval_mode::{
            value::{
                Value,
                ValueByRef,
            },
            INLINE,
            INLINE_BY_REF,
            INLINE_CONST,
            INLINE_CONST_BY_REF,
            INLINE_CONST_CHECKER,
            INLINE_CONST_CHECKER_BY_REF,
            INLINE_FREE,
            INLINE_FREE_BY_REF,
            INLINE_FREE_CHECKER,
            INLINE_FREE_CHECKER_BY_REF,
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

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Option<Val> {
        Some(Eval.eval_ref(ctx, ref_val))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Option<Val> {
        DefaultByVal::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Option<Val> {
        DefaultByVal::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Option<Val> {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_FREE.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        OpValBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Option<Val> {
        let Val::Func(FuncVal(func)) = self.eval(ctx, func)? else {
            return Some(Val::default());
        };
        if !func.is_ctx_free() {
            return None;
        }
        let input = func.input_eval_mode.eval_free(ctx, input)?;
        Some(func.eval(ctx, input))
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Option<Val> {
        Some(Eval.eval_reverse(ctx, func, output))
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

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &'a RefVal) -> Option<Val> {
        Some(EvalByRef.eval_ref(ctx, ref_val))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Option<Val> {
        DefaultByRef::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Option<Val> {
        DefaultByRef::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Option<Val> {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_FREE_BY_REF.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        OpValBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Option<Val> {
        let Val::Func(FuncVal(func)) = self.eval(ctx, func)? else {
            return Some(Val::default());
        };
        if !func.is_ctx_free() {
            return None;
        }
        let input = func.input_eval_mode.eval_free_by_ref(ctx, input)?;
        Some(func.eval(ctx, input))
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Option<Val> {
        Some(EvalByRef.eval_reverse(ctx, func, output))
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

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: RefVal) -> Option<Val> {
        Some(Eval.eval_ref(ctx, ref_val))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> Option<Val> {
        DefaultByVal::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> Option<Val> {
        DefaultByVal::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> Option<Val> {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_CONST.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        OpValBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> Option<Val> {
        let Val::Func(FuncVal(func)) = self.eval(ctx, func)? else {
            return Some(Val::default());
        };
        if !func.is_ctx_const() {
            return None;
        }
        let input = func.input_eval_mode.eval_const(ctx, input)?;
        Some(func.eval(ctx, input))
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: Val, output: Val) -> Option<Val> {
        Some(Eval.eval_reverse(ctx, func, output))
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

    fn eval_ref(&self, ctx: &mut Ctx, ref_val: &'a RefVal) -> Option<Val> {
        Some(EvalByRef.eval_ref(ctx, ref_val))
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> Option<Val> {
        DefaultByRef::eval_pair(self, ctx, first, second, &OpValBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> Option<Val> {
        DefaultByRef::eval_list(self, ctx, list, &OpValBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> Option<Val> {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_CONST_BY_REF.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        OpValBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> Option<Val> {
        let Val::Func(FuncVal(func)) = self.eval(ctx, func)? else {
            return Some(Val::default());
        };
        if !func.is_ctx_const() {
            return None;
        }
        let input = func.input_eval_mode.eval_const_by_ref(ctx, input)?;
        Some(func.eval(ctx, input))
    }

    fn eval_reverse(&self, ctx: &mut Ctx, func: &'a Val, output: &'a Val) -> Option<Val> {
        Some(EvalByRef.eval_reverse(ctx, func, output))
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

    fn eval_ref(&self, _ctx: &mut Ctx, _ref_val: RefVal) -> bool {
        true
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> bool {
        DefaultByVal::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> bool {
        DefaultByVal::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> bool {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_FREE_CHECKER.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        BoolAndBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> bool {
        let Some(func) = EvalFree.eval(ctx, func) else {
            return false;
        };
        let Val::Func(FuncVal(func)) = func else {
            return true;
        };
        if !func.is_ctx_free() {
            return false;
        }
        func.input_eval_mode.is_free(ctx, input)
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: Val, _output: Val) -> bool {
        true
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

    fn eval_ref(&self, _ctx: &mut Ctx, _ref_val: &'a RefVal) -> bool {
        true
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> bool {
        DefaultByRef::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> bool {
        DefaultByRef::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> bool {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_FREE_CHECKER_BY_REF.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        BoolAndBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> bool {
        let Some(func) = EvalFreeByRef.eval(ctx, func) else {
            return false;
        };
        let Val::Func(FuncVal(func)) = func else {
            return true;
        };
        if !func.is_ctx_free() {
            return false;
        }
        func.input_eval_mode.is_free_by_ref(ctx, input)
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: &'a Val, _output: &'a Val) -> bool {
        true
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

    fn eval_ref(&self, _ctx: &mut Ctx, _ref_val: RefVal) -> bool {
        true
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: Val, second: Val) -> bool {
        DefaultByVal::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: ListVal) -> bool {
        DefaultByVal::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: MapVal) -> bool {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_CONST_CHECKER.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        BoolAndBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: Val, input: Val) -> bool {
        let Some(func) = EvalConst.eval(ctx, func) else {
            return false;
        };
        let Val::Func(FuncVal(func)) = func else {
            return true;
        };
        if !func.is_ctx_const() {
            return false;
        }
        func.input_eval_mode.is_const(ctx, input)
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: Val, _output: Val) -> bool {
        true
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

    fn eval_ref(&self, _ctx: &mut Ctx, _ref_val: &'a RefVal) -> bool {
        true
    }

    fn eval_pair(&self, ctx: &mut Ctx, first: &'a Val, second: &'a Val) -> bool {
        DefaultByRef::eval_pair(self, ctx, first, second, &BoolAndBuilder)
    }

    fn eval_list(&self, ctx: &mut Ctx, list: &'a ListVal) -> bool {
        DefaultByRef::eval_list(self, ctx, list, &BoolAndBuilder)
    }

    fn eval_map(&self, ctx: &mut Ctx, map: &'a MapVal) -> bool {
        let map = map.into_iter().map(|(k, v)| {
            let key = INLINE_CONST_CHECKER_BY_REF.eval(ctx, k);
            let value = self.eval(ctx, v);
            (key, value)
        });
        BoolAndBuilder.from_map(map)
    }

    fn eval_call(&self, ctx: &mut Ctx, func: &'a Val, input: &'a Val) -> bool {
        let Some(func) = EvalConstByRef.eval(ctx, func) else {
            return false;
        };
        let Val::Func(FuncVal(func)) = func else {
            return true;
        };
        if !func.is_ctx_const() {
            return false;
        }
        func.input_eval_mode.is_const_by_ref(ctx, input)
    }

    fn eval_reverse(&self, _ctx: &mut Ctx, _func: &'a Val, _output: &'a Val) -> bool {
        true
    }
}
