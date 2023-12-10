use crate::{
    eval_mode::EvalMode,
    io_mode::{
        IoMode,
        ListItemIoMode,
    },
    types::{
        Bool,
        Call,
        Pair,
        Reverse,
        Symbol,
        Unit,
    },
    val::{
        ListVal,
        MapVal,
        PairVal,
        ReverseVal,
    },
    Val,
};

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

const SYMBOL: &str = "symbol";
const CALL: &str = "call";
const LIST: &str = "list";
const LIST_FOR_ALL: &str = "list_items";
const MAP: &str = "map";
const MAP_FOR_ALL: &str = "map_items";
const ELLIPSIS: &str = "...";

pub(crate) fn parse_io_mode(io_mode: Val) -> Option<IoMode> {
    match io_mode {
        Val::Unit(_) => Some(IoMode::Any(EvalMode::Value)),
        Val::Bool(b) => Some(IoMode::Any(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Pair(pair) => parse_io_mode_pair(*pair),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            parse_io_mode_tag(&tag, call.input)
        }
        Val::Reverse(reverse) => parse_io_mode_reverse(*reverse),
        Val::List(list) => parse_io_mode_list_for_some(list),
        Val::Map(map) => parse_io_mode_map_for_some(map),
        _ => None,
    }
}

fn parse_eval_mode(eval_mode: Val) -> Option<EvalMode> {
    match eval_mode {
        Val::Unit(_) => Some(EvalMode::Value),
        Val::Bool(b) => Some(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        }),
        _ => None,
    }
}

fn parse_io_mode_tag(tag: &str, io_mode: Val) -> Option<IoMode> {
    match tag {
        SYMBOL => parse_io_mode_symbol(io_mode),
        CALL => parse_io_mode_call(io_mode),
        LIST => parse_io_mode_list(io_mode),
        LIST_FOR_ALL => parse_io_mode_list_for_all(io_mode),
        MAP => parse_io_mode_map(io_mode),
        MAP_FOR_ALL => parse_io_mode_map_for_all(io_mode),
        _ => None,
    }
}

fn parse_io_mode_symbol(eval_mode: Val) -> Option<IoMode> {
    Some(IoMode::Symbol(parse_eval_mode(eval_mode)?))
}

fn parse_io_mode_pair(io_mode: PairVal) -> Option<IoMode> {
    let first = parse_io_mode(io_mode.first)?;
    let second = parse_io_mode(io_mode.second)?;
    let pair = IoMode::Pair(Box::new(Pair::new(first, second)));
    Some(pair)
}

fn parse_io_mode_call(io_mode: Val) -> Option<IoMode> {
    let Val::Call(call) = io_mode else {
        return None;
    };
    let func = parse_io_mode(call.func)?;
    let input = parse_io_mode(call.input)?;
    let call = IoMode::Call(Box::new(Call::new(func, input)));
    Some(call)
}

fn parse_io_mode_reverse(io_mode: ReverseVal) -> Option<IoMode> {
    let func = parse_io_mode(io_mode.func)?;
    let output = parse_io_mode(io_mode.output)?;
    let reverse = IoMode::Reverse(Box::new(Reverse::new(func, output)));
    Some(reverse)
}

fn parse_io_mode_list(eval_mode: Val) -> Option<IoMode> {
    Some(IoMode::List(parse_eval_mode(eval_mode)?))
}

fn parse_io_mode_list_for_all(io_mode: Val) -> Option<IoMode> {
    let io_mode = parse_io_mode(io_mode)?;
    Some(IoMode::ListForAll(Box::new(io_mode)))
}

fn parse_io_mode_list_for_some(io_mode: ListVal) -> Option<IoMode> {
    let list = io_mode
        .into_iter()
        .map(parse_list_item_io_mode)
        .try_collect()?;
    let list = IoMode::ListForSome(list);
    Some(list)
}

fn parse_list_item_io_mode(io_mode: Val) -> Option<ListItemIoMode> {
    let Val::Call(call) = io_mode else {
        let io_mode = parse_io_mode(io_mode)?;
        return Some(ListItemIoMode {
            io_mode,
            ellipsis: false,
        });
    };
    let Val::Symbol(tag) = call.func else {
        return None;
    };
    let io_mode = if &*tag == ELLIPSIS {
        let io_mode = parse_io_mode(call.input)?;
        ListItemIoMode {
            io_mode,
            ellipsis: true,
        }
    } else {
        let io_mode = parse_io_mode_tag(&tag, call.input)?;
        ListItemIoMode {
            io_mode,
            ellipsis: false,
        }
    };
    Some(io_mode)
}

fn parse_io_mode_map(eval_mode: Val) -> Option<IoMode> {
    Some(IoMode::Map(parse_eval_mode(eval_mode)?))
}

fn parse_io_mode_map_for_all(io_mode: Val) -> Option<IoMode> {
    let Val::Pair(pair) = io_mode else {
        return None;
    };
    let key = parse_io_mode(pair.first)?;
    let value = parse_io_mode(pair.second)?;
    Some(IoMode::MapForAll(Box::new(Pair::new(key, value))))
}

fn parse_io_mode_map_for_some(io_mode: MapVal) -> Option<IoMode> {
    let map = io_mode
        .into_iter()
        .map(|(k, v)| {
            let io_mode = parse_io_mode(v)?;
            Some((k, io_mode))
        })
        .try_collect()?;
    let map = IoMode::MapForSome(map);
    Some(map)
}

pub(crate) fn eval_mode_to_val(eval_mode: EvalMode) -> Val {
    match eval_mode {
        EvalMode::Value => Val::Unit(Unit),
        EvalMode::More => Val::Bool(Bool::t()),
        EvalMode::Less => Val::Bool(Bool::f()),
    }
}

pub(crate) fn generate_io_mode(io_mode: &IoMode) -> Val {
    match io_mode {
        IoMode::Any(mode) => eval_mode_to_val(*mode),
        IoMode::Symbol(mode) => {
            let tag = symbol(SYMBOL);
            let mode = eval_mode_to_val(*mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        IoMode::Pair(pair) => {
            let first = generate_io_mode(&pair.first);
            let second = generate_io_mode(&pair.second);
            Val::Pair(Box::new(Pair::new(first, second)))
        }
        IoMode::Call(call) => {
            let tag = symbol(CALL);
            let func = generate_io_mode(&call.func);
            let input = generate_io_mode(&call.input);
            let call = Val::Call(Box::new(Call::new(func, input)));
            Val::Call(Box::new(Call::new(tag, call)))
        }
        IoMode::Reverse(reverse) => {
            let func = generate_io_mode(&reverse.func);
            let output = generate_io_mode(&reverse.output);
            Val::Reverse(Box::new(Reverse::new(func, output)))
        }
        IoMode::List(mode) => {
            let tag = symbol(LIST);
            let mode = eval_mode_to_val(*mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        IoMode::ListForAll(mode) => {
            let tag = symbol(LIST_FOR_ALL);
            let mode = generate_io_mode(mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        IoMode::ListForSome(list) => {
            let list = list
                .iter()
                .map(|mode| {
                    if mode.ellipsis {
                        let tag = symbol(ELLIPSIS);
                        let mode = generate_io_mode(&mode.io_mode);
                        Val::Call(Box::new(Call::new(tag, mode)))
                    } else {
                        generate_io_mode(&mode.io_mode)
                    }
                })
                .collect();
            Val::List(list)
        }
        IoMode::Map(mode) => {
            let tag = symbol(MAP);
            let mode = eval_mode_to_val(*mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        IoMode::MapForAll(mode) => {
            let tag = symbol(MAP_FOR_ALL);
            let key = generate_io_mode(&mode.first);
            let value = generate_io_mode(&mode.second);
            let pair = Val::Pair(Box::new(Pair::new(key, value)));
            Val::Call(Box::new(Call::new(tag, pair)))
        }
        IoMode::MapForSome(map) => {
            let map = map
                .iter()
                .map(|(k, v)| {
                    let mode = generate_io_mode(v);
                    (k.clone(), mode)
                })
                .collect();
            Val::Map(map)
        }
    }
}

pub(crate) fn symbol(s: &str) -> Val {
    Val::Symbol(Symbol::from_str(s))
}
