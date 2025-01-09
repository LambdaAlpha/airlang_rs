use std::{
    borrow::Borrow,
    hash::Hash,
};

use crate::{
    Abstract,
    AbstractVal,
    Ask,
    AskVal,
    Call,
    CallVal,
    ConstCtx,
    Ctx,
    FuncVal,
    List,
    ListVal,
    Map,
    MapVal,
    MutCtx,
    MutFnCtx,
    Pair,
    PairVal,
    Symbol,
    Val,
    ctx::{
        CtxValue,
        default::DefaultCtx,
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    func::FuncTrait,
    mode::form::{
        LITERAL,
        MOVE,
        REF,
    },
    solver::{
        optimize,
        solve,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

pub(crate) struct FormCore;

impl FormCore {
    pub(crate) fn transform_val<'a, Ctx, T>(t: &T, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: ByVal<Val>,
    {
        match input {
            Val::Symbol(s) => t.transform_symbol(ctx, s),
            Val::Pair(p) => t.transform_pair(ctx, p),
            Val::Call(c) => t.transform_call(ctx, c),
            Val::Abstract(a) => t.transform_abstract(ctx, a),
            Val::Ask(a) => t.transform_ask(ctx, a),
            Val::List(l) => t.transform_list(ctx, l),
            Val::Map(m) => t.transform_map(ctx, m),
            v => t.transform_default(ctx, v),
        }
    }

    pub(crate) fn transform_symbol<'a, const DEFAULT: char, Ctx>(ctx: Ctx, s: Symbol) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let (prefix, s) = match s.chars().next() {
            Some(LITERAL) => (LITERAL, Symbol::from_str(&s[1..])),
            Some(REF) => (REF, Symbol::from_str(&s[1..])),
            Some(MOVE) => (MOVE, Symbol::from_str(&s[1..])),
            _ => (DEFAULT, s),
        };
        match prefix {
            LITERAL => Val::Symbol(s),
            REF => DefaultCtx::get_or_default(ctx, s),
            MOVE => DefaultCtx::remove_or_default(ctx, s),
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }

    pub(crate) fn transform_pair<'a, Ctx, First, Second>(
        first: &First,
        second: &Second,
        mut ctx: Ctx,
        pair: PairVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        First: Transformer<Val, Val>,
        Second: Transformer<Val, Val>,
    {
        let pair = Pair::from(pair);
        let first = first.transform(ctx.reborrow(), pair.first);
        let second = second.transform(ctx, pair.second);
        Val::Pair(Pair::new(first, second).into())
    }

    pub(crate) fn transform_call<'a, Ctx, Func, Input>(
        func: &Func,
        input: &Input,
        mut ctx: Ctx,
        call: CallVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>,
    {
        let call = Call::from(call);
        let func = func.transform(ctx.reborrow(), call.func);
        let input = input.transform(ctx, call.input);
        Val::Call(Call::new(func, input).into())
    }

    pub(crate) fn transform_abstract<'a, Ctx, Func, Input>(
        func: &Func,
        input: &Input,
        mut ctx: Ctx,
        abstract1: AbstractVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>,
    {
        let abstract1 = Abstract::from(abstract1);
        let func = func.transform(ctx.reborrow(), abstract1.func);
        let input = input.transform(ctx, abstract1.input);
        Val::Abstract(Abstract::new(func, input).into())
    }

    pub(crate) fn transform_ask<'a, Ctx, Func, Output>(
        func: &Func,
        output: &Output,
        mut ctx: Ctx,
        ask: AskVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Output: Transformer<Val, Val>,
    {
        let ask = Ask::from(ask);
        let func = func.transform(ctx.reborrow(), ask.func);
        let output = output.transform(ctx, ask.output);
        Val::Ask(Ask::new(func, output).into())
    }

    pub(crate) fn transform_list<'a, Ctx, Item>(item: &Item, mut ctx: Ctx, list: ListVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        Item: Transformer<Val, Val>,
    {
        let list: List<Val> = List::from(list)
            .into_iter()
            .map(|v| item.transform(ctx.reborrow(), v))
            .collect();
        Val::List(list.into())
    }

    pub(crate) fn transform_list_head_tail<'a, Ctx, Head, Tail>(
        head: &List<Head>,
        tail: &Tail,
        mut ctx: Ctx,
        list: ListVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Head: Transformer<Val, Val>,
        Tail: Transformer<Val, Val>,
    {
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

    pub(crate) fn transform_map<'a, Ctx, K, V>(key: &K, value: &V, mut ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
        K: Transformer<Val, Val>,
        V: Transformer<Val, Val>,
    {
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
        some: &Map<SomeK, SomeV>,
        key: &ElseK,
        value: &ElseV,
        mut ctx: Ctx,
        map: MapVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        SomeK: Borrow<Val> + Eq + Hash,
        SomeV: Transformer<Val, Val>,
        ElseK: Transformer<Val, Val>,
        ElseV: Transformer<Val, Val>,
    {
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
        func_trans: &Func,
        input_trans: &Input,
        mut ctx: Ctx,
        call: CallVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>,
    {
        let call = Call::from(call);
        let func = func_trans.transform(ctx.reborrow(), call.func);
        Self::eval_input_then_call(input_trans, ctx, func, call.input)
    }

    pub(crate) fn eval_input_then_call<'a, Ctx, Input>(
        input_trans: &Input,
        mut ctx: Ctx,
        func: Val,
        input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>,
    {
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

    pub(crate) fn call_eval_input<'a, Ctx, Input>(
        input_trans: &Input,
        ctx: Ctx,
        func: &Val,
        input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>,
    {
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
    where
        Ctx: CtxMeta<'a>,
    {
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
        ctx: Ctx,
        func_name: Symbol,
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
        let ctx_value = CtxValue {
            val: Val::Func(func),
            ..ctx_value
        };
        ctx.variables_mut().put_unchecked(func_name, ctx_value);
        output
    }

    // f ! v evaluates to any i that (f ; i) == (f ; v)
    pub(crate) fn transform_abstract<'a, Ctx, Func, Input>(
        func_trans: &Func,
        input_trans: &Input,
        mut ctx: Ctx,
        abstract1: AbstractVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Input: Transformer<Val, Val>,
    {
        let abstract1 = Abstract::from(abstract1);
        let func = func_trans.transform(ctx.reborrow(), abstract1.func);
        Self::eval_input_then_abstract(input_trans, ctx, func, abstract1.input)
    }

    pub(crate) fn eval_input_then_abstract<'a, Ctx, Input>(
        input_trans: &Input,
        mut ctx: Ctx,
        func: Val,
        input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>,
    {
        if let Val::Func(func) = func {
            let input = func.mode().abstract1.transform(ctx.reborrow(), input);
            Self::abstract_func(ctx, func, input)
        } else {
            input_trans.transform(ctx, input)
        }
    }

    pub(crate) fn abstract_eval_input<'a, Ctx, Input>(
        input_trans: &Input,
        ctx: Ctx,
        func: &Val,
        input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>,
    {
        if let Val::Func(func) = func {
            func.mode().abstract1.transform(ctx, input)
        } else {
            input_trans.transform(ctx, input)
        }
    }

    pub(crate) fn abstract1<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            Self::abstract_func(ctx, func, input)
        } else {
            input
        }
    }

    pub(crate) fn abstract_func<'a, Ctx>(mut ctx: Ctx, func: FuncVal, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let question = Abstract::new(Val::Func(func.clone()), input.clone());
        let question = Val::Abstract(question.into());
        if let Some(answer) = Self::call_solver(ctx.reborrow(), question) {
            if answer != input {
                return answer;
            }
        }
        optimize(ctx, func, input)
    }

    pub(crate) fn transform_ask<'a, Ctx, Func, Output>(
        func_trans: &Func,
        output_trans: &Output,
        mut ctx: Ctx,
        ask: AskVal,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Func: Transformer<Val, Val>,
        Output: Transformer<Val, Val>,
    {
        let ask = Ask::from(ask);
        let func = func_trans.transform(ctx.reborrow(), ask.func);
        Self::eval_output_then_solve(output_trans, ctx, func, ask.output)
    }

    pub(crate) fn eval_output_then_solve<'a, Ctx, Output>(
        output_trans: &Output,
        mut ctx: Ctx,
        func: Val,
        output: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Output: Transformer<Val, Val>,
    {
        if let Val::Func(func) = func {
            let output = func.mode().ask.transform(ctx.reborrow(), output);
            Self::ask_func(ctx, func, output)
        } else {
            let output = output_trans.transform(ctx, output);
            Val::Ask(Ask::new(func, output).into())
        }
    }

    pub(crate) fn ask_eval_output<'a, Ctx, Output>(
        output_trans: &Output,
        ctx: Ctx,
        func: &Val,
        output: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Output: Transformer<Val, Val>,
    {
        if let Val::Func(f) = func {
            f.mode().ask.transform(ctx, output)
        } else {
            output_trans.transform(ctx, output)
        }
    }

    pub(crate) fn ask<'a, Ctx>(ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            Self::ask_func(ctx, func, output)
        } else {
            Val::Ask(Ask::new(func, output).into())
        }
    }

    pub(crate) fn ask_func<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let question = Ask::new(Val::Func(func.clone()), output.clone());
        let question = Val::Ask(question.into());
        if let Some(answer) = Self::call_solver(ctx.reborrow(), question) {
            if !answer.is_unit() {
                return answer;
            }
        }
        solve(ctx, func, output)
    }

    pub(crate) fn call_solver<'a, Ctx>(ctx: Ctx, question: Val) -> Option<Val>
    where
        Ctx: CtxMeta<'a>,
    {
        let (ctx, is_const) = match ctx.for_mut_fn() {
            MutFnCtx::Free(_) => return None,
            MutFnCtx::Const(ctx) => (ctx.unwrap(), true),
            MutFnCtx::Mut(ctx) => (ctx.unwrap(), false),
        };
        let mut solver = ctx.set_solver(None).unwrap()?;
        let answer = if solver.is_cell() {
            let answer = Self::call_ref(&mut solver, ctx, is_const, question);
            let _ = ctx.set_solver(Some(solver));
            answer
        } else {
            let _ = ctx.set_solver(Some(solver.clone()));
            if is_const {
                solver.transform(ConstCtx::new(ctx), question)
            } else {
                solver.transform(MutCtx::new(ctx), question)
            }
        };
        Some(answer)
    }
}
