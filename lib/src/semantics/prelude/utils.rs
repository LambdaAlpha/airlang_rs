use crate::{
    semantics::{
        eval_mode::{
            BasicEvalMode,
            EvalMode,
            ListItemEvalMode,
        },
        prelude::names,
        val::{
            ListVal,
            MapVal,
            PairVal,
            ReverseVal,
        },
        Val,
    },
    types::{
        Call,
        Pair,
        Reverse,
        Symbol,
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

pub(crate) fn parse_eval_mode(eval_mode: Val) -> Option<EvalMode> {
    match eval_mode {
        Val::Symbol(s) => parse_eval_mode_any(s),
        Val::Pair(pair) => parse_eval_mode_pair(*pair),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            parse_eval_mode_tag(&tag, call.input)
        }
        Val::Reverse(reverse) => parse_eval_mode_reverse(*reverse),
        Val::List(list) => parse_eval_mode_list_for_some(list),
        Val::Map(map) => parse_eval_mode_map_for_some(map),
        Val::Unit(_) => Some(EvalMode::Any(BasicEvalMode::Eval)),
        _ => None,
    }
}

fn parse_basic_eval_mode(eval_mode: Val) -> Option<BasicEvalMode> {
    let Val::Symbol(s) = eval_mode else {
        return None;
    };
    symbol_to_basic_eval_mode(&s)
}

fn parse_eval_mode_tag(tag: &str, eval_mode: Val) -> Option<EvalMode> {
    match tag {
        SYMBOL => parse_eval_mode_symbol(eval_mode),
        CALL => parse_eval_mode_call(eval_mode),
        LIST => parse_eval_mode_list(eval_mode),
        LIST_FOR_ALL => parse_eval_mode_list_for_all(eval_mode),
        MAP => parse_eval_mode_map(eval_mode),
        MAP_FOR_ALL => parse_eval_mode_map_for_all(eval_mode),
        _ => None,
    }
}

fn parse_eval_mode_any(eval_mode: Symbol) -> Option<EvalMode> {
    Some(EvalMode::Any(symbol_to_basic_eval_mode(&eval_mode)?))
}

fn parse_eval_mode_symbol(eval_mode: Val) -> Option<EvalMode> {
    Some(EvalMode::Symbol(parse_basic_eval_mode(eval_mode)?))
}

fn parse_eval_mode_pair(eval_mode: PairVal) -> Option<EvalMode> {
    let first = parse_eval_mode(eval_mode.first)?;
    let second = parse_eval_mode(eval_mode.second)?;
    let pair = EvalMode::Pair(Box::new(Pair::new(first, second)));
    Some(pair)
}

fn parse_eval_mode_call(eval_mode: Val) -> Option<EvalMode> {
    let Val::Call(call) = eval_mode else {
        return None;
    };
    let func = parse_eval_mode(call.func)?;
    let input = parse_eval_mode(call.input)?;
    let call = EvalMode::Call(Box::new(Call::new(func, input)));
    Some(call)
}

fn parse_eval_mode_reverse(eval_mode: ReverseVal) -> Option<EvalMode> {
    let func = parse_eval_mode(eval_mode.func)?;
    let output = parse_eval_mode(eval_mode.output)?;
    let reverse = EvalMode::Reverse(Box::new(Reverse::new(func, output)));
    Some(reverse)
}

fn parse_eval_mode_list(eval_mode: Val) -> Option<EvalMode> {
    Some(EvalMode::List(parse_basic_eval_mode(eval_mode)?))
}

fn parse_eval_mode_list_for_all(eval_mode: Val) -> Option<EvalMode> {
    let eval_mode = parse_eval_mode(eval_mode)?;
    Some(EvalMode::ListForAll(Box::new(eval_mode)))
}

fn parse_eval_mode_list_for_some(eval_mode: ListVal) -> Option<EvalMode> {
    let list = eval_mode
        .into_iter()
        .map(parse_list_item_eval_mode)
        .try_collect()?;
    let list = EvalMode::ListForSome(list);
    Some(list)
}

fn parse_list_item_eval_mode(eval_mode: Val) -> Option<ListItemEvalMode> {
    let Val::Call(call) = eval_mode else {
        let eval_mode = parse_eval_mode(eval_mode)?;
        return Some(ListItemEvalMode {
            eval_mode,
            ellipsis: false,
        });
    };
    let Val::Symbol(tag) = call.func else {
        return None;
    };
    let eval_mode = if &*tag == ELLIPSIS {
        let eval_mode = parse_eval_mode(call.input)?;
        ListItemEvalMode {
            eval_mode,
            ellipsis: true,
        }
    } else {
        let eval_mode = parse_eval_mode_tag(&tag, call.input)?;
        ListItemEvalMode {
            eval_mode,
            ellipsis: false,
        }
    };
    Some(eval_mode)
}

fn parse_eval_mode_map(eval_mode: Val) -> Option<EvalMode> {
    Some(EvalMode::Map(parse_basic_eval_mode(eval_mode)?))
}

fn parse_eval_mode_map_for_all(eval_mode: Val) -> Option<EvalMode> {
    let Val::Pair(pair) = eval_mode else {
        return None;
    };
    let key = parse_eval_mode(pair.first)?;
    let value = parse_eval_mode(pair.second)?;
    Some(EvalMode::MapForAll(Box::new(Pair::new(key, value))))
}

fn parse_eval_mode_map_for_some(eval_mode: MapVal) -> Option<EvalMode> {
    let map = eval_mode
        .into_iter()
        .map(|(k, v)| {
            let eval_mode = parse_eval_mode(v)?;
            Some((k, eval_mode))
        })
        .try_collect()?;
    let map = EvalMode::MapForSome(map);
    Some(map)
}

fn symbol_to_basic_eval_mode(name: &Symbol) -> Option<BasicEvalMode> {
    let eval_mode = match &**name {
        names::VALUE => BasicEvalMode::Value,
        names::EVAL => BasicEvalMode::Eval,
        names::QUOTE => BasicEvalMode::Quote,
        _ => return None,
    };
    Some(eval_mode)
}

pub(crate) fn basic_eval_mode_to_symbol(eval_mode: BasicEvalMode) -> Symbol {
    let str = match eval_mode {
        BasicEvalMode::Value => names::VALUE,
        BasicEvalMode::Eval => names::EVAL,
        BasicEvalMode::Quote => names::QUOTE,
    };
    Symbol::from_str(str)
}

pub(crate) fn generate_eval_mode(eval_mode: &EvalMode) -> Val {
    match eval_mode {
        EvalMode::Any(mode) => {
            let s = basic_eval_mode_to_symbol(*mode);
            Val::Symbol(s)
        }
        EvalMode::Symbol(mode) => {
            let tag = symbol(SYMBOL);
            let mode = Val::Symbol(basic_eval_mode_to_symbol(*mode));
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        EvalMode::Pair(pair) => {
            let first = generate_eval_mode(&pair.first);
            let second = generate_eval_mode(&pair.second);
            Val::Pair(Box::new(Pair::new(first, second)))
        }
        EvalMode::Call(call) => {
            let tag = symbol(CALL);
            let func = generate_eval_mode(&call.func);
            let input = generate_eval_mode(&call.input);
            let call = Val::Call(Box::new(Call::new(func, input)));
            Val::Call(Box::new(Call::new(tag, call)))
        }
        EvalMode::Reverse(reverse) => {
            let func = generate_eval_mode(&reverse.func);
            let output = generate_eval_mode(&reverse.output);
            Val::Reverse(Box::new(Reverse::new(func, output)))
        }
        EvalMode::List(mode) => {
            let tag = symbol(LIST);
            let mode = basic_eval_mode_to_symbol(*mode);
            Val::Call(Box::new(Call::new(tag, Val::Symbol(mode))))
        }
        EvalMode::ListForAll(mode) => {
            let tag = symbol(LIST_FOR_ALL);
            let mode = generate_eval_mode(mode);
            Val::Call(Box::new(Call::new(tag, mode)))
        }
        EvalMode::ListForSome(list) => {
            let list = list
                .iter()
                .map(|mode| {
                    if mode.ellipsis {
                        let tag = symbol(ELLIPSIS);
                        let mode = generate_eval_mode(&mode.eval_mode);
                        Val::Call(Box::new(Call::new(tag, mode)))
                    } else {
                        generate_eval_mode(&mode.eval_mode)
                    }
                })
                .collect();
            Val::List(list)
        }
        EvalMode::Map(mode) => {
            let tag = symbol(MAP);
            let mode = basic_eval_mode_to_symbol(*mode);
            Val::Call(Box::new(Call::new(tag, Val::Symbol(mode))))
        }
        EvalMode::MapForAll(mode) => {
            let tag = symbol(MAP_FOR_ALL);
            let key = generate_eval_mode(&mode.first);
            let value = generate_eval_mode(&mode.second);
            let pair = Val::Pair(Box::new(Pair::new(key, value)));
            Val::Call(Box::new(Call::new(tag, pair)))
        }
        EvalMode::MapForSome(map) => {
            let map = map
                .iter()
                .map(|(k, v)| {
                    let mode = generate_eval_mode(v);
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
