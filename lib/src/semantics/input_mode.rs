use crate::{
    semantics::{
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
        Val,
    },
    types::{
        Call,
        List,
        Map,
        Pair,
        Reverse,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum InputMode {
    Any(EvalMode),
    Symbol(EvalMode),
    Pair(Box<Pair<InputMode, InputMode>>),
    Call(Box<Call<InputMode, InputMode>>),
    Reverse(Box<Reverse<InputMode, InputMode>>),
    List(EvalMode),
    ListForAll(Box<InputMode>),
    ListForSome(List<ListItemInputMode>),
    Map(EvalMode),
    MapForAll(Box<Pair<InputMode, InputMode>>),
    MapForSome(Map<Val, InputMode>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct ListItemInputMode {
    pub(crate) input_mode: InputMode,
    pub(crate) ellipsis: bool,
}

impl<Ctx> Evaluator<Ctx, Val, Val> for InputMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: Val) -> Val {
        match (self, input) {
            (InputMode::Any(mode), input) => mode.eval(ctx, input),
            (InputMode::Symbol(mode), Val::Symbol(s)) => mode.eval(ctx, Val::Symbol(s)),
            (InputMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair.first.eval(ctx, val_pair.first);
                let second = mode_pair.second.eval(ctx, val_pair.second);
                ValBuilder.from_pair(first, second)
            }
            (InputMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call.func.eval(ctx, val_call.func);
                let input = mode_call.input.eval(ctx, val_call.input);
                ValBuilder.from_call(func, input)
            }
            (InputMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse.func.eval(ctx, val_reverse.func);
                let output = mode_reverse.output.eval(ctx, val_reverse.output);
                ValBuilder.from_reverse(func, output)
            }
            (InputMode::List(mode), Val::List(val_list)) => mode.eval(ctx, Val::List(val_list)),
            (InputMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list.into_iter().map(|v| mode.eval(ctx, v));
                ValBuilder.from_list(list)
            }
            (InputMode::ListForSome(mode_list), Val::List(val_list)) => {
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
                                let val = mode.input_mode.eval(ctx, val);
                                list.push(val);
                            }
                        }
                    } else if let Some(val) = val_iter.next() {
                        let val = mode.input_mode.eval(ctx, val);
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
            (InputMode::Map(mode), Val::Map(val_map)) => mode.eval(ctx, Val::Map(val_map)),
            (InputMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval(ctx, k);
                    let v = mode.second.eval(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            (InputMode::MapForSome(mode_map), Val::Map(val_map)) => {
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

impl<'a, Ctx> Evaluator<Ctx, &'a Val, Val> for InputMode
where
    Ctx: CtxAccessor,
{
    fn eval(&self, ctx: &mut Ctx, input: &'a Val) -> Val {
        match (self, input) {
            (InputMode::Any(mode), input) => mode.eval(ctx, input),
            (InputMode::Symbol(mode), Val::Symbol(_)) => mode.eval(ctx, input),
            (InputMode::Pair(mode_pair), Val::Pair(val_pair)) => {
                let first = mode_pair.first.eval(ctx, &val_pair.first);
                let second = mode_pair.second.eval(ctx, &val_pair.second);
                ValBuilder.from_pair(first, second)
            }
            (InputMode::Call(mode_call), Val::Call(val_call)) => {
                let func = mode_call.func.eval(ctx, &val_call.func);
                let input = mode_call.input.eval(ctx, &val_call.input);
                ValBuilder.from_call(func, input)
            }
            (InputMode::Reverse(mode_reverse), Val::Reverse(val_reverse)) => {
                let func = mode_reverse.func.eval(ctx, &val_reverse.func);
                let output = mode_reverse.output.eval(ctx, &val_reverse.output);
                ValBuilder.from_reverse(func, output)
            }
            (InputMode::List(mode), Val::List(_)) => mode.eval(ctx, input),
            (InputMode::ListForAll(mode), Val::List(val_list)) => {
                let list = val_list.into_iter().map(|v| mode.eval(ctx, v));
                ValBuilder.from_list(list)
            }
            (InputMode::ListForSome(mode_list), Val::List(val_list)) => {
                let mut list = Vec::with_capacity(val_list.len());
                let mut mode_iter = mode_list.into_iter();
                let mut val_iter = val_list.into_iter();
                while let Some(val) = val_iter.next() {
                    if let Some(mode) = mode_iter.next() {
                        let val = mode.input_mode.eval(ctx, val);
                        list.push(val);
                        if mode.ellipsis {
                            let name_len = mode_iter.len();
                            let val_len = val_iter.len();
                            if val_len > name_len {
                                for _ in 0..(val_len - name_len) {
                                    let val = val_iter.next().unwrap();
                                    let val = mode.input_mode.eval(ctx, val);
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
            (InputMode::Map(mode), Val::Map(_)) => mode.eval(ctx, input),
            (InputMode::MapForAll(mode), Val::Map(val_map)) => {
                let map = val_map.into_iter().map(|(k, v)| {
                    let k = mode.first.eval(ctx, k);
                    let v = mode.second.eval(ctx, v);
                    (k, v)
                });
                ValBuilder.from_map(map)
            }
            (InputMode::MapForSome(mode_map), Val::Map(val_map)) => {
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
