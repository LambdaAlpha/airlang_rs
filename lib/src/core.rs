use std::borrow::Borrow;
use std::hash::Hash;

use crate::Call;
use crate::CallVal;
use crate::ConstCellFn;
use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::List;
use crate::ListVal;
use crate::Map;
use crate::MapVal;
use crate::MutCellFn;
use crate::MutStaticFn;
use crate::Pair;
use crate::PairVal;
use crate::Symbol;
use crate::Val;
use crate::VarAccess;
use crate::ctx::main::MainCtx;
use crate::func::FuncTrait;
use crate::mode::symbol::LITERAL_CHAR;
use crate::mode::symbol::MOVE_CHAR;
use crate::mode::symbol::REF_CHAR;
use crate::solver::Solver;

pub(crate) struct SymbolForm<const DEFAULT: char>;

impl<const DEFAULT: char> SymbolForm<DEFAULT> {
    fn recognize(&self, input: Symbol) -> (char, Symbol) {
        match input.chars().next() {
            Some(LITERAL_CHAR) => (LITERAL_CHAR, Symbol::from_str(&input[1 ..])),
            Some(REF_CHAR) => (REF_CHAR, Symbol::from_str(&input[1 ..])),
            Some(MOVE_CHAR) => (MOVE_CHAR, Symbol::from_str(&input[1 ..])),
            _ => (DEFAULT, input),
        }
    }
}

impl<const DEFAULT: char> FreeStaticFn<Symbol, Val> for SymbolForm<DEFAULT> {
    fn free_static_call(&self, input: Symbol) -> Val {
        let (mode, s) = self.recognize(input);
        match mode {
            LITERAL_CHAR => Val::Symbol(s),
            REF_CHAR => Val::default(),
            MOVE_CHAR => Val::default(),
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<const DEFAULT: char> ConstStaticFn<Ctx, Symbol, Val> for SymbolForm<DEFAULT> {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: Symbol) -> Val {
        let (mode, s) = self.recognize(input);
        match mode {
            LITERAL_CHAR => Val::Symbol(s),
            REF_CHAR => MainCtx::get_or_default(&ctx, s),
            MOVE_CHAR => Val::default(),
            _ => unreachable!("DEFAULT should be predefined character"),
        }
    }
}

impl<const DEFAULT: char> MutStaticFn<Ctx, Symbol, Val> for SymbolForm<DEFAULT> {
    fn mut_static_call(&self, ctx: &mut Ctx, input: Symbol) -> Val {
        let (mode, s) = self.recognize(input);
        match mode {
            LITERAL_CHAR => Val::Symbol(s),
            REF_CHAR => MainCtx::get_or_default(ctx, s),
            MOVE_CHAR => MainCtx::remove_or_default(ctx, s),
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

pub(crate) struct CallForm<'a, Func, Input> {
    pub(crate) func: &'a Func,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Input> FreeStaticFn<CallVal, Val> for CallForm<'a, Func, Input>
where
    Func: FreeStaticFn<Val, Val>,
    Input: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: CallVal) -> Val {
        let call = Call::from(input);
        let func = self.func.free_static_call(call.func);
        let input = self.input.free_static_call(call.input);
        Val::Call(Call::new(call.reverse, func, input).into())
    }
}

impl<'a, Func, Input, Ctx> ConstStaticFn<Ctx, CallVal, Val> for CallForm<'a, Func, Input>
where
    Func: ConstStaticFn<Ctx, Val, Val>,
    Input: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: CallVal) -> Val {
        let call = Call::from(input);
        let func = self.func.const_static_call(ctx.reborrow(), call.func);
        let input = self.input.const_static_call(ctx, call.input);
        Val::Call(Call::new(call.reverse, func, input).into())
    }
}

impl<'a, Func, Input, Ctx> MutStaticFn<Ctx, CallVal, Val> for CallForm<'a, Func, Input>
where
    Func: MutStaticFn<Ctx, Val, Val>,
    Input: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        let call = Call::from(input);
        let func = self.func.mut_static_call(ctx, call.func);
        let input = self.input.mut_static_call(ctx, call.input);
        Val::Call(Call::new(call.reverse, func, input).into())
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
        for mode in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = mode.free_static_call(val);
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
        for mode in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = mode.const_static_call(ctx.reborrow(), val);
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
        for mode in self.head {
            let Some(val) = iter.next() else {
                break;
            };
            let val = mode.mut_static_call(ctx, val);
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

pub(crate) struct MapForm<'a, SomeKey, SomeValue, ElseKey, ElseValue>
where SomeKey: Eq + Hash {
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
                if let Some(mode) = self.some.get(&k) {
                    let v = mode.free_static_call(v);
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
                if let Some(mode) = self.some.get(&k) {
                    let v = mode.const_static_call(ctx.reborrow(), v);
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
                if let Some(mode) = self.some.get(&k) {
                    let v = mode.mut_static_call(ctx, v);
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

pub(crate) struct CallEval<'a, Func, Input> {
    pub(crate) func: &'a Func,
    pub(crate) input: &'a Input,
}

impl<'a, Func, Input> FreeStaticFn<CallVal, Val> for CallEval<'a, Func, Input>
where
    Func: FreeStaticFn<Val, Val>,
    Input: FreeStaticFn<Val, Val>,
{
    fn free_static_call(&self, input: CallVal) -> Val {
        let call = Call::from(input);
        match self.func.free_static_call(call.func) {
            Val::Func(func) => {
                if call.reverse {
                    let input = func.mode().reverse.free_static_call(call.input);
                    let question = Val::Call(Call::new(true, Val::Func(func), input).into());
                    Solver.free_static_call(question)
                } else {
                    let input = func.mode().forward.free_static_call(call.input);
                    func.free_static_call(input)
                }
            }
            Val::Symbol(func) => {
                CallRefEval.free_static_call(Call::new(call.reverse, func, call.input))
            }
            func => {
                let input = self.input.free_static_call(call.input);
                Val::Call(Call::new(call.reverse, func, input).into())
            }
        }
    }
}

impl<'a, Func, Input> ConstStaticFn<Ctx, CallVal, Val> for CallEval<'a, Func, Input>
where
    Func: ConstStaticFn<Ctx, Val, Val>,
    Input: ConstStaticFn<Ctx, Val, Val>,
{
    fn const_static_call(&self, mut ctx: ConstRef<Ctx>, input: CallVal) -> Val {
        let call = Call::from(input);
        match self.func.const_static_call(ctx.reborrow(), call.func) {
            Val::Func(func) => {
                if call.reverse {
                    let input = func.mode().reverse.const_static_call(ctx.reborrow(), call.input);
                    let question = Val::Call(Call::new(true, Val::Func(func), input).into());
                    Solver.const_static_call(ctx, question)
                } else {
                    let input = func.mode().forward.const_static_call(ctx.reborrow(), call.input);
                    func.const_static_call(ctx, input)
                }
            }
            Val::Symbol(func) => {
                CallRefEval.const_static_call(ctx, Call::new(call.reverse, func, call.input))
            }
            func => {
                let input = self.input.const_static_call(ctx, call.input);
                Val::Call(Call::new(call.reverse, func, input).into())
            }
        }
    }
}

impl<'a, Func, Input> MutStaticFn<Ctx, CallVal, Val> for CallEval<'a, Func, Input>
where
    Func: MutStaticFn<Ctx, Val, Val>,
    Input: MutStaticFn<Ctx, Val, Val>,
{
    fn mut_static_call(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        let call = Call::from(input);
        match self.func.mut_static_call(ctx, call.func) {
            Val::Func(func) => {
                if call.reverse {
                    let input = func.mode().reverse.mut_static_call(ctx, call.input);
                    let question = Val::Call(Call::new(true, Val::Func(func), input).into());
                    Solver.mut_static_call(ctx, question)
                } else {
                    let input = func.mode().forward.mut_static_call(ctx, call.input);
                    func.mut_static_call(ctx, input)
                }
            }
            Val::Symbol(func) => {
                CallRefEval.mut_static_call(ctx, Call::new(call.reverse, func, call.input))
            }
            func => {
                let input = self.input.mut_static_call(ctx, call.input);
                Val::Call(Call::new(call.reverse, func, input).into())
            }
        }
    }
}

pub(crate) struct CallRefEval;

impl FreeStaticFn<Call<Symbol, Val>, Val> for CallRefEval {
    fn free_static_call(&self, _input: Call<Symbol, Val>) -> Val {
        Val::default()
    }
}

impl ConstStaticFn<Ctx, Call<Symbol, Val>, Val> for CallRefEval {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, call: Call<Symbol, Val>) -> Val {
        let ctx = ctx.unwrap();
        let Some(ctx_value) = ctx.variables_mut().set_inaccessible(call.func.clone()) else {
            return Val::default();
        };
        let Val::Func(mut func) = ctx_value.val else {
            ctx.variables_mut().set_accessible(call.func, ctx_value.val);
            return Val::default();
        };
        if call.reverse {
            let input = func.mode().reverse.const_static_call(ConstRef::new(ctx), call.input);
            ctx.variables_mut().set_accessible(call.func.clone(), Val::Func(func));
            let question = Val::Call(Call::new(true, Val::Symbol(call.func), input).into());
            Solver.const_static_call(ConstRef::new(ctx), question)
        } else {
            let input = func.mode().forward.const_static_call(ConstRef::new(ctx), call.input);
            let output = func.const_cell_call(ConstRef::new(ctx), input);
            ctx.variables_mut().set_accessible(call.func, Val::Func(func));
            output
        }
    }
}

impl MutStaticFn<Ctx, Call<Symbol, Val>, Val> for CallRefEval {
    fn mut_static_call(&self, ctx: &mut Ctx, call: Call<Symbol, Val>) -> Val {
        let Some(ctx_value) = ctx.variables_mut().set_inaccessible(call.func.clone()) else {
            return Val::default();
        };
        let Val::Func(mut func) = ctx_value.val else {
            ctx.variables_mut().set_accessible(call.func, ctx_value.val);
            return Val::default();
        };
        if call.reverse {
            let input = func.mode().reverse.mut_static_call(ctx, call.input);
            ctx.variables_mut().set_accessible(call.func.clone(), Val::Func(func));
            let question = Val::Call(Call::new(true, Val::Symbol(call.func), input).into());
            Solver.mut_static_call(ctx, question)
        } else {
            let input = func.mode().forward.mut_static_call(ctx, call.input);
            let output = if ctx_value.access == VarAccess::Const {
                func.mut_static_call(ctx, input)
            } else {
                func.mut_cell_call(ctx, input)
            };
            ctx.variables_mut().set_accessible(call.func, Val::Func(func));
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
                    let question = Val::Call(Call::new(true, Val::Func(func), call.input).into());
                    Solver.free_static_call(question)
                } else {
                    func.free_static_call(call.input)
                }
            }
            Val::Symbol(func) => {
                CallRefApply.free_static_call(Call::new(call.reverse, func, call.input))
            }
            func => Val::Call(Call::new(call.reverse, func, call.input).into()),
        }
    }
}

impl ConstStaticFn<Ctx, CallVal, Val> for CallApply {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: CallVal) -> Val {
        let call = Call::from(input);
        match call.func {
            Val::Func(func) => {
                if call.reverse {
                    let question = Val::Call(Call::new(true, Val::Func(func), call.input).into());
                    Solver.const_static_call(ctx, question)
                } else {
                    func.const_static_call(ctx, call.input)
                }
            }
            Val::Symbol(func) => {
                CallRefApply.const_static_call(ctx, Call::new(call.reverse, func, call.input))
            }
            func => Val::Call(Call::new(call.reverse, func, call.input).into()),
        }
    }
}

impl MutStaticFn<Ctx, CallVal, Val> for CallApply {
    fn mut_static_call(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        let call = Call::from(input);
        match call.func {
            Val::Func(func) => {
                if call.reverse {
                    let question = Val::Call(Call::new(true, Val::Func(func), call.input).into());
                    Solver.mut_static_call(ctx, question)
                } else {
                    func.mut_static_call(ctx, call.input)
                }
            }
            Val::Symbol(func_name) => {
                CallRefApply.mut_static_call(ctx, Call::new(call.reverse, func_name, call.input))
            }
            func => Val::Call(Call::new(call.reverse, func, call.input).into()),
        }
    }
}

pub(crate) struct CallRefApply;

impl FreeStaticFn<Call<Symbol, Val>, Val> for CallRefApply {
    fn free_static_call(&self, _input: Call<Symbol, Val>) -> Val {
        Val::default()
    }
}

impl ConstStaticFn<Ctx, Call<Symbol, Val>, Val> for CallRefApply {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, call: Call<Symbol, Val>) -> Val {
        if call.reverse {
            let Ok(val) = ctx.variables().get_ref(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(_) = val else {
                return Val::default();
            };
            let question = Val::Call(Call::new(true, Val::Symbol(call.func), call.input).into());
            Solver.const_static_call(ctx, question)
        } else {
            let ctx = ctx.unwrap();
            let Some(ctx_value) = ctx.variables_mut().set_inaccessible(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(mut func) = ctx_value.val else {
                ctx.variables_mut().set_accessible(call.func, ctx_value.val);
                return Val::default();
            };
            let output = func.const_cell_call(ConstRef::new(ctx), call.input);
            ctx.variables_mut().set_accessible(call.func, Val::Func(func));
            output
        }
    }
}

impl MutStaticFn<Ctx, Call<Symbol, Val>, Val> for CallRefApply {
    fn mut_static_call(&self, ctx: &mut Ctx, call: Call<Symbol, Val>) -> Val {
        if call.reverse {
            let Ok(val) = ctx.variables().get_ref(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(_) = val else {
                return Val::default();
            };
            let question = Val::Call(Call::new(true, Val::Symbol(call.func), call.input).into());
            Solver.mut_static_call(ctx, question)
        } else {
            let Some(ctx_value) = ctx.variables_mut().set_inaccessible(call.func.clone()) else {
                return Val::default();
            };
            let Val::Func(mut func) = ctx_value.val else {
                ctx.variables_mut().set_accessible(call.func, ctx_value.val);
                return Val::default();
            };
            let output = if ctx_value.access == VarAccess::Const {
                func.mut_static_call(ctx, call.input)
            } else {
                func.mut_cell_call(ctx, call.input)
            };
            ctx.variables_mut().set_accessible(call.func, Val::Func(func));
            output
        }
    }
}
