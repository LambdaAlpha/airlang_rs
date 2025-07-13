use std::borrow::Borrow;
use std::hash::Hash;

use const_format::concatcp;

use super::func::ConstStaticFn;
use super::func::FreeStaticFn;
use super::func::FuncSetup;
use super::func::MutCellFn;
use super::func::MutStaticFn;
use super::solver::Solve;
use super::val::CallVal;
use super::val::FuncVal;
use super::val::ListVal;
use super::val::MapVal;
use super::val::PairVal;
use super::val::Val;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::CtxInput;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Symbol;

pub(crate) struct SymbolForm<'a, Fn> {
    pub(crate) default: char,
    pub(crate) f: &'a Fn,
}

pub(crate) const SYMBOL_LITERAL_CHAR: char = '.';
pub(crate) const SYMBOL_LITERAL: &str = concatcp!(SYMBOL_LITERAL_CHAR);
pub(crate) const SYMBOL_REF_CHAR: char = '@';
pub(crate) const SYMBOL_REF: &str = concatcp!(SYMBOL_REF_CHAR);
pub(crate) const SYMBOL_MOVE_CHAR: char = '#';
pub(crate) const SYMBOL_MOVE: &str = concatcp!(SYMBOL_MOVE_CHAR);
pub(crate) const SYMBOL_EVAL_CHAR: char = '$';
pub(crate) const SYMBOL_EVAL: &str = concatcp!(SYMBOL_EVAL_CHAR);

impl<'a, Fn> SymbolForm<'a, Fn> {
    fn recognize(&self, input: Symbol) -> (char, Symbol) {
        match input.chars().next() {
            Some(SYMBOL_LITERAL_CHAR) => {
                (SYMBOL_LITERAL_CHAR, Symbol::from_str_unchecked(&input[1 ..]))
            }
            Some(SYMBOL_REF_CHAR) => (SYMBOL_REF_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            Some(SYMBOL_MOVE_CHAR) => (SYMBOL_MOVE_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            Some(SYMBOL_EVAL_CHAR) => (SYMBOL_EVAL_CHAR, Symbol::from_str_unchecked(&input[1 ..])),
            _ => (self.default, input),
        }
    }
}

impl<'a, Fn> FreeStaticFn<Symbol, Val> for SymbolForm<'a, Fn> {
    fn free_static_call(&self, input: Symbol) -> Val {
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => Val::default(),
            SYMBOL_MOVE_CHAR => Val::default(),
            SYMBOL_EVAL_CHAR => Val::default(),
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> ConstStaticFn<Val, Symbol, Val> for SymbolForm<'a, Fn>
where Fn: ConstStaticFn<Val, Val, Val>
{
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                let Val::Ctx(ctx) = &*ctx else {
                    return Val::default();
                };
                ctx.variables().get_ref(s).cloned().unwrap_or_default()
            }
            SYMBOL_MOVE_CHAR => Val::default(),
            SYMBOL_EVAL_CHAR => {
                let Val::Ctx(ctx1) = &*ctx else {
                    return Val::default();
                };
                let Ok(val) = ctx1.variables().get_ref(s) else {
                    return Val::default();
                };
                let val = val.clone();
                self.f.const_static_call(ctx, val)
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<'a, Fn> MutStaticFn<Val, Symbol, Val> for SymbolForm<'a, Fn>
where Fn: MutStaticFn<Val, Val, Val>
{
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        let (prefix, s) = self.recognize(input);
        match prefix {
            SYMBOL_LITERAL_CHAR => Val::Symbol(s),
            SYMBOL_REF_CHAR => {
                let Val::Ctx(ctx) = &*ctx else {
                    return Val::default();
                };
                ctx.variables().get_ref(s).cloned().unwrap_or_default()
            }
            SYMBOL_MOVE_CHAR => {
                let Val::Ctx(ctx) = ctx else {
                    return Val::default();
                };
                ctx.variables_mut().remove(s).unwrap_or_default()
            }
            SYMBOL_EVAL_CHAR => {
                let Val::Ctx(ctx1) = &*ctx else {
                    return Val::default();
                };
                let Ok(val) = ctx1.variables().get_ref(s) else {
                    return Val::default();
                };
                self.f.mut_static_call(ctx, val.clone())
            }
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

pub(crate) struct PairForm<'a, First, Second> {
    pub(crate) first: &'a First,
    pub(crate) second: &'a Second,
}

impl<'a, First, Second> FreeStaticFn<PairVal, Val> for PairForm<'a, First, Second>
where
    First: FreeStaticFn<Val, Val>,
    Second: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: PairVal) -> Val {
        let pair = Pair::from(input);
        let first = self.first.free_static_call(pair.first);
        let second = self.second.free_static_call(pair.second);
        Val::Pair(Pair::new(first, second).into())
    }
}

impl<'a, First, Second, Ctx> ConstStaticFn<Ctx, PairVal, Val> for PairForm<'a, First, Second>
where
    First: ConstStaticFn<Ctx, Val, Val>,
    Second: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: PairVal) -> Val {
        let pair = Pair::from(input);
        let first = self.first.const_static_call(ctx.reborrow(), pair.first);
        let second = self.second.const_static_call(ctx, pair.second);
        Val::Pair(Pair::new(first, second).into())
    }
}

impl<'a, First, Second, Ctx> MutStaticFn<Ctx, PairVal, Val> for PairForm<'a, First, Second>
where
    First: MutStaticFn<Ctx, Val, Val>,
    Second: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: PairVal) -> Val {
        let pair = Pair::from(input);
        let first = self.first.mut_static_call(ctx, pair.first);
        let second = self.second.mut_static_call(ctx, pair.second);
        Val::Pair(Pair::new(first, second).into())
    }
}

pub(crate) struct CallForm<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput> {
    pub(crate) func: &'a Func,
    pub(crate) ctx: &'a Ctx,
    pub(crate) input: &'a Input,
    pub(crate) some: &'a Map<SomeFunc, CtxInput<SomeCtx, SomeInput>>,
}

impl<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput> FreeStaticFn<CallVal, Val>
    for CallForm<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput>
where
    Func: FreeStaticFn<Val, Val>,
    Ctx: FreeStaticFn<Val, Val>,
    Input: FreeStaticFn<Val, Val>,
    SomeFunc: Borrow<Val> + Eq + Hash,
    SomeCtx: FreeStaticFn<Val, Val>,
    SomeInput: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: CallVal) -> Val {
        let call = Call::from(input);
        if let Some(ctx_input) = self.some.get(&call.func) {
            let ctx = ctx_input.ctx.free_static_call(call.ctx);
            let input = ctx_input.input.free_static_call(call.input);
            return Val::Call(Call::new(call.reverse, call.func, ctx, input).into());
        }
        let func = self.func.free_static_call(call.func);
        let ctx = self.ctx.free_static_call(call.ctx);
        let input = self.input.free_static_call(call.input);
        Val::Call(Call::new(call.reverse, func, ctx, input).into())
    }
}

impl<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput, C> ConstStaticFn<C, CallVal, Val>
    for CallForm<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput>
where
    Func: ConstStaticFn<C, Val, Val>,
    Ctx: ConstStaticFn<C, Val, Val>,
    Input: ConstStaticFn<C, Val, Val>,
    SomeFunc: Borrow<Val> + Eq + Hash,
    SomeCtx: ConstStaticFn<C, Val, Val>,
    SomeInput: ConstStaticFn<C, Val, Val>,
{
    fn const_static_call(&self, mut c: ConstRef<C>, input: CallVal) -> Val {
        let call = Call::from(input);
        if let Some(ctx_input) = self.some.get(&call.func) {
            let ctx = ctx_input.ctx.const_static_call(c.reborrow(), call.ctx);
            let input = ctx_input.input.const_static_call(c, call.input);
            return Val::Call(Call::new(call.reverse, call.func, ctx, input).into());
        }
        let func = self.func.const_static_call(c.reborrow(), call.func);
        let ctx = self.ctx.const_static_call(c.reborrow(), call.ctx);
        let input = self.input.const_static_call(c, call.input);
        Val::Call(Call::new(call.reverse, func, ctx, input).into())
    }
}

impl<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput, C> MutStaticFn<C, CallVal, Val>
    for CallForm<'a, Func, Ctx, Input, SomeFunc, SomeCtx, SomeInput>
where
    Func: MutStaticFn<C, Val, Val>,
    Ctx: MutStaticFn<C, Val, Val>,
    Input: MutStaticFn<C, Val, Val>,
    SomeFunc: Borrow<Val> + Eq + Hash,
    SomeCtx: MutStaticFn<C, Val, Val>,
    SomeInput: MutStaticFn<C, Val, Val>,
{
    fn mut_static_call(&self, c: &mut C, input: CallVal) -> Val {
        let call = Call::from(input);
        if let Some(ctx_input) = self.some.get(&call.func) {
            let ctx = ctx_input.ctx.mut_static_call(c, call.ctx);
            let input = ctx_input.input.mut_static_call(c, call.input);
            return Val::Call(Call::new(call.reverse, call.func, ctx, input).into());
        }
        let func = self.func.mut_static_call(c, call.func);
        let ctx = self.ctx.mut_static_call(c, call.ctx);
        let input = self.input.mut_static_call(c, call.input);
        Val::Call(Call::new(call.reverse, func, ctx, input).into())
    }
}

pub(crate) struct ListUniForm<'a, Item> {
    pub(crate) item: &'a Item,
}

impl<'a, Item> FreeStaticFn<ListVal, Val> for ListUniForm<'a, Item>
where Item: FreeStaticFn<Val, Val>
{
    fn free_static_call(&self, input: ListVal) -> Val {
        let list: List<Val> =
            List::from(input).into_iter().map(|v| self.item.free_static_call(v)).collect();
        Val::List(list.into())
    }
}

impl<'a, Item, Ctx> ConstStaticFn<Ctx, ListVal, Val> for ListUniForm<'a, Item>
where Item: ConstStaticFn<Ctx, Val, Val>
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: ListVal) -> Val {
        let list: List<Val> = List::from(input)
            .into_iter()
            .map(|v| self.item.const_static_call(ctx.reborrow(), v))
            .collect();
        Val::List(list.into())
    }
}

impl<'a, Item, Ctx> MutStaticFn<Ctx, ListVal, Val> for ListUniForm<'a, Item>
where Item: MutStaticFn<Ctx, Val, Val>
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: ListVal) -> Val {
        let list: List<Val> =
            List::from(input).into_iter().map(|v| self.item.mut_static_call(ctx, v)).collect();
        Val::List(list.into())
    }
}

pub(crate) struct ListForm<'a, Head, Tail> {
    pub(crate) head: &'a List<Head>,
    pub(crate) tail: &'a Tail,
}

impl<'a, Head, Tail> FreeStaticFn<ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: FreeStaticFn<Val, Val>,
    Tail: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: ListVal) -> Val {
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = f.free_static_call(val);
            list.push(val);
        }
        for val in iter {
            list.push(self.tail.free_static_call(val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> ConstStaticFn<Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: ConstStaticFn<Ctx, Val, Val>,
    Tail: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: ListVal) -> Val {
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = f.const_static_call(ctx.reborrow(), val);
            list.push(val);
        }
        for val in iter {
            list.push(self.tail.const_static_call(ctx.reborrow(), val));
        }
        Val::List(List::from(list).into())
    }
}

impl<'a, Head, Tail, Ctx> MutStaticFn<Ctx, ListVal, Val> for ListForm<'a, Head, Tail>
where
    Head: MutStaticFn<Ctx, Val, Val>,
    Tail: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: ListVal) -> Val {
        let mut iter = List::from(input).into_iter();
        let mut list = Vec::with_capacity(iter.len());
        for f in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = f.mut_static_call(ctx, val);
            list.push(val);
        }
        for val in iter {
            list.push(self.tail.mut_static_call(ctx, val));
        }
        Val::List(List::from(list).into())
    }
}

pub(crate) struct MapUniForm<'a, Key, Value> {
    pub(crate) key: &'a Key,
    pub(crate) value: &'a Value,
}

impl<'a, Key, Value> FreeStaticFn<MapVal, Val> for MapUniForm<'a, Key, Value>
where
    Key: FreeStaticFn<Val, Val>,
    Value: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: MapVal) -> Val {
        let map: Map<Val, Val> = Map::from(input)
            .into_iter()
            .map(|(k, v)| {
                let key = self.key.free_static_call(k);
                let value = self.value.free_static_call(v);
                (key, value)
            })
            .collect();
        Val::Map(map.into())
    }
}

impl<'a, Key, Value, Ctx> ConstStaticFn<Ctx, MapVal, Val> for MapUniForm<'a, Key, Value>
where
    Key: ConstStaticFn<Ctx, Val, Val>,
    Value: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: MapVal) -> Val {
        let map: Map<Val, Val> = Map::from(input)
            .into_iter()
            .map(|(k, v)| {
                let key = self.key.const_static_call(ctx.reborrow(), k);
                let value = self.value.const_static_call(ctx.reborrow(), v);
                (key, value)
            })
            .collect();
        Val::Map(map.into())
    }
}

impl<'a, Key, Value, Ctx> MutStaticFn<Ctx, MapVal, Val> for MapUniForm<'a, Key, Value>
where
    Key: MutStaticFn<Ctx, Val, Val>,
    Value: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: MapVal) -> Val {
        let map: Map<Val, Val> = Map::from(input)
            .into_iter()
            .map(|(k, v)| {
                let key = self.key.mut_static_call(ctx, k);
                let value = self.value.mut_static_call(ctx, v);
                (key, value)
            })
            .collect();
        Val::Map(map.into())
    }
}

pub(crate) struct MapForm<'a, SomeKey, SomeValue, ElseKey, ElseValue> {
    pub(crate) some: &'a Map<SomeKey, SomeValue>,
    pub(crate) key: &'a ElseKey,
    pub(crate) value: &'a ElseValue,
}

impl<'a, SomeKey, SomeValue, ElseKey, ElseValue> FreeStaticFn<MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseKey, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: FreeStaticFn<Val, Val>,
    ElseKey: FreeStaticFn<Val, Val>,
    ElseValue: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: MapVal) -> Val {
        let map: Map<Val, Val> = Map::from(input)
            .into_iter()
            .map(|(k, v)| {
                if let Some(f) = self.some.get(&k) {
                    let v = f.free_static_call(v);
                    (k, v)
                } else {
                    let k = self.key.free_static_call(k);
                    let v = self.value.free_static_call(v);
                    (k, v)
                }
            })
            .collect();
        Val::Map(map.into())
    }
}

impl<'a, SomeKey, SomeValue, ElseKey, ElseValue, Ctx> ConstStaticFn<Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseKey, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: ConstStaticFn<Ctx, Val, Val>,
    ElseKey: ConstStaticFn<Ctx, Val, Val>,
    ElseValue: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: MapVal) -> Val {
        let map: Map<Val, Val> = Map::from(input)
            .into_iter()
            .map(|(k, v)| {
                if let Some(f) = self.some.get(&k) {
                    let v = f.const_static_call(ctx.reborrow(), v);
                    (k, v)
                } else {
                    let k = self.key.const_static_call(ctx.reborrow(), k);
                    let v = self.value.const_static_call(ctx.reborrow(), v);
                    (k, v)
                }
            })
            .collect();
        Val::Map(map.into())
    }
}

impl<'a, SomeKey, SomeValue, ElseKey, ElseValue, Ctx> MutStaticFn<Ctx, MapVal, Val>
    for MapForm<'a, SomeKey, SomeValue, ElseKey, ElseValue>
where
    SomeKey: Borrow<Val> + Eq + Hash,
    SomeValue: MutStaticFn<Ctx, Val, Val>,
    ElseKey: MutStaticFn<Ctx, Val, Val>,
    ElseValue: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: MapVal) -> Val {
        let map: Map<Val, Val> = Map::from(input)
            .into_iter()
            .map(|(k, v)| {
                if let Some(f) = self.some.get(&k) {
                    let v = f.mut_static_call(ctx, v);
                    (k, v)
                } else {
                    let k = self.key.mut_static_call(ctx, k);
                    let v = self.value.mut_static_call(ctx, v);
                    (k, v)
                }
            })
            .collect();
        Val::Map(map.into())
    }
}

pub(crate) struct CallEval<'a, Func, Ctx, Input> {
    pub(crate) func: &'a Func,
    pub(crate) ctx: &'a Ctx,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Ctx, Input> FreeStaticFn<CallVal, Val> for CallEval<'a, Func, Ctx, Input>
where
    Func: FreeStaticFn<Val, Val>,
    Ctx: FreeStaticFn<Val, Val>,
    Input: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: CallVal) -> Val {
        let call = Call::from(input);
        match self.func.free_static_call(call.func) {
            Val::Func(func) => {
                if call.reverse {
                    let ctx = func.reverse_ctx().free_static_call(call.ctx);
                    let input = func.reverse_input().free_static_call(call.input);
                    let question = Val::Call(Call::new(true, Val::Func(func), ctx, input).into());
                    return Solve.free_static_call(question);
                }
                let _ = func.forward_ctx().free_static_call(call.ctx);
                let input = func.forward_input().free_static_call(call.input);
                func.free_static_call(input)
            }
            Val::Symbol(func) => {
                CallRefEval.free_static_call(Call::new(call.reverse, func, call.ctx, call.input))
            }
            // todo design
            func => {
                let ctx = self.ctx.free_static_call(call.ctx);
                let input = self.input.free_static_call(call.input);
                Val::Call(Call::new(call.reverse, func, ctx, input).into())
            }
        }
    }
}

impl<'a, Func, Ctx, Input> ConstStaticFn<Val, CallVal, Val> for CallEval<'a, Func, Ctx, Input>
where
    Func: ConstStaticFn<Val, Val, Val>,
    Ctx: ConstStaticFn<Val, Val, Val>,
    Input: ConstStaticFn<Val, Val, Val>,
{
    fn const_static_call(&self, mut c: ConstRef<Val>, input: CallVal) -> Val {
        let call = Call::from(input);
        match self.func.const_static_call(c.reborrow(), call.func) {
            Val::Func(func) => {
                if call.reverse {
                    let ctx = func.reverse_ctx().const_static_call(c.reborrow(), call.ctx);
                    let input = func.reverse_input().const_static_call(c.reborrow(), call.input);
                    let question = Val::Call(Call::new(true, Val::Func(func), ctx, input).into());
                    return Solve.const_static_call(c, question);
                }
                let ctx = func.forward_ctx().const_static_call(c.reborrow(), call.ctx);
                let input = func.forward_input().const_static_call(c.reborrow(), call.input);
                const_static_func_call(c, &func, ctx, input)
            }
            Val::Symbol(func) => CallRefEval
                .const_static_call(c, Call::new(call.reverse, func, call.ctx, call.input)),
            // todo design
            func => {
                let ctx = self.ctx.const_static_call(c.reborrow(), call.ctx);
                let input = self.input.const_static_call(c, call.input);
                Val::Call(Call::new(call.reverse, func, ctx, input).into())
            }
        }
    }
}

impl<'a, Func, Ctx, Input> MutStaticFn<Val, CallVal, Val> for CallEval<'a, Func, Ctx, Input>
where
    Func: MutStaticFn<Val, Val, Val>,
    Ctx: MutStaticFn<Val, Val, Val>,
    Input: MutStaticFn<Val, Val, Val>,
{
    fn mut_static_call(&self, c: &mut Val, input: CallVal) -> Val {
        let call = Call::from(input);
        match self.func.mut_static_call(c, call.func) {
            Val::Func(func) => {
                if call.reverse {
                    let ctx = func.reverse_ctx().mut_static_call(c, call.ctx);
                    let input = func.reverse_input().mut_static_call(c, call.input);
                    let question = Val::Call(Call::new(true, Val::Func(func), ctx, input).into());
                    return Solve.mut_static_call(c, question);
                }
                let ctx = func.forward_ctx().mut_static_call(c, call.ctx);
                let input = func.forward_input().mut_static_call(c, call.input);
                mut_static_func_call(c, &func, ctx, input)
            }
            Val::Symbol(func) => {
                CallRefEval.mut_static_call(c, Call::new(call.reverse, func, call.ctx, call.input))
            }
            // todo design
            func => {
                let ctx = self.ctx.mut_static_call(c, call.ctx);
                let input = self.input.mut_static_call(c, call.input);
                Val::Call(Call::new(call.reverse, func, ctx, input).into())
            }
        }
    }
}

pub(crate) struct CallRefEval;

impl FreeStaticFn<Call<Symbol, Val, Val>, Val> for CallRefEval {
    fn free_static_call(&self, _input: Call<Symbol, Val, Val>) -> Val {
        Val::default()
    }
}

impl ConstStaticFn<Val, Call<Symbol, Val, Val>, Val> for CallRefEval {
    fn const_static_call(&self, c: ConstRef<Val>, call: Call<Symbol, Val, Val>) -> Val {
        let c = c.unwrap();
        let Val::Ctx(ctx_val) = c else {
            return Val::default();
        };
        let Ok(ctx_value) = ctx_val.variables_mut().lock(call.func.clone()) else {
            return Val::default();
        };
        let Val::Func(func) = ctx_value.val else {
            ctx_val.variables_mut().unlock(call.func, ctx_value.val);
            return Val::default();
        };
        if call.reverse {
            let ctx = func.reverse_ctx().const_static_call(ConstRef::new(c), call.ctx);
            let input = func.reverse_input().const_static_call(ConstRef::new(c), call.input);
            let Val::Ctx(ctx_val) = c else {
                unreachable!("CallRefEval reverse ctx invariant is broken!!!");
            };
            ctx_val.variables_mut().unlock(call.func.clone(), Val::Func(func));
            let question = Val::Call(Call::new(true, Val::Symbol(call.func), ctx, input).into());
            Solve.const_static_call(ConstRef::new(c), question)
        } else {
            let ctx = func.forward_ctx().const_static_call(ConstRef::new(c), call.ctx);
            let input = func.forward_input().const_static_call(ConstRef::new(c), call.input);
            let output = const_static_func_call(ConstRef::new(c), &func, ctx, input);
            let Val::Ctx(ctx_val) = c else {
                unreachable!("CallRefEval forward ctx invariant is broken!!!");
            };
            ctx_val.variables_mut().unlock(call.func, Val::Func(func));
            output
        }
    }
}

impl MutStaticFn<Val, Call<Symbol, Val, Val>, Val> for CallRefEval {
    fn mut_static_call(&self, c: &mut Val, call: Call<Symbol, Val, Val>) -> Val {
        let Val::Ctx(ctx_val) = c else {
            return Val::default();
        };
        let Ok(ctx_value) = ctx_val.variables_mut().lock(call.func.clone()) else {
            return Val::default();
        };
        let Val::Func(mut func) = ctx_value.val else {
            ctx_val.variables_mut().unlock(call.func, ctx_value.val);
            return Val::default();
        };
        if call.reverse {
            let ctx = func.reverse_ctx().mut_static_call(c, call.ctx);
            let input = func.reverse_input().mut_static_call(c, call.input);
            let Val::Ctx(ctx_val) = c else {
                unreachable!("CallRefEval reverse ctx invariant is broken!!!");
            };
            ctx_val.variables_mut().unlock(call.func.clone(), Val::Func(func));
            let question = Val::Call(Call::new(true, Val::Symbol(call.func), ctx, input).into());
            Solve.mut_static_call(c, question)
        } else {
            let ctx = func.forward_ctx().mut_static_call(c, call.ctx);
            let input = func.forward_input().mut_static_call(c, call.input);
            let output = if ctx_value.contract.is_mutable() {
                mut_cell_func_call(c, &mut func, ctx, input)
            } else {
                mut_static_func_call(c, &func, ctx, input)
            };
            let Val::Ctx(ctx_val) = c else {
                unreachable!("CallRefEval forward ctx invariant is broken!!!");
            };
            ctx_val.variables_mut().unlock(call.func, Val::Func(func));
            output
        }
    }
}

pub(crate) struct CallApply;

impl FreeStaticFn<CallVal, Val> for CallApply {
    fn free_static_call(&self, input: CallVal) -> Val {
        let call = Call::from(input);
        match call.func {
            Val::Func(func) => {
                if call.reverse {
                    let question =
                        Val::Call(Call::new(true, Val::Func(func), call.ctx, call.input).into());
                    return Solve.free_static_call(question);
                }
                func.free_static_call(call.input)
            }
            Val::Symbol(func) => {
                CallRefApply.free_static_call(Call::new(call.reverse, func, call.ctx, call.input))
            }
            // todo design
            func => Val::Call(Call::new(call.reverse, func, call.ctx, call.input).into()),
        }
    }
}

impl ConstStaticFn<Val, CallVal, Val> for CallApply {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: CallVal) -> Val {
        let call = Call::from(input);
        match call.func {
            Val::Func(func) => {
                if call.reverse {
                    let question =
                        Val::Call(Call::new(true, Val::Func(func), call.ctx, call.input).into());
                    return Solve.const_static_call(ctx, question);
                }
                const_static_func_call(ctx, &func, call.ctx, call.input)
            }
            Val::Symbol(func) => CallRefApply
                .const_static_call(ctx, Call::new(call.reverse, func, call.ctx, call.input)),
            // todo design
            func => Val::Call(Call::new(call.reverse, func, call.ctx, call.input).into()),
        }
    }
}

impl MutStaticFn<Val, CallVal, Val> for CallApply {
    fn mut_static_call(&self, ctx: &mut Val, input: CallVal) -> Val {
        let call = Call::from(input);
        match call.func {
            Val::Func(func) => {
                if call.reverse {
                    let question =
                        Val::Call(Call::new(true, Val::Func(func), call.ctx, call.input).into());
                    return Solve.mut_static_call(ctx, question);
                }
                mut_static_func_call(ctx, &func, call.ctx, call.input)
            }
            Val::Symbol(func_name) => CallRefApply
                .mut_static_call(ctx, Call::new(call.reverse, func_name, call.ctx, call.input)),
            // todo design
            func => Val::Call(Call::new(call.reverse, func, call.ctx, call.input).into()),
        }
    }
}

pub(crate) struct CallRefApply;

impl FreeStaticFn<Call<Symbol, Val, Val>, Val> for CallRefApply {
    fn free_static_call(&self, _input: Call<Symbol, Val, Val>) -> Val {
        Val::default()
    }
}

impl ConstStaticFn<Val, Call<Symbol, Val, Val>, Val> for CallRefApply {
    fn const_static_call(&self, ctx: ConstRef<Val>, call: Call<Symbol, Val, Val>) -> Val {
        let ctx = ctx.unwrap();
        let Val::Ctx(ctx_val) = ctx else {
            return Val::default();
        };
        if call.reverse {
            let Ok(val) = ctx_val.variables().get_ref(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(_) = val else {
                return Val::default();
            };
            let question =
                Val::Call(Call::new(true, Val::Symbol(call.func), call.ctx, call.input).into());
            Solve.const_static_call(ConstRef::new(ctx), question)
        } else {
            let Ok(ctx_value) = ctx_val.variables_mut().lock(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(func) = ctx_value.val else {
                ctx_val.variables_mut().unlock(call.func, ctx_value.val);
                return Val::default();
            };
            let output = const_static_func_call(ConstRef::new(ctx), &func, call.ctx, call.input);
            let Val::Ctx(ctx_val) = ctx else {
                unreachable!("CallRefApply ctx invariant is broken!!!");
            };
            ctx_val.variables_mut().unlock(call.func, Val::Func(func));
            output
        }
    }
}

impl MutStaticFn<Val, Call<Symbol, Val, Val>, Val> for CallRefApply {
    fn mut_static_call(&self, ctx: &mut Val, call: Call<Symbol, Val, Val>) -> Val {
        let Val::Ctx(ctx_val) = ctx else {
            return Val::default();
        };
        if call.reverse {
            let Ok(val) = ctx_val.variables().get_ref(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(_) = val else {
                return Val::default();
            };
            let question =
                Val::Call(Call::new(true, Val::Symbol(call.func), call.ctx, call.input).into());
            Solve.mut_static_call(ctx, question)
        } else {
            let Ok(ctx_value) = ctx_val.variables_mut().lock(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(mut func) = ctx_value.val else {
                ctx_val.variables_mut().unlock(call.func, ctx_value.val);
                return Val::default();
            };
            let output = if ctx_value.contract.is_mutable() {
                mut_cell_func_call(ctx, &mut func, call.ctx, call.input)
            } else {
                mut_static_func_call(ctx, &func, call.ctx, call.input)
            };
            let Val::Ctx(ctx_val) = ctx else {
                unreachable!("CallRefApply ctx invariant is broken!!!");
            };
            ctx_val.variables_mut().unlock(call.func, Val::Func(func));
            output
        }
    }
}

fn const_static_func_call(c: ConstRef<Val>, func: &FuncVal, ctx: Val, input: Val) -> Val {
    if ctx.is_unit() {
        return func.const_static_call(c, input);
    }
    let Val::Ctx(ctx_val) = c.unwrap() else {
        return Val::default();
    };
    let Val::Symbol(name) = ctx else {
        return Val::default();
    };
    let Ok(val_ref) = ctx_val.variables_mut().get_ref_dyn(name) else {
        return Val::default();
    };
    func.const_static_call(val_ref.into_const(), input)
}

fn mut_static_func_call(c: &mut Val, func: &FuncVal, ctx: Val, input: Val) -> Val {
    if ctx.is_unit() {
        return func.mut_static_call(c, input);
    }
    let Val::Ctx(ctx_val) = c else {
        return Val::default();
    };
    let Val::Symbol(name) = ctx else {
        return Val::default();
    };
    let Ok(val_ref) = ctx_val.variables_mut().get_ref_dyn(name) else {
        return Val::default();
    };
    func.dyn_static_call(val_ref, input)
}

fn mut_cell_func_call(c: &mut Val, func: &mut FuncVal, ctx: Val, input: Val) -> Val {
    if ctx.is_unit() {
        return func.mut_cell_call(c, input);
    }
    let Val::Ctx(ctx_val) = c else {
        return Val::default();
    };
    let Val::Symbol(name) = ctx else {
        return Val::default();
    };
    let Ok(val_ref) = ctx_val.variables_mut().get_ref_dyn(name) else {
        return Val::default();
    };
    func.dyn_cell_call(val_ref, input)
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Eval;

impl FreeStaticFn<Val, Val> for Eval {
    fn free_static_call(&self, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.free_static_call(symbol),
            Val::Pair(pair) => self.free_static_call(pair),
            Val::Call(call) => self.free_static_call(call),
            Val::List(list) => self.free_static_call(list),
            Val::Map(map) => self.free_static_call(map),
            v => v,
        }
    }
}

impl ConstStaticFn<Val, Val, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.const_static_call(ctx, symbol),
            Val::Pair(pair) => self.const_static_call(ctx, pair),
            Val::Call(call) => self.const_static_call(ctx, call),
            Val::List(list) => self.const_static_call(ctx, list),
            Val::Map(map) => self.const_static_call(ctx, map),
            v => v,
        }
    }
}

impl MutStaticFn<Val, Val, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: Val) -> Val {
        match input {
            Val::Symbol(symbol) => self.mut_static_call(ctx, symbol),
            Val::Pair(pair) => self.mut_static_call(ctx, pair),
            Val::Call(call) => self.mut_static_call(ctx, call),
            Val::List(list) => self.mut_static_call(ctx, list),
            Val::Map(map) => self.mut_static_call(ctx, map),
            v => v,
        }
    }
}

impl FreeStaticFn<Symbol, Val> for Eval {
    fn free_static_call(&self, input: Symbol) -> Val {
        SymbolForm { default: SYMBOL_REF_CHAR, f: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, Symbol, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: Symbol) -> Val {
        SymbolForm { default: SYMBOL_REF_CHAR, f: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, Symbol, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: Symbol) -> Val {
        SymbolForm { default: SYMBOL_REF_CHAR, f: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<PairVal, Val> for Eval {
    fn free_static_call(&self, input: PairVal) -> Val {
        PairForm { first: self, second: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, PairVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: PairVal) -> Val {
        PairForm { first: self, second: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, PairVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: PairVal) -> Val {
        PairForm { first: self, second: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<CallVal, Val> for Eval {
    fn free_static_call(&self, input: CallVal) -> Val {
        CallEval { func: self, ctx: self, input: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, CallVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: CallVal) -> Val {
        CallEval { func: self, ctx: self, input: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, CallVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: CallVal) -> Val {
        CallEval { func: self, ctx: self, input: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<ListVal, Val> for Eval {
    fn free_static_call(&self, input: ListVal) -> Val {
        ListUniForm { item: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, ListVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: ListVal) -> Val {
        ListUniForm { item: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, ListVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: ListVal) -> Val {
        ListUniForm { item: self }.mut_static_call(ctx, input)
    }
}

impl FreeStaticFn<MapVal, Val> for Eval {
    fn free_static_call(&self, input: MapVal) -> Val {
        MapUniForm { key: self, value: self }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, MapVal, Val> for Eval {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        MapUniForm { key: self, value: self }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, MapVal, Val> for Eval {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        MapUniForm { key: self, value: self }.mut_static_call(ctx, input)
    }
}
