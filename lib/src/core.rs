use std::borrow::Borrow;
use std::hash::Hash;

use crate::Call;
use crate::CallVal;
use crate::ConstCtx;
use crate::Ctx;
use crate::FuncVal;
use crate::List;
use crate::ListVal;
use crate::Map;
use crate::MapVal;
use crate::MutCtx;
use crate::MutFnCtx;
use crate::Pair;
use crate::PairVal;
use crate::Symbol;
use crate::Val;
use crate::ctx::main::MainCtx;
use crate::ctx::map::CtxValue;
use crate::ctx::ref1::CtxMeta;
use crate::ctx::ref1::CtxRef;
use crate::func::FuncTrait;
use crate::mode::symbol::LITERAL_CHAR;
use crate::mode::symbol::MOVE_CHAR;
use crate::mode::symbol::REF_CHAR;
use crate::transformer::ByVal;
use crate::transformer::Transformer;

pub(crate) struct FormCore;

impl FormCore {
    pub(crate) fn transform_val<'a, Ctx, T>(t: &T, ctx: Ctx, input: Val) -> Val
    where
        Ctx: CtxMeta<'a>,
        T: ByVal<Val>, {
        match input {
            Val::Symbol(symbol) => t.transform_symbol(ctx, symbol),
            Val::Pair(pair) => t.transform_pair(ctx, pair),
            Val::Call(call) => t.transform_call(ctx, call),
            Val::List(list) => t.transform_list(ctx, list),
            Val::Map(map) => t.transform_map(ctx, map),
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
            REF_CHAR => MainCtx::get_or_default(ctx, s),
            MOVE_CHAR => MainCtx::remove_or_default(ctx, s),
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
        Val::Call(Call { reverse: call.reverse, func, input }.into())
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
        Self::eval_input_then_call(input_trans, ctx, call.reverse, func, call.input)
    }

    pub(crate) fn eval_input_then_call<'a, Ctx, Input>(
        input_trans: &Input, mut ctx: Ctx, reverse: bool, func: Val, input: Val,
    ) -> Val
    where
        Ctx: CtxMeta<'a>,
        Input: Transformer<Val, Val>, {
        match func {
            Val::Func(func) => {
                let input = if reverse {
                    func.mode().reverse.transform(ctx.reborrow(), input)
                } else {
                    func.mode().forward.transform(ctx.reborrow(), input)
                };
                Self::call_func(ctx, reverse, func, input)
            }
            Val::Symbol(func) => {
                if reverse {
                    let input = input_trans.transform(ctx, input);
                    return Val::Call(Call::new(true, Val::Symbol(func), input).into());
                }
                Self::with_ref(ctx, func, |func, ctx, is_const| {
                    let input = Self::call_ref_eval_input(func, ctx, is_const, input);
                    Self::call_ref(func, ctx, is_const, input)
                })
            }
            _ => {
                let input = input_trans.transform(ctx, input);
                Val::Call(Call::new(reverse, func, input).into())
            }
        }
    }

    fn call_ref_eval_input(func: &FuncVal, ctx: &mut Ctx, is_const: bool, input: Val) -> Val {
        if is_const {
            func.mode().forward.transform(ConstCtx::new(ctx), input)
        } else {
            func.mode().forward.transform(MutCtx::new(ctx), input)
        }
    }

    pub(crate) fn call<'a, Ctx>(ctx: Ctx, reverse: bool, func: Val, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        match func {
            Val::Func(func) => Self::call_func(ctx, reverse, func, input),
            Val::Symbol(func) => Self::with_ref(ctx, func, |func, ctx, is_const| {
                Self::call_ref(func, ctx, is_const, input)
            }),
            _ => Val::Call(Call::new(reverse, func, input).into()),
        }
    }

    fn call_func<'a, Ctx>(ctx: Ctx, reverse: bool, func: FuncVal, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        if reverse { Self::reverse_func(ctx, func, input) } else { func.transform(ctx, input) }
    }

    fn reverse_func<'a, Ctx>(mut ctx: Ctx, func: FuncVal, input: Val) -> Val
    where Ctx: CtxMeta<'a> {
        let question = Val::Call(Call::new(true, Val::Func(func.clone()), input.clone()).into());
        if let Some(answer) = Self::call_solver(ctx.reborrow(), question) {
            if !answer.is_unit() {
                return answer;
            }
        }
        crate::solver::reverse(ctx, func, input)
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

    pub(crate) fn call_solver<'a, Ctx>(ctx: Ctx, question: Val) -> Option<Val>
    where Ctx: CtxMeta<'a> {
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
