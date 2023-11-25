use crate::{
    semantics::{
        eval_mode::EvalMode,
        input_mode::{
            InputMode,
            ListItemInputMode,
        },
        val::{
            ListVal,
            MapVal,
            PairVal,
            ReverseVal,
        },
        Val,
    },
    types::{
        Bool,
        Call,
        Pair,
        Reverse,
        Symbol,
        Unit,
    },
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

pub(crate) fn parse_input_mode(input_mode: Val) -> Option<InputMode> {
    match input_mode {
        Val::Unit(_) => Some(InputMode::Any(EvalMode::Value)),
        Val::Bool(b) => Some(InputMode::Any(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Pair(pair) => parse_input_mode_pair(*pair),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            parse_input_mode_tag(&tag, call.input)
        }
        Val::Reverse(reverse) => parse_input_mode_reverse(*reverse),
        Val::List(list) => parse_input_mode_list_for_some(list),
        Val::Map(map) => parse_input_mode_map_for_some(map),
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

fn parse_input_mode_tag(tag: &str, input_mode: Val) -> Option<InputMode> {
    match tag {
        SYMBOL => parse_input_mode_symbol(input_mode),
        CALL => parse_input_mode_call(input_mode),
        LIST => parse_input_mode_list(input_mode),
        LIST_FOR_ALL => parse_input_mode_list_for_all(input_mode),
        MAP => parse_input_mode_map(input_mode),
        MAP_FOR_ALL => parse_input_mode_map_for_all(input_mode),
        _ => None,
    }
}

fn parse_input_mode_symbol(eval_mode: Val) -> Option<InputMode> {
    Some(InputMode::Symbol(parse_eval_mode(eval_mode)?))
}

fn parse_input_mode_pair(input_mode: PairVal) -> Option<InputMode> {
    let first = parse_input_mode(input_mode.first)?;
    let second = parse_input_mode(input_mode.second)?;
    let pair = InputMode::Pair(Box::new(Pair::new(first, second)));
    Some(pair)
}

fn parse_input_mode_call(input_mode: Val) -> Option<InputMode> {
    let Val::Call(call) = input_mode else {
        return None;
    };
    let func = parse_input_mode(call.func)?;
    let input = parse_input_mode(call.input)?;
    let call = InputMode::Call(Box::new(Call::new(func, input)));
    Some(call)
}

fn parse_input_mode_reverse(input_mode: ReverseVal) -> Option<InputMode> {
    let func = parse_input_mode(input_mode.func)?;
    let output = parse_input_mode(input_mode.output)?;
    let reverse = InputMode::Reverse(Box::new(Reverse::new(func, output)));
    Some(reverse)
}

fn parse_input_mode_list(eval_mode: Val) -> Option<InputMode> {
    Some(InputMode::List(parse_eval_mode(eval_mode)?))
}

fn parse_input_mode_list_for_all(input_mode: Val) -> Option<InputMode> {
    let input_mode = parse_input_mode(input_mode)?;
    Some(InputMode::ListForAll(Box::new(input_mode)))
}

fn parse_input_mode_list_for_some(input_mode: ListVal) -> Option<InputMode> {
    let list = input_mode
        .into_iter()
        .map(parse_list_item_input_mode)
        .try_collect()?;
    let list = InputMode::ListForSome(list);
    Some(list)
}

fn parse_list_item_input_mode(input_mode: Val) -> Option<ListItemInputMode> {
    let Val::Call(call) = input_mode else {
        let input_mode = parse_input_mode(input_mode)?;
        return Some(ListItemInputMode {
            input_mode,
            ellipsis: false,
        });
    };
    let Val::Symbol(tag) = call.func else {
        return None;
    };
    let input_mode = if &*tag == ELLIPSIS {
        let input_mode = parse_input_mode(call.input)?;
        ListItemInputMode {
            input_mode,
            ellipsis: true,
        }
    } else {
        let input_mode = parse_input_mode_tag(&tag, call.input)?;
        ListItemInputMode {
            input_mode,
            ellipsis: false,
        }
    };
    Some(input_mode)
}

fn parse_input_mode_map(eval_mode: Val) -> Option<InputMode> {
    Some(InputMode::Map(parse_eval_mode(eval_mode)?))
}

fn parse_input_mode_map_for_all(input_mode: Val) -> Option<InputMode> {
    let Val::Pair(pair) = input_mode else {
        return None;
    };
    let key = parse_input_mode(pair.first)?;
    let value = parse_input_mode(pair.second)?;
    Some(InputMode::MapForAll(Box::new(Pair::new(key, value))))
}

fn parse_input_mode_map_for_some(input_mode: MapVal) -> Option<InputMode> {
    let map = input_mode
        .into_iter()
        .map(|(k, v)| {
            let input_mode = parse_input_mode(v)?;
            Some((k, input_mode))
        })
        .try_collect()?;
    let map = InputMode::MapForSome(map);
    Some(map)
}

pub(crate) fn eval_mode_to_val(eval_mode: EvalMode) -> Val {
    match eval_mode {
        EvalMode::Value => Val::Unit(Unit),
        EvalMode::More => Val::Bool(Bool::t()),
        EvalMode::Less => Val::Bool(Bool::f()),
    }
}

pub(crate) fn generate_input_mode(input_mode: &InputMode) -> Val {
    match input_mode {
        InputMode::Any(mode) => eval_mode_to_val(*mode),
        InputMode::Symbol(mode) => {
            let tag = symbol(SYMBOL);
            let mode = eval_mode_to_val(*mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        InputMode::Pair(pair) => {
            let first = generate_input_mode(&pair.first);
            let second = generate_input_mode(&pair.second);
            Val::Pair(Box::new(Pair::new(first, second)))
        }
        InputMode::Call(call) => {
            let tag = symbol(CALL);
            let func = generate_input_mode(&call.func);
            let input = generate_input_mode(&call.input);
            let call = Val::Call(Box::new(Call::new(func, input)));
            Val::Call(Box::new(Call::new(tag, call)))
        }
        InputMode::Reverse(reverse) => {
            let func = generate_input_mode(&reverse.func);
            let output = generate_input_mode(&reverse.output);
            Val::Reverse(Box::new(Reverse::new(func, output)))
        }
        InputMode::List(mode) => {
            let tag = symbol(LIST);
            let mode = eval_mode_to_val(*mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        InputMode::ListForAll(mode) => {
            let tag = symbol(LIST_FOR_ALL);
            let mode = generate_input_mode(mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        InputMode::ListForSome(list) => {
            let list = list
                .iter()
                .map(|mode| {
                    if mode.ellipsis {
                        let tag = symbol(ELLIPSIS);
                        let mode = generate_input_mode(&mode.input_mode);
                        Val::Call(Box::new(Call::new(tag, mode)))
                    } else {
                        generate_input_mode(&mode.input_mode)
                    }
                })
                .collect();
            Val::List(list)
        }
        InputMode::Map(mode) => {
            let tag = symbol(MAP);
            let mode = eval_mode_to_val(*mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        InputMode::MapForAll(mode) => {
            let tag = symbol(MAP_FOR_ALL);
            let key = generate_input_mode(&mode.first);
            let value = generate_input_mode(&mode.second);
            let pair = Val::Pair(Box::new(Pair::new(key, value)));
            Val::Call(Box::new(Call::new(tag, pair)))
        }
        InputMode::MapForSome(map) => {
            let map = map
                .iter()
                .map(|(k, v)| {
                    let mode = generate_input_mode(v);
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
