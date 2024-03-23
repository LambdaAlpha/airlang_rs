use std::ops::Deref;

use crate::{
    call::Call,
    ctx_access::CtxAccessor,
    list::List,
    map::Map,
    pair::Pair,
    reverse::Reverse,
    transform::{
        eval::{
            Eval,
            EvalByRef,
        },
        id::{
            Id,
            IdByRef,
        },
        Transform,
    },
    transformer::{
        output::OutputBuilder,
        Transformer,
        ValBuilder,
    },
    CallVal,
    CtxForMutableFn,
    ListVal,
    MapVal,
    PairVal,
    ReverseVal,
    Val,
};

pub type TransformMode = Mode<Transform, ValMode>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Mode<G, S> {
    Generic(G),
    Specific(S),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct ValMode {
    pub symbol: Transform,
    pub pair: Box<Pair<TransformMode, TransformMode>>,
    pub call: Mode<Transform, Box<CallMode>>,
    pub reverse: Mode<Transform, Box<ReverseMode>>,
    pub list: Box<ListMode>,
    pub map: Box<MapMode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CallMode {
    Call(Call<TransformMode, TransformMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReverseMode {
    Reverse(Reverse<TransformMode, TransformMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ListMode {
    ForAll(TransformMode),
    ForSome(List<ListItemMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListItemMode {
    pub mode: TransformMode,
    pub ellipsis: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MapMode {
    ForAll(Pair<TransformMode, TransformMode>),
    ForSome(Map<Val, TransformMode>),
}

impl<G: Default, S> Default for Mode<G, S> {
    fn default() -> Self {
        Mode::Generic(Default::default())
    }
}

impl Default for ListMode {
    fn default() -> Self {
        ListMode::ForAll(Default::default())
    }
}

impl Default for MapMode {
    fn default() -> Self {
        MapMode::ForAll(Default::default())
    }
}

impl<Ctx, G, S> Transformer<Ctx, Val, Val> for Mode<G, S>
where
    Ctx: CtxAccessor,
    G: Transformer<Ctx, Val, Val>,
    S: Transformer<Ctx, Val, Val>,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            Mode::Generic(mode) => mode.transform(ctx, input),
            Mode::Specific(mode) => mode.transform(ctx, input),
        }
    }
}

impl<'a, Ctx, G, S> Transformer<Ctx, &'a Val, Val> for Mode<G, S>
where
    Ctx: CtxAccessor,
    G: Transformer<Ctx, &'a Val, Val>,
    S: Transformer<Ctx, &'a Val, Val>,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            Mode::Generic(mode) => mode.transform(ctx, input),
            Mode::Specific(mode) => mode.transform(ctx, input),
        }
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for ValMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(_) => self.symbol.transform(ctx, input),
            Val::Pair(pair) => self.pair.transform(ctx, *pair),
            Val::Call(call) => self.call.transform(ctx, call),
            Val::Reverse(reverse) => self.reverse.transform(ctx, reverse),
            Val::List(list) => self.list.transform(ctx, list),
            Val::Map(map) => self.map.transform(ctx, map),
            val => val,
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a Val, Val> for ValMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match input {
            Val::Symbol(_) => self.symbol.transform(ctx, input),
            Val::Pair(pair) => self.pair.transform(ctx, &**pair),
            Val::Call(call) => self.call.transform(ctx, (&**call, input)),
            Val::Reverse(reverse) => self.reverse.transform(ctx, (&**reverse, input)),
            Val::List(list) => self.list.transform(ctx, list),
            Val::Map(map) => self.map.transform(ctx, map),
            val => val.clone(),
        }
    }
}

impl<Ctx> Transformer<Ctx, PairVal, Val> for Pair<TransformMode, TransformMode>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: PairVal) -> Val {
        let first = self.first.transform(ctx, input.first);
        let second = self.second.transform(ctx, input.second);
        ValBuilder.from_pair(first, second)
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a PairVal, Val> for Pair<TransformMode, TransformMode>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a PairVal) -> Val {
        let first = self.first.transform(ctx, &input.first);
        let second = self.second.transform(ctx, &input.second);
        ValBuilder.from_pair(first, second)
    }
}

impl<Ctx> Transformer<Ctx, CallVal, Val> for CallMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: CallVal) -> Val {
        match self {
            CallMode::Call(call) => {
                let func = call.func.transform(ctx, input.func);
                let input = call.input.transform(ctx, input.input);
                ValBuilder.from_call(func, input)
            }
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a CallVal, Val> for CallMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a CallVal) -> Val {
        match self {
            CallMode::Call(call) => {
                let func = call.func.transform(ctx, &input.func);
                let input = call.input.transform(ctx, &input.input);
                ValBuilder.from_call(func, input)
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, Box<CallVal>, Val> for Mode<Transform, Box<CallMode>>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Box<CallVal>) -> Val {
        match self {
            Mode::Generic(mode) => mode.transform(ctx, Val::Call(input)),
            Mode::Specific(mode) => mode.transform(ctx, *input),
        }
    }
}

impl<Ctx> Transformer<Ctx, ReverseVal, Val> for ReverseMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: ReverseVal) -> Val {
        match self {
            ReverseMode::Reverse(reverse) => {
                let func = reverse.func.transform(ctx, input.func);
                let output = reverse.output.transform(ctx, input.output);
                ValBuilder.from_reverse(func, output)
            }
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a ReverseVal, Val> for ReverseMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a ReverseVal) -> Val {
        match self {
            ReverseMode::Reverse(reverse) => {
                let func = reverse.func.transform(ctx, &input.func);
                let output = reverse.output.transform(ctx, &input.output);
                ValBuilder.from_reverse(func, output)
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, Box<ReverseVal>, Val> for Mode<Transform, Box<ReverseMode>>
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Box<ReverseVal>) -> Val {
        match self {
            Mode::Generic(mode) => mode.transform(ctx, Val::Reverse(input)),
            Mode::Specific(mode) => mode.transform(ctx, *input),
        }
    }
}

impl<Ctx> Transformer<Ctx, ListVal, Val> for ListMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, val_list: ListVal) -> Val {
        match self {
            ListMode::ForAll(mode) => {
                let list = val_list
                    .into_iter()
                    .map(|val| mode.transform(ctx, val))
                    .collect();
                Val::List(list)
            }
            ListMode::ForSome(mode_list) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(mode) = mode_iter.next() {
                    if mode.ellipsis {
                        let name_len = mode_iter.len();
                        let val_len = val_iter.len();
                        if val_len > name_len {
                            for _ in 0..(val_len - name_len) {
                                let val = val_iter.next().unwrap();
                                let val = mode.mode.transform(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.mode.transform(ctx, val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(Eval.transform(ctx, val));
                }
                ValBuilder.from_list(list.into_iter())
            }
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a ListVal, Val> for ListMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, val_list: &'a ListVal) -> Val {
        match self {
            ListMode::ForAll(mode) => {
                let list = val_list
                    .into_iter()
                    .map(|val| mode.transform(ctx, val))
                    .collect();
                Val::List(list)
            }
            ListMode::ForSome(mode_list) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(mode) = mode_iter.next() {
                    if mode.ellipsis {
                        let name_len = mode_iter.len();
                        let val_len = val_iter.len();
                        if val_len > name_len {
                            for _ in 0..(val_len - name_len) {
                                let val = val_iter.next().unwrap();
                                let val = mode.mode.transform(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.mode.transform(ctx, val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(EvalByRef.transform(ctx, val));
                }
                ValBuilder.from_list(list.into_iter())
            }
        }
    }
}

impl<Ctx> Transformer<Ctx, MapVal, Val> for MapMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, val_map: MapVal) -> Val {
        match self {
            MapMode::ForAll(mode) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.transform(ctx, k);
                    let v = mode.second.transform(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            MapMode::ForSome(mode_map) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(&k) {
                        mode.transform(ctx, v)
                    } else {
                        Eval.transform(ctx, v)
                    };
                    let k = Id.transform(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a MapVal, Val> for MapMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, val_map: &'a MapVal) -> Val {
        match self {
            MapMode::ForAll(mode) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.transform(ctx, k);
                    let v = mode.second.transform(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            MapMode::ForSome(mode_map) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(k) {
                        mode.transform(ctx, v)
                    } else {
                        EvalByRef.transform(ctx, v)
                    };
                    let k = IdByRef.transform(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
        }
    }
}

impl<'a, Ctx, T, S> Transformer<Ctx, (&'a T, &'a Val), Val> for Mode<Transform, S>
where
    Ctx: CtxAccessor,
    S: Transformer<Ctx, &'a T, Val>,
{
    fn transform(&self, ctx: &mut Ctx, input: (&'a T, &'a Val)) -> Val {
        match self {
            Mode::Generic(mode) => mode.transform(ctx, input.1),
            Mode::Specific(mode) => mode.transform(ctx, input.0),
        }
    }
}

impl<Ctx, I, O, T> Transformer<Ctx, I, O> for Box<T>
where
    Ctx: CtxAccessor,
    T: Transformer<Ctx, I, O>,
{
    fn transform(&self, ctx: &mut Ctx, input: I) -> O {
        self.deref().transform(ctx, input)
    }
}

impl TransformMode {
    pub fn apply(&self, mut ctx: CtxForMutableFn, val: Val) -> Val {
        self.transform(&mut ctx, val)
    }
}

impl<G, S> Mode<G, Box<S>> {
    pub fn new(t: S) -> Self {
        Mode::Specific(Box::new(t))
    }
}

pub(crate) mod repr;
