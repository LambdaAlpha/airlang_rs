use crate::{
    call::Call,
    ctx_access::CtxAccessor,
    eval::{
        output::OutputBuilder,
        Evaluator,
        ValBuilder,
    },
    eval_mode::{
        eager::{
            Eager,
            EagerByRef,
        },
        id::{
            Id,
            IdByRef,
        },
        EvalMode,
    },
    list::List,
    map::Map,
    pair::Pair,
    reverse::Reverse,
    CtxForMutableFn,
    Val,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum IoMode {
    Eval(EvalMode),
    Match(MatchMode),
}

#[derive(Default, Debug, Clone, Eq, PartialEq, Hash)]
pub struct MatchMode {
    pub symbol: EvalMode,
    pub pair: Box<PairMode>,
    pub call: Box<CallMode>,
    pub reverse: Box<ReverseMode>,
    pub list: Box<ListMode>,
    pub map: Box<MapMode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PairMode {
    Eval(EvalMode),
    Pair(Pair<IoMode, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CallMode {
    Eval(EvalMode),
    Call(Call<IoMode, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ReverseMode {
    Eval(EvalMode),
    Reverse(Reverse<IoMode, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ListMode {
    Eval(EvalMode),
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
    Eval(EvalMode),
    ForAll(Pair<IoMode, IoMode>),
    ForSome(Map<Val, IoMode>),
}

impl Default for IoMode {
    fn default() -> Self {
        IoMode::Eval(EvalMode::Eager)
    }
}

impl Default for PairMode {
    fn default() -> Self {
        PairMode::Eval(EvalMode::Eager)
    }
}

impl Default for CallMode {
    fn default() -> Self {
        CallMode::Eval(EvalMode::Eager)
    }
}

impl Default for ReverseMode {
    fn default() -> Self {
        ReverseMode::Eval(EvalMode::Eager)
    }
}

impl Default for ListMode {
    fn default() -> Self {
        ListMode::Eval(EvalMode::Eager)
    }
}

impl Default for MapMode {
    fn default() -> Self {
        MapMode::Eval(EvalMode::Eager)
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for IoMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match self {
            IoMode::Eval(mode) => mode.eval(ctx, input),
            IoMode::Match(mode) => mode.eval(ctx, input),
        }
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for IoMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match self {
            IoMode::Eval(mode) => mode.eval(ctx, input),
            IoMode::Match(mode) => mode.eval(ctx, input),
        }
    }
}

impl<Ctx> Evaluator<Ctx, Val, Val> for MatchMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match input {
            Val::Symbol(_) => self.symbol.eval(ctx, input),
            Val::Pair(pair) => match &*self.pair {
                PairMode::Eval(mode) => mode.eval(ctx, Val::Pair(pair)),
                PairMode::Pair(pair_mode) => {
                    let first = pair_mode.first.eval(ctx, pair.first);
                    let second = pair_mode.second.eval(ctx, pair.second);
                    ValBuilder.from_pair(first, second)
                }
            },
            Val::Call(call) => match &*self.call {
                CallMode::Eval(mode) => mode.eval(ctx, Val::Call(call)),
                CallMode::Call(call_mode) => {
                    let func = call_mode.func.eval(ctx, call.func);
                    let input = call_mode.input.eval(ctx, call.input);
                    ValBuilder.from_call(func, input)
                }
            },
            Val::Reverse(reverse) => match &*self.reverse {
                ReverseMode::Eval(mode) => mode.eval(ctx, Val::Reverse(reverse)),
                ReverseMode::Reverse(reverse_mode) => {
                    let func = reverse_mode.func.eval(ctx, reverse.func);
                    let output = reverse_mode.output.eval(ctx, reverse.output);
                    ValBuilder.from_reverse(func, output)
                }
            },
            Val::List(_) => self.list.eval(ctx, input),
            Val::Map(_) => self.map.eval(ctx, input),
            val => val,
        }
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for MatchMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match input {
            Val::Symbol(_) => self.symbol.eval(ctx, input),
            Val::Pair(pair) => match &*self.pair {
                PairMode::Eval(mode) => mode.eval(ctx, input),
                PairMode::Pair(pair_mode) => {
                    let first = pair_mode.first.eval(ctx, &pair.first);
                    let second = pair_mode.second.eval(ctx, &pair.second);
                    ValBuilder.from_pair(first, second)
                }
            },
            Val::Call(call) => match &*self.call {
                CallMode::Eval(mode) => mode.eval(ctx, input),
                CallMode::Call(call_mode) => {
                    let func = call_mode.func.eval(ctx, &call.func);
                    let input = call_mode.input.eval(ctx, &call.input);
                    ValBuilder.from_call(func, input)
                }
            },
            Val::Reverse(reverse) => match &*self.reverse {
                ReverseMode::Eval(mode) => mode.eval(ctx, input),
                ReverseMode::Reverse(reverse_mode) => {
                    let func = reverse_mode.func.eval(ctx, &reverse.func);
                    let output = reverse_mode.output.eval(ctx, &reverse.output);
                    ValBuilder.from_reverse(func, output)
                }
            },
            Val::List(_) => self.list.eval_by_ref(ctx, input),
            Val::Map(_) => self.map.eval_by_ref(ctx, input),
            val => val.clone(),
        }
    }
}

impl ListMode {
    fn eval<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, list: Val) -> Val {
        match self {
            ListMode::Eval(mode) => mode.eval(ctx, list),
            ListMode::ForAll(mode) => {
                let Val::List(list) = list else {
                    unreachable!()
                };
                let list = list.into_iter().map(|val| mode.eval(ctx, val)).collect();
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
                                let val = mode.io_mode.eval(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.io_mode.eval(ctx, val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(Eager.eval(ctx, val));
                }
                ValBuilder.from_list(list.into_iter())
            }
        }
    }
}

impl ListMode {
    fn eval_by_ref<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, list: &Val) -> Val {
        match self {
            ListMode::Eval(mode) => mode.eval(ctx, list),
            ListMode::ForAll(mode) => {
                let Val::List(list) = list else {
                    unreachable!()
                };
                let list = list.into_iter().map(|val| mode.eval(ctx, val)).collect();
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
                                let val = mode.io_mode.eval(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.io_mode.eval(ctx, val);
                        list.push(val);
                    } else {
                        break;
                    }
                }
                for val in val_iter {
                    list.push(EagerByRef.eval(ctx, val));
                }
                ValBuilder.from_list(list.into_iter())
            }
        }
    }
}

impl MapMode {
    fn eval<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, map: Val) -> Val {
        match self {
            MapMode::Eval(mode) => mode.eval(ctx, map),
            MapMode::ForAll(mode) => {
                let Val::Map(val_map) = map else {
                    unreachable!()
                };
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval(ctx, k);
                    let v = mode.second.eval(ctx, v);
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
                        mode.eval(ctx, v)
                    } else {
                        Eager.eval(ctx, v)
                    };
                    let k = Id.eval(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
        }
    }
}

impl MapMode {
    fn eval_by_ref<Ctx: CtxAccessor>(&self, ctx: &mut Ctx, map: &Val) -> Val {
        match self {
            MapMode::Eval(mode) => mode.eval(ctx, map),
            MapMode::ForAll(mode) => {
                let Val::Map(val_map) = map else {
                    unreachable!()
                };
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval(ctx, k);
                    let v = mode.second.eval(ctx, v);
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
                        mode.eval(ctx, v)
                    } else {
                        EagerByRef.eval(ctx, v)
                    };
                    let k = IdByRef.eval(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
        }
    }
}

impl IoMode {
    pub fn apply(&self, mut ctx: CtxForMutableFn, val: Val) -> Val {
        self.eval(&mut ctx, val)
    }
}
