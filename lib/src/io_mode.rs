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
    CtxForMutableFn,
    Val,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum IoMode {
    Transform(Transform),
    Match(MatchMode),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatchMode {
    pub symbol: Transform,
    pub pair: Box<PairMode>,
    pub call: Box<CallMode>,
    pub reverse: Box<ReverseMode>,
    pub list: Box<ListMode>,
    pub map: Box<MapMode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PairMode {
    Transform(Transform),
    Pair(Pair<IoMode, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CallMode {
    Transform(Transform),
    Call(Call<IoMode, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReverseMode {
    Transform(Transform),
    Reverse(Reverse<IoMode, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ListMode {
    Transform(Transform),
    ForAll(IoMode),
    ForSome(List<ListItemMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListItemMode {
    pub io_mode: IoMode,
    pub ellipsis: bool,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum MapMode {
    Transform(Transform),
    ForAll(Pair<IoMode, IoMode>),
    ForSome(Map<Val, IoMode>),
}

impl Default for IoMode {
    fn default() -> Self {
        IoMode::Transform(Transform::Eval)
    }
}

impl Default for PairMode {
    fn default() -> Self {
        PairMode::Transform(Transform::Eval)
    }
}

impl Default for CallMode {
    fn default() -> Self {
        CallMode::Transform(Transform::Eval)
    }
}

impl Default for ReverseMode {
    fn default() -> Self {
        ReverseMode::Transform(Transform::Eval)
    }
}

impl Default for ListMode {
    fn default() -> Self {
        ListMode::Transform(Transform::Eval)
    }
}

impl Default for MapMode {
    fn default() -> Self {
        MapMode::Transform(Transform::Eval)
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for IoMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            IoMode::Transform(mode) => mode.transform(ctx, input),
            IoMode::Match(mode) => mode.transform(ctx, input),
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a Val, Val> for IoMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            IoMode::Transform(mode) => mode.transform(ctx, input),
            IoMode::Match(mode) => mode.transform(ctx, input),
        }
    }
}

impl<Ctx> Transformer<Ctx, Val, Val> for MatchMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(_) => self.symbol.transform(ctx, input),
            Val::Pair(pair) => match &*self.pair {
                PairMode::Transform(mode) => mode.transform(ctx, Val::Pair(pair)),
                PairMode::Pair(pair_mode) => {
                    let first = pair_mode.first.transform(ctx, pair.first);
                    let second = pair_mode.second.transform(ctx, pair.second);
                    ValBuilder.from_pair(first, second)
                }
            },
            Val::Call(call) => match &*self.call {
                CallMode::Transform(mode) => mode.transform(ctx, Val::Call(call)),
                CallMode::Call(call_mode) => {
                    let func = call_mode.func.transform(ctx, call.func);
                    let input = call_mode.input.transform(ctx, call.input);
                    ValBuilder.from_call(func, input)
                }
            },
            Val::Reverse(reverse) => match &*self.reverse {
                ReverseMode::Transform(mode) => mode.transform(ctx, Val::Reverse(reverse)),
                ReverseMode::Reverse(reverse_mode) => {
                    let func = reverse_mode.func.transform(ctx, reverse.func);
                    let output = reverse_mode.output.transform(ctx, reverse.output);
                    ValBuilder.from_reverse(func, output)
                }
            },
            Val::List(_) => self.list.transform(ctx, input),
            Val::Map(_) => self.map.transform(ctx, input),
            val => val,
        }
    }
}

impl<'a, Ctx> Transformer<Ctx, &'a Val, Val> for MatchMode
where
    Ctx: CtxAccessor,
{
    fn transform(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match input {
            Val::Symbol(_) => self.symbol.transform(ctx, input),
            Val::Pair(pair) => match &*self.pair {
                PairMode::Transform(mode) => mode.transform(ctx, input),
                PairMode::Pair(pair_mode) => {
                    let first = pair_mode.first.transform(ctx, &pair.first);
                    let second = pair_mode.second.transform(ctx, &pair.second);
                    ValBuilder.from_pair(first, second)
                }
            },
            Val::Call(call) => match &*self.call {
                CallMode::Transform(mode) => mode.transform(ctx, input),
                CallMode::Call(call_mode) => {
                    let func = call_mode.func.transform(ctx, &call.func);
                    let input = call_mode.input.transform(ctx, &call.input);
                    ValBuilder.from_call(func, input)
                }
            },
            Val::Reverse(reverse) => match &*self.reverse {
                ReverseMode::Transform(mode) => mode.transform(ctx, input),
                ReverseMode::Reverse(reverse_mode) => {
                    let func = reverse_mode.func.transform(ctx, &reverse.func);
                    let output = reverse_mode.output.transform(ctx, &reverse.output);
                    ValBuilder.from_reverse(func, output)
                }
            },
            Val::List(_) => self.list.transform_by_ref(ctx, input),
            Val::Map(_) => self.map.transform_by_ref(ctx, input),
            val => val.clone(),
        }
    }
}

impl ListMode {
    fn transform<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, list: Val) -> Val {
        match self {
            ListMode::Transform(mode) => mode.transform(ctx, list),
            ListMode::ForAll(mode) => {
                let Val::List(list) = list else {
                    unreachable!()
                };
                let list = list
                    .into_iter()
                    .map(|val| mode.transform(ctx, val))
                    .collect();
                Val::List(list)
            }
            ListMode::ForSome(mode_list) => {
                let Val::List(val_list) = list else {
                    unreachable!()
                };
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
                                let val = mode.io_mode.transform(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.io_mode.transform(ctx, val);
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

impl ListMode {
    fn transform_by_ref<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, list: &Val) -> Val {
        match self {
            ListMode::Transform(mode) => mode.transform(ctx, list),
            ListMode::ForAll(mode) => {
                let Val::List(list) = list else {
                    unreachable!()
                };
                let list = list
                    .into_iter()
                    .map(|val| mode.transform(ctx, val))
                    .collect();
                Val::List(list)
            }
            ListMode::ForSome(mode_list) => {
                let Val::List(val_list) = list else {
                    unreachable!()
                };
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
                                let val = mode.io_mode.transform(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.io_mode.transform(ctx, val);
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

impl MapMode {
    fn transform<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, map: Val) -> Val {
        match self {
            MapMode::Transform(mode) => mode.transform(ctx, map),
            MapMode::ForAll(mode) => {
                let Val::Map(val_map) = map else {
                    unreachable!()
                };
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.transform(ctx, k);
                    let v = mode.second.transform(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            MapMode::ForSome(mode_map) => {
                let Val::Map(val_map) = map else {
                    unreachable!()
                };
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

impl MapMode {
    fn transform_by_ref<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, map: &Val) -> Val {
        match self {
            MapMode::Transform(mode) => mode.transform(ctx, map),
            MapMode::ForAll(mode) => {
                let Val::Map(val_map) = map else {
                    unreachable!()
                };
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.transform(ctx, k);
                    let v = mode.second.transform(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            MapMode::ForSome(mode_map) => {
                let Val::Map(val_map) = map else {
                    unreachable!()
                };
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

impl IoMode {
    pub fn apply(&self, mut ctx: CtxForMutableFn, val: Val) -> Val {
        self.transform(&mut ctx, val)
    }
}
