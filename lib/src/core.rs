use std::{
    borrow::Borrow,
    hash::Hash,
};

use crate::{
    Abstract,
    AbstractVal,
    Answer,
    AnswerVal,
    Ask,
    AskVal,
    Call,
    CallVal,
    ConstCtx,
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
        default::DefaultCtx,
        map::CtxMapRef,
        ref1::{
            CtxMeta,
            CtxRef,
        },
    },
    optimize::optimize,
    transformer::{
        ByVal,
        Transformer,
    },
};

pub(crate) const SYMBOL_ID_PREFIX: char = '.';
pub(crate) const SYMBOL_REF_PREFIX: char = '$';
pub(crate) const SYMBOL_MOVE_PREFIX: char = '&';

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
            Some(SYMBOL_ID_PREFIX) => (SYMBOL_ID_PREFIX, Symbol::from_str(&s[1..])),
            Some(SYMBOL_REF_PREFIX) => (SYMBOL_REF_PREFIX, Symbol::from_str(&s[1..])),
            Some(SYMBOL_MOVE_PREFIX) => (SYMBOL_MOVE_PREFIX, Symbol::from_str(&s[1..])),
            _ => (DEFAULT, s),
        };
        match prefix {
            SYMBOL_ID_PREFIX => Val::Symbol(s),
            SYMBOL_REF_PREFIX => DefaultCtx.get_or_default(ctx, s),
            SYMBOL_MOVE_PREFIX => DefaultCtx.remove_or_default(ctx, s),
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
        let input = input_trans.transform(ctx, abstract1.input);
        let Val::Func(func) = func else {
            return input;
        };
        optimize(func, input)
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
            Val::Func(mut func) => {
                let input = func.call_mode().transform(ctx.reborrow(), input);
                func.transform_mut(ctx, input)
            }
            Val::Symbol(s) => EvalCore::call_cell(ctx, s, input),
            _ => {
                let input = input_trans.transform(ctx, input);
                Val::Call(Call::new(func, input).into())
            }
        }
    }

    pub(crate) fn eval_input<'a, Ctx, Input>(
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
            func.call_mode().transform(ctx, input)
        } else {
            input_trans.transform(ctx, input)
        }
    }

    pub(crate) fn call<'a, Ctx>(ctx: Ctx, func: Val, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(mut func) = func {
            func.transform_mut(ctx, input)
        } else {
            Val::Call(Call::new(func, input).into())
        }
    }

    pub(crate) fn call_cell<'a, Ctx>(ctx: Ctx, func_name: Symbol, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match ctx.for_mut_fn() {
            MutFnCtx::Free(_) => Val::default(),
            MutFnCtx::Const(ctx) => Self::call_cell_const(ctx, func_name, input),
            MutFnCtx::Mut(ctx) => Self::call_cell_mut(ctx, func_name, input),
        }
    }

    fn call_cell_const(mut ctx: ConstCtx, func_name: Symbol, input: Val) -> Val {
        let Ok(val) = ctx.reborrow().get_ctx_ref().variables().get_ref(func_name) else {
            return Val::default();
        };
        let Val::Func(FuncVal::Cell(func)) = val else {
            return Val::default();
        };
        let mut func = func.clone();
        let input = func.call_mode().transform(ctx, input);
        func.transform_mut(input)
    }

    fn call_cell_mut(ctx: MutCtx, func_name: Symbol, input: Val) -> Val {
        let ctx = ctx.unwrap();
        let Ok(val) = ctx.variables().get_ref(func_name.clone()) else {
            return Val::default();
        };
        let Val::Func(FuncVal::Cell(func)) = val else {
            return Val::default();
        };
        let input = func.call_mode().clone().transform(MutCtx::new(ctx), input);
        let Ok(val) = ctx.variables_mut().get_ref_dyn(func_name) else {
            return Val::default();
        };
        let Val::Func(FuncVal::Cell(func)) = val.ref1 else {
            return Val::default();
        };
        if val.is_const {
            func.transform(input)
        } else {
            func.transform_mut(input)
        }
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
            let output = func.ask_mode().transform(ctx.reborrow(), output);
            Val::Answer(Self::solve(ctx, func, output))
        } else {
            let output = output_trans.transform(ctx, output);
            Val::Ask(Ask::new(func, output).into())
        }
    }

    pub(crate) fn eval_output<'a, Ctx, Output>(
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
            f.ask_mode().transform(ctx, output)
        } else {
            output_trans.transform(ctx, output)
        }
    }

    pub(crate) fn ask<'a, Ctx>(ctx: Ctx, func: Val, output: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        if let Val::Func(func) = func {
            Val::Answer(Self::solve(ctx, func, output))
        } else {
            Val::Ask(Ask::new(func, output).into())
        }
    }

    pub(crate) fn solve<'a, Ctx>(mut ctx: Ctx, func: FuncVal, output: Val) -> AnswerVal
    where
        Ctx: CtxMeta<'a>,
    {
        let none = AnswerVal::from(Answer::None);
        let Ok(solver) = ctx.reborrow().get_solver_dyn() else {
            return none;
        };
        let ask = Ask::new(Val::Func(func.clone()), output.clone());
        let ask = Val::Ask(ask.into());
        let answer = if solver.is_const {
            solver.ref1.transform(ask)
        } else {
            solver.ref1.transform_mut(ask)
        };
        let Val::Answer(answer) = answer else {
            return none;
        };
        let Answer::Cache(cache) = &*answer else {
            return answer;
        };
        let Val::Func(cache_func) = &cache.func else {
            return none;
        };
        if *cache_func != func || cache.output != output {
            return none;
        }
        answer
    }
}
