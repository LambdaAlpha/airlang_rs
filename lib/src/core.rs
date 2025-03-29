use std::{
    borrow::Borrow,
    hash::Hash,
};

use crate::{
    Call,
    CallVal,
    Change,
    ChangeVal,
    ConstCtx,
    Ctx,
    FuncVal,
    List,
    ListVal,
    Map,
    MapVal,
    MutCtx,
    MutFnCtx,
    Optimize,
    OptimizeVal,
    Pair,
    PairVal,
    Solve,
    SolveVal,
    Symbol,
    Val,
    abstract1::Abstract,
    advisor::{
        optimize,
        solve,
    },
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    func::FuncTrait,
    mode::symbol::{
        LITERAL_CHAR,
        MOVE_CHAR,
        REF_CHAR,
    },
    transformer::{
        ByVal,
        Transformer,
    },
    val::abstract1::AbstractVal,
};

pub(crate) struct FormCore;

impl FormCore {
    pub(crate) fn transform_val<'a, Ctx, T>(t: &T, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: ByVal<Val>, {
        match input {
            Val::Symbol(s) => t.transform_symbol(ctx, s),
            Val::Pair(p) => t.transform_pair(ctx, p),
            Val::Change(c) => t.transform_change(ctx, c),
            Val::Call(c) => t.transform_call(ctx, c),
            Val::Optimize(o) => t.transform_optimize(ctx, o),
            Val::Solve(s) => t.transform_solve(ctx, s),
            Val::Abstract(a) => t.transform_abstract(ctx, a),
            Val::List(l) => t.transform_list(ctx, l),
            Val::Map(m) => t.transform_map(ctx, m),
            v => t.transform_default(ctx, v),
        }
    }

    pub(crate) fn transform_symbol<'a, const DEFAULT: char, Ctx>(ctx: Ctx, s: Symbol) -> Val
    where Ctx: CtxMeta<'a> {
        let (mode, s) = match s.chars().next() {
            Some(LITERAL_CHAR) => (LITERAL_CHAR, Symbol::from_str(&s[1 ..])),
            Some(REF_CHAR) => (REF_CHAR, Symbol::from_str(&s[1 ..])),
            Some(MOVE_CHAR) => (MOVE_CHAR, Symbol::from_str(&s[1 ..])),
            _ => (DEFAULT, s),
        };
        match mode {
            LITERAL_CHAR => Val::Symbol(s),
            REF_CHAR => DefaultCtx::get_or_default(ctx, s),
            MOVE_CHAR => DefaultCtx::remove_or_default(ctx, s),
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }

    pub(crate) fn transform_pair<'a, Ctx, First, Second>(
        first: &First, second: &Second, mut ctx: Ctx, pair: PairVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        First: Transformer<Val, Val>,
        Second: Transformer<Val, Val>, {
        let pair = Pair::from(pair);
        let first = first.transform(ctx.reborrow(), pair.first);
        let second = second.transform(ctx, pair.second);
        Val::Pair(Pair::new(first, second).into())
    }

    pub(crate) fn transform_change<'a, Ctx, From, To>(
        from: &From, to: &To, mut ctx: Ctx, change: ChangeVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        From: Transformer<Val, Val>,
        To: Transformer<Val, Val>, {
        let change = Change::from(change);
        let from = from.transform(ctx.reborrow(), change.from);
        let to = to.transform(ctx, change.to);
        Val::Change(Change::new(from, to).into())
    }

    pub(crate) fn transform_call<'a, Ctx, Func, Input>(
        func: &Func, input: &Input, mut ctx: Ctx, call: CallVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>, {
        let call = Call::from(call);
        let func = func.transform(ctx.reborrow(), call.func);
        let input = input.transform(ctx, call.input);
        Val::Call(Call::new(func, input).into())
    }

    pub(crate) fn transform_optimize<'a, Ctx, Func, Input>(
        func: &Func, input: &Input, mut ctx: Ctx, optimize: OptimizeVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>, {
        let optimize = Optimize::from(optimize);
        let func = func.transform(ctx.reborrow(), optimize.func);
        let input = input.transform(ctx, optimize.input);
        Val::Optimize(Optimize::new(func, input).into())
    }

    pub(crate) fn transform_solve<'a, Ctx, Func, Output>(
        func: &Func, output: &Output, mut ctx: Ctx, solve: SolveVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Output: Transformer<Val, Val>, {
        let solve = Solve::from(solve);
        let func = func.transform(ctx.reborrow(), solve.func);
        let output = output.transform(ctx, solve.output);
        Val::Solve(Solve::new(func, output).into())
    }

    pub(crate) fn transform_abstract<'a, Ctx, Value>(
        value: &Value, mut ctx: Ctx, abstract1: AbstractVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Value: Transformer<Val, Val>, {
        let abstract1 = Abstract::from(abstract1);
        let value = value.transform(ctx.reborrow(), abstract1.value);
        Val::Abstract(Abstract::new(value).into())
    }

    pub(crate) fn transform_list<'a, Ctx, Item>(item: &Item, mut ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        Item: Transformer<Val, Val>, {
        let list: List<Val> =
            List::from(list).into_iter().map(|v| item.transform(ctx.reborrow(), v)).collect();
        Val::List(list.into())
    }

    pub(crate) fn transform_list_head_tail<'a, Ctx, Head, Tail>(
        head: &List<Head>, tail: &Tail, mut ctx: Ctx, list: ListVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Head: Transformer<Val, Val>,
        Tail: Transformer<Val, Val>, {
        let mut iter = List::from(list).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for mode in head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = mode.transform(ctx.reborrow(), val);
            list.push(val);
        }
        for val in iter {
            list.push(tail.transform(ctx.reborrow(), val));
        }
        Val::List(List::from(list).into())
    }

    pub(crate) fn transform_map<'a, Ctx, K, V>(
        key: &K, value: &V, mut ctx: Ctx, map: MapVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        K: Transformer<Val, Val>,
        V: Transformer<Val, Val>, {
        let map: Map<Val, Val> = Map::from(map)
            .into_iter()
            .map(|(k, v)| {
                let key = key.transform(ctx.reborrow(), k);
                let value = value.transform(ctx.reborrow(), v);
                (key, value)
            })
            .collect();
        Val::Map(map.into())
    }

    pub(crate) fn transform_map_some_else<'a, Ctx, SomeK, SomeV, ElseK, ElseV>(
        some: &Map<SomeK, SomeV>, key: &ElseK, value: &ElseV, mut ctx: Ctx, map: MapVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        SomeK: Borrow<Val> + Eq + Hash,
        SomeV: Transformer<Val, Val>,
        ElseK: Transformer<Val, Val>,
        ElseV: Transformer<Val, Val>, {
        let map: Map<Val, Val> = Map::from(map)
            .into_iter()
            .map(|(k, v)| {
                if let Some(mode) = some.get(&k) {
                    let v = mode.transform(ctx.reborrow(), v);
                    (k, v)
                } else {
                    let k = key.transform(ctx.reborrow(), k);
                    let v = value.transform(ctx.reborrow(), v);
                    (k, v)
                }
            })
            .collect();
        Val::Map(map.into())
    }
}

pub(crate) struct EvalCore;

impl EvalCore {
    pub(crate) fn transform_call<'a, Ctx, Func, Input>(
        func_trans: &Func, input_trans: &Input, mut ctx: Ctx, call: CallVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>, {
        let call = Call::from(call);
        let func = func_trans.transform(ctx.reborrow(), call.func);
        Self::eval_input_then_call(input_trans, ctx, func, call.input)
    }

    pub(crate) fn eval_input_then_call<'a, Ctx, Input>(
        input_trans: &Input, mut ctx: Ctx, func: Val, input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>, {
        match func {
            Val::Func(func) => {
                let input = func.mode().call.transform(ctx.reborrow(), input);
                func.transform(ctx, input)
            }
            Val::Symbol(func) => Self::with_ref(ctx, func, |func, ctx, is_const| {
                let input = Self::call_ref_eval_input(func, ctx, is_const, input);
                Self::call_ref(func, ctx, is_const, input)
            }),
            _ => {
                let input = input_trans.transform(ctx, input);
                Val::Call(Call::new(func, input).into())
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn call_eval_input<'a, Ctx, Input>(
        input_trans: &Input, ctx: Ctx, func: &Val, input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>, {
        match func {
            Val::Func(func) => func.mode().call.transform(ctx, input),
            Val::Symbol(func) => Self::with_ref(ctx, func.clone(), |func, ctx, is_const| {
                Self::call_ref_eval_input(func, ctx, is_const, input)
            }),
            _ => input_trans.transform(ctx, input),
        }
    }

    fn call_ref_eval_input(func: &FuncVal, ctx: &mut Ctx, is_const: bool, input: Val) -> Val {
        if is_const {
            func.mode().call.transform(ConstCtx::new(ctx), input)
        } else {
            func.mode().call.transform(MutCtx::new(ctx), input)
        }
    }

    pub(crate) fn call<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        match func {
            Val::Func(func) => func.transform(ctx, input),
            Val::Symbol(func) => Self::with_ref(ctx, func, |func, ctx, is_const| {
                Self::call_ref(func, ctx, is_const, input)
            }),
            _ => Val::Call(Call::new(func, input).into()),
        }
    }

    fn call_ref(func: &mut FuncVal, ctx: &mut Ctx, is_const: bool, input: Val) -> Val {
        if is_const {
            func.transform(ConstCtx::new(ctx), input)
        } else {
            func.transform_mut(MutCtx::new(ctx), input)
        }
    }

    pub(crate) fn with_ref<'a, Ctx>(
        ctx: Ctx, func_name: Symbol,
        f: impl FnOnce(&mut FuncVal, &mut crate::Ctx, bool /*is_const*/) -> Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let (ctx, is_const) = match ctx.for_mut_fn() {
            MutFnCtx::Free(_) => return Val::default(),
            MutFnCtx::Const(ctx) => (ctx.unwrap(), true),
            MutFnCtx::Mut(ctx) => (ctx.unwrap(), false),
        };
        let variables = ctx.variables_mut();
        let Some(ctx_value) = variables.remove_unchecked(&func_name) else {
            return Val::default();
        };
        let Val::Func(mut func) = ctx_value.val else {
            variables.put_unchecked(func_name, ctx_value);
            return Val::default();
        };
        let output = f(&mut func, ctx, is_const);
        let ctx_value = CtxValue { val: Val::Func(func), ..ctx_value };
        ctx.variables_mut().put_unchecked(func_name, ctx_value);
        output
    }

    // f ! v evaluates to any i that (f ; i) == (f ; v)
    pub(crate) fn transform_optimize<'a, Ctx, Func, Input>(
        func_trans: &Func, input_trans: &Input, mut ctx: Ctx, optimize: OptimizeVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>, {
        let optimize = Optimize::from(optimize);
        let func = func_trans.transform(ctx.reborrow(), optimize.func);
        Self::eval_input_then_optimize(input_trans, ctx, func, optimize.input)
    }

    pub(crate) fn eval_input_then_optimize<'a, Ctx, Input>(
        input_trans: &Input, mut ctx: Ctx, func: Val, input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>, {
        if let Val::Func(func) = func {
            let input = func.mode().optimize.transform(ctx.reborrow(), input);
            Self::optimize_func(ctx, func, input)
        } else {
            input_trans.transform(ctx, input)
        }
    }

    #[allow(dead_code)]
    pub(crate) fn optimize_eval_input<'a, Ctx, Input>(
        input_trans: &Input, ctx: Ctx, func: &Val, input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>, {
        if let Val::Func(func) = func {
            func.mode().optimize.transform(ctx, input)
        } else {
            input_trans.transform(ctx, input)
        }
    }

    pub(crate) fn optimize<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        if let Val::Func(func) = func { Self::optimize_func(ctx, func, input) } else { input }
    }

    pub(crate) fn optimize_func<'a, Ctx>(mut ctx: Ctx, func: FuncVal, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        let question = Optimize::new(Val::Func(func.clone()), input.clone());
        let question = Val::Optimize(question.into());
        if let Some(answer) = Self::call_advisor(ctx.reborrow(), question) {
            if answer != input {
                return answer;
            }
        }
        optimize(ctx, func, input)
    }

    // f ? v evaluates to . or any i that (f ; i) == v
    pub(crate) fn transform_solve<'a, Ctx, Func, Output>(
        func_trans: &Func, output_trans: &Output, mut ctx: Ctx, solve: SolveVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Output: Transformer<Val, Val>, {
        let solve = Solve::from(solve);
        let func = func_trans.transform(ctx.reborrow(), solve.func);
        Self::eval_output_then_solve(output_trans, ctx, func, solve.output)
    }

    pub(crate) fn eval_output_then_solve<'a, Ctx, Output>(
        output_trans: &Output, mut ctx: Ctx, func: Val, output: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Output: Transformer<Val, Val>, {
        if let Val::Func(func) = func {
            let output = func.mode().solve.transform(ctx.reborrow(), output);
            Self::solve_func(ctx, func, output)
        } else {
            let output = output_trans.transform(ctx, output);
            Val::Solve(Solve::new(func, output).into())
        }
    }

    #[allow(dead_code)]
    pub(crate) fn solve_eval_output<'a, Ctx, Output>(
        output_trans: &Output, ctx: Ctx, func: &Val, output: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Output: Transformer<Val, Val>, {
        if let Val::Func(f) = func {
            f.mode().solve.transform(ctx, output)
        } else {
            output_trans.transform(ctx, output)
        }
    }

    pub(crate) fn solve<'a, Ctx>(ctx: Ctx, func: Val, output: Val) -> Val
    where Ctx: CtxMeta<'a> {
        if let Val::Func(func) = func {
            Self::solve_func(ctx, func, output)
        } else {
            Val::Solve(Solve::new(func, output).into())
        }
    }

    pub(crate) fn solve_func<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> Val
    where Ctx: CtxMeta<'a> {
        let question = Solve::new(Val::Func(func.clone()), output.clone());
        let question = Val::Solve(question.into());
        if let Some(answer) = Self::call_advisor(ctx.reborrow(), question) {
            if !answer.is_unit() {
                return answer;
            }
        }
        solve(ctx, func, output)
    }

    pub(crate) fn call_advisor<'a, Ctx>(ctx: Ctx, question: Val) -> Option<Val>
    where Ctx: CtxMeta<'a> {
        let (ctx, is_const) = match ctx.for_mut_fn() {
            MutFnCtx::Free(_) => return None,
            MutFnCtx::Const(ctx) => (ctx.unwrap(), true),
            MutFnCtx::Mut(ctx) => (ctx.unwrap(), false),
        };
        let mut advisor = ctx.set_advisor(None).unwrap()?;
        let answer = if advisor.is_cell() {
            let answer = Self::call_ref(&mut advisor, ctx, is_const, question);
            let _ = ctx.set_advisor(Some(advisor));
            answer
        } else {
            let _ = ctx.set_advisor(Some(advisor.clone()));
            if is_const {
                advisor.transform(ConstCtx::new(ctx), question)
            } else {
                advisor.transform(MutCtx::new(ctx), question)
            }
        };
        Some(answer)
    }
}
