use crate::{
    call::Call,
    ctx_access::CtxAccessor,
    eval::{
        output::OutputBuilder,
        Evaluator,
        ValBuilder,
    },
    eval_mode::{
        more::{
            More,
            MoreByRef,
        },
        value::{
            Value,
            ValueByRef,
        },
        EvalMode,
    },
    list::List,
    map::Map,
    pair::Pair,
    reverse::Reverse,
    Val,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum IoMode {
    Any(EvalMode),
    Symbol(EvalMode),
    Pair(Box<Pair<IoMode, IoMode>>),
    Call(Box<Call<IoMode, IoMode>>),
    Reverse(Box<Reverse<IoMode, IoMode>>),
    List(EvalMode),
    ListForAll(Box<IoMode>),
    ListForSome(List<ListItemIoMode>),
    Map(EvalMode),
    MapForAll(Box<Pair<IoMode, IoMode>>),
    MapForSome(Map<Val, IoMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct ListItemIoMode {
    pub(crate) io_mode: IoMode,
    pub(crate) ellipsis: bool,
}

impl<Ctx> Evaluator<Ctx, Val, Val> for IoMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match (self, input) {
            (IoMode::Any(mode), input) => mode.eval(ctx, input),
            (IoMode::Symbol(mode), Val::Symbol(s)) => mode.eval(ctx, Val::Symbol(s)),
            (IoMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair.first.eval(ctx, val_pair.first);
                let second = mode_pair.second.eval(ctx, val_pair.second);
                ValBuilder.from_pair(first, second)
            }
            (IoMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call.func.eval(ctx, val_call.func);
                let input = mode_call.input.eval(ctx, val_call.input);
                ValBuilder.from_call(func, input)
            }
            (IoMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse.func.eval(ctx, val_reverse.func);
                let output = mode_reverse.output.eval(ctx, val_reverse.output);
                ValBuilder.from_reverse(func, output)
            }
            (IoMode::List(mode), Val::List(val_list)) => mode.eval(ctx, Val::List(val_list)),
            (IoMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list.into_iter().map(|v| mode.eval(ctx, v));
                ValBuilder.from_list(list)
            }
            (IoMode::ListForSome(mode_list), Val::List(val_list)) => {
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
                    list.push(More.eval(ctx, val));
                }
                ValBuilder.from_list(list.into_iter())
            }
            (IoMode::Map(mode), Val::Map(val_map)) => mode.eval(ctx, Val::Map(val_map)),
            (IoMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval(ctx, k);
                    let v = mode.second.eval(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            (IoMode::MapForSome(mode_map), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(&k) {
                        mode.eval(ctx, v)
                    } else {
                        More.eval(ctx, v)
                    };
                    let k = Value.eval(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            (_, input) => More.eval(ctx, input),
        }
    }
}

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for IoMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match (self, input) {
            (IoMode::Any(mode), input) => mode.eval(ctx, input),
            (IoMode::Symbol(mode), Val::Symbol(_)) => mode.eval(ctx, input),
            (IoMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair.first.eval(ctx, &val_pair.first);
                let second = mode_pair.second.eval(ctx, &val_pair.second);
                ValBuilder.from_pair(first, second)
            }
            (IoMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call.func.eval(ctx, &val_call.func);
                let input = mode_call.input.eval(ctx, &val_call.input);
                ValBuilder.from_call(func, input)
            }
            (IoMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse.func.eval(ctx, &val_reverse.func);
                let output = mode_reverse.output.eval(ctx, &val_reverse.output);
                ValBuilder.from_reverse(func, output)
            }
            (IoMode::List(mode), Val::List(_)) => mode.eval(ctx, input),
            (IoMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list.into_iter().map(|v| mode.eval(ctx, v));
                ValBuilder.from_list(list)
            }
            (IoMode::ListForSome(mode_list), Val::List(val_list)) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(val) = val_iter.next() {
                    if let Some(mode) = mode_iter.next() {
                        let val = mode.io_mode.eval(ctx, val);
                        list.push(val);
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
                        }
                    } else {
                        list.push(MoreByRef.eval(ctx, val));
                    }
                }
                ValBuilder.from_list(list.into_iter())
            }
            (IoMode::Map(mode), Val::Map(_)) => mode.eval(ctx, input),
            (IoMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval(ctx, k);
                    let v = mode.second.eval(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            (IoMode::MapForSome(mode_map), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let v = if let Some(mode) = mode_map.get(k) {
                        mode.eval(ctx, v)
                    } else {
                        MoreByRef.eval(ctx, v)
                    };
                    let k = ValueByRef.eval(ctx, k);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            (_, input) => MoreByRef.eval(ctx, input),
        }
    }
}
