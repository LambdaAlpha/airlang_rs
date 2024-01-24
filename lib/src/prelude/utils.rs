use crate::{
    bool::Bool,
    call::Call,
    eval_mode::EvalMode,
    io_mode::{
        IoMode,
        ListItemMode,
        ListMode,
        MapMode,
        MatchMode,
    },
    pair::Pair,
    reverse::Reverse,
    symbol::Symbol,
    unit::Unit,
    val::{
        list::ListVal,
        map::MapVal,
    },
    CallMode,
    Map,
    PairMode,
    ReverseMode,
    Val,
};

pub(crate) fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

const SYMBOL: &str = "symbol";
const PAIR: &str = "pair";
const CALL: &str = "call";
const REVERSE: &str = "reverse";
const LIST: &str = "list";
const MAP: &str = "map";

const ELLIPSIS: &str = "...";
const FOR_ALL: &str = "all";

pub(crate) fn parse_io_mode(io_mode: Val) -> Option<IoMode> {
    match io_mode {
        Val::Unit(_) => Some(IoMode::Eval(EvalMode::Value)),
        Val::Bool(b) => Some(IoMode::Eval(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Map(map) => Some(IoMode::Match(parse_match_mode(map)?)),
        _ => None,
    }
}

fn parse_match_mode(mut map: MapVal) -> Option<MatchMode> {
    let mut mode = MatchMode::default();
    if let Some(symbol_mode) = map.remove(&symbol(SYMBOL)) {
        mode.symbol = parse_eval_mode(symbol_mode)?;
    }
    if let Some(pair_mode) = map.remove(&symbol(PAIR)) {
        mode.pair = Box::new(parse_pair_mode(pair_mode)?);
    }
    if let Some(call_mode) = map.remove(&symbol(CALL)) {
        mode.call = Box::new(parse_call_mode(call_mode)?);
    }
    if let Some(reverse_mode) = map.remove(&symbol(REVERSE)) {
        mode.reverse = Box::new(parse_reverse_mode(reverse_mode)?);
    }
    if let Some(list_mode) = map.remove(&symbol(LIST)) {
        mode.list = Box::new(parse_list_mode(list_mode)?);
    }
    if let Some(map_mode) = map.remove(&symbol(MAP)) {
        mode.map = Box::new(parse_map_mode(map_mode)?);
    }
    Some(mode)
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

fn parse_pair_mode(io_mode: Val) -> Option<PairMode> {
    match io_mode {
        Val::Unit(_) => Some(PairMode::Eval(EvalMode::Value)),
        Val::Bool(b) => Some(PairMode::Eval(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Pair(pair) => {
            let first = parse_io_mode(pair.first)?;
            let second = parse_io_mode(pair.second)?;
            let pair = Pair::new(first, second);
            Some(PairMode::Pair(pair))
        }
        _ => None,
    }
}

fn parse_call_mode(io_mode: Val) -> Option<CallMode> {
    match io_mode {
        Val::Unit(_) => Some(CallMode::Eval(EvalMode::Value)),
        Val::Bool(b) => Some(CallMode::Eval(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Call(call) => {
            let func = parse_io_mode(call.func)?;
            let input = parse_io_mode(call.input)?;
            let call = Call::new(func, input);
            Some(CallMode::Call(call))
        }
        _ => None,
    }
}

fn parse_reverse_mode(io_mode: Val) -> Option<ReverseMode> {
    match io_mode {
        Val::Unit(_) => Some(ReverseMode::Eval(EvalMode::Value)),
        Val::Bool(b) => Some(ReverseMode::Eval(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Reverse(reverse) => {
            let func = parse_io_mode(reverse.func)?;
            let output = parse_io_mode(reverse.output)?;
            let reverse = Reverse::new(func, output);
            Some(ReverseMode::Reverse(reverse))
        }
        _ => None,
    }
}

fn parse_list_mode(io_mode: Val) -> Option<ListMode> {
    match io_mode {
        Val::Unit(_) => Some(ListMode::Eval(EvalMode::Value)),
        Val::Bool(b) => Some(ListMode::Eval(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::List(list) => Some(parse_list_mode_for_some(list)?),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            if *tag != *FOR_ALL {
                return None;
            }
            Some(ListMode::ForAll(parse_io_mode(call.input)?))
        }
        _ => None,
    }
}

fn parse_list_mode_for_some(io_mode: ListVal) -> Option<ListMode> {
    let list = io_mode
        .into_iter()
        .map(parse_list_item_mode)
        .try_collect()?;
    let list = ListMode::ForSome(list);
    Some(list)
}

fn parse_list_item_mode(io_mode: Val) -> Option<ListItemMode> {
    let Val::Call(call) = io_mode else {
        let io_mode = parse_io_mode(io_mode)?;
        return Some(ListItemMode {
            io_mode,
            ellipsis: false,
        });
    };
    let Val::Symbol(tag) = call.func else {
        return None;
    };
    if &*tag != ELLIPSIS {
        return None;
    }
    let io_mode = parse_io_mode(call.input)?;
    let io_mode = ListItemMode {
        io_mode,
        ellipsis: true,
    };
    Some(io_mode)
}

fn parse_map_mode(io_mode: Val) -> Option<MapMode> {
    match io_mode {
        Val::Unit(_) => Some(MapMode::Eval(EvalMode::Value)),
        Val::Bool(b) => Some(MapMode::Eval(if b.bool() {
            EvalMode::More
        } else {
            EvalMode::Less
        })),
        Val::Map(map) => Some(parse_map_mode_for_some(map)?),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            if *tag != *FOR_ALL {
                return None;
            }
            let Val::Pair(pair) = call.input else {
                return None;
            };
            let first = parse_io_mode(pair.first)?;
            let second = parse_io_mode(pair.second)?;
            Some(MapMode::ForAll(Pair::new(first, second)))
        }
        _ => None,
    }
}

fn parse_map_mode_for_some(io_mode: MapVal) -> Option<MapMode> {
    let map = io_mode
        .into_iter()
        .map(|(k, v)| {
            let io_mode = parse_io_mode(v)?;
            Some((k, io_mode))
        })
        .try_collect()?;
    let map = MapMode::ForSome(map);
    Some(map)
}

pub(crate) fn generate_eval_mode(eval_mode: EvalMode) -> Val {
    match eval_mode {
        EvalMode::Value => Val::Unit(Unit),
        EvalMode::More => Val::Bool(Bool::t()),
        EvalMode::Less => Val::Bool(Bool::f()),
    }
}

pub(crate) fn generate_io_mode(io_mode: &IoMode) -> Val {
    match io_mode {
        IoMode::Eval(mode) => generate_eval_mode(*mode),
        IoMode::Match(mode) => generate_match_mode(mode),
    }
}

pub(crate) fn generate_match_mode(mode: &MatchMode) -> Val {
    let mut map = Map::default();
    if mode.symbol != EvalMode::More {
        map.insert(symbol(SYMBOL), generate_eval_mode(mode.symbol));
    }
    if *mode.pair != PairMode::Eval(EvalMode::More) {
        let val = generate_pair_mode(&mode.pair);
        map.insert(symbol(PAIR), val);
    }
    if *mode.call != CallMode::Eval(EvalMode::More) {
        let val = generate_call_mode(&mode.call);
        map.insert(symbol(CALL), val);
    }
    if *mode.reverse != ReverseMode::Eval(EvalMode::More) {
        let val = generate_reverse_mode(&mode.reverse);
        map.insert(symbol(REVERSE), val);
    }
    if *mode.list != ListMode::Eval(EvalMode::More) {
        let val = generate_list_mode(&mode.list);
        map.insert(symbol(LIST), val);
    }
    if *mode.map != MapMode::Eval(EvalMode::More) {
        let val = generate_map_mode(&mode.map);
        map.insert(symbol(MAP), val);
    }
    Val::Map(map)
}

pub(crate) fn generate_pair_mode(pair: &PairMode) -> Val {
    match pair {
        PairMode::Eval(mode) => generate_eval_mode(*mode),
        PairMode::Pair(pair) => {
            let first = generate_io_mode(&pair.first);
            let second = generate_io_mode(&pair.second);
            Val::Pair(Box::new(Pair::new(first, second)))
        }
    }
}

pub(crate) fn generate_call_mode(call: &CallMode) -> Val {
    match call {
        CallMode::Eval(mode) => generate_eval_mode(*mode),
        CallMode::Call(call) => {
            let func = generate_io_mode(&call.func);
            let input = generate_io_mode(&call.input);
            Val::Call(Box::new(Call::new(func, input)))
        }
    }
}

pub(crate) fn generate_reverse_mode(reverse: &ReverseMode) -> Val {
    match reverse {
        ReverseMode::Eval(mode) => generate_eval_mode(*mode),
        ReverseMode::Reverse(reverse) => {
            let func = generate_io_mode(&reverse.func);
            let output = generate_io_mode(&reverse.output);
            Val::Reverse(Box::new(Reverse::new(func, output)))
        }
    }
}

pub(crate) fn generate_list_mode(list: &ListMode) -> Val {
    match list {
        ListMode::Eval(mode) => generate_eval_mode(*mode),
        ListMode::ForAll(mode) => {
            let mode = generate_io_mode(mode);
            Val::Call(Box::new(Call::new(symbol(FOR_ALL), mode)))
        }
        ListMode::ForSome(mode_list) => {
            let list = mode_list
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
    }
}

pub(crate) fn generate_map_mode(map: &MapMode) -> Val {
    match map {
        MapMode::Eval(mode) => generate_eval_mode(*mode),
        MapMode::ForAll(mode) => {
            let first = generate_io_mode(&mode.first);
            let second = generate_io_mode(&mode.second);
            let pair = Val::Pair(Box::new(Pair::new(first, second)));
            Val::Call(Box::new(Call::new(symbol(FOR_ALL), pair)))
        }
        MapMode::ForSome(mode_map) => {
            let map = mode_map
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
