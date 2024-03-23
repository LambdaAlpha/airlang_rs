use crate::{
    mode::Mode,
    transformer::{
        EVAL,
        ID,
        LAZY,
    },
    utils::val::symbol,
    val::{
        CALL,
        LIST,
        MAP,
        PAIR,
        REVERSE,
        SYMBOL,
    },
    Call,
    CallMode,
    List,
    ListItemMode,
    ListMode,
    ListVal,
    Map,
    MapMode,
    MapVal,
    Pair,
    Reverse,
    ReverseMode,
    Transform,
    TransformMode,
    Val,
    ValMode,
};

const ELLIPSIS: &str = "...";
const FOR_ALL: &str = "all";

pub(crate) fn parse(mode: Val) -> Option<TransformMode> {
    parse_mode(mode, |mode| {
        let Val::Map(map) = mode else {
            return None;
        };
        parse_val(map)
    })
}

pub(crate) fn generate(mode: &TransformMode) -> Val {
    generate_mode(mode, generate_val)
}

fn transform_from_symbol(s: &str) -> Option<Transform> {
    match s {
        ID => Some(Transform::Id),
        LAZY => Some(Transform::Lazy),
        EVAL => Some(Transform::Eval),
        _ => None,
    }
}

fn parse_transform(val: Val) -> Option<Transform> {
    match val {
        Val::Unit(_) => Some(Transform::Id),
        Val::Symbol(s) => transform_from_symbol(&s),
        _ => None,
    }
}

pub(crate) fn generate_transform(transform: Transform) -> Val {
    let s = match transform {
        Transform::Id => ID,
        Transform::Eval => EVAL,
        Transform::Lazy => LAZY,
    };
    symbol(s)
}

fn parse_val(mut map: MapVal) -> Option<ValMode> {
    let mut mode = ValMode::default();
    if let Some(symbol_mode) = map.remove(&symbol(SYMBOL)) {
        mode.symbol = parse_transform(symbol_mode)?;
    }
    if let Some(pair_mode) = map.remove(&symbol(PAIR)) {
        mode.pair = Box::new(parse_pair(pair_mode)?);
    }
    if let Some(call_mode) = map.remove(&symbol(CALL)) {
        mode.call = parse_call(call_mode)?;
    }
    if let Some(reverse_mode) = map.remove(&symbol(REVERSE)) {
        mode.reverse = parse_reverse(reverse_mode)?;
    }
    if let Some(list_mode) = map.remove(&symbol(LIST)) {
        mode.list = Box::new(parse_list(list_mode)?);
    }
    if let Some(map_mode) = map.remove(&symbol(MAP)) {
        mode.map = Box::new(parse_map(map_mode)?);
    }
    Some(mode)
}

pub(crate) fn generate_val(mode: &ValMode) -> Val {
    let mut map = Map::default();
    if mode.symbol != Default::default() {
        map.insert(symbol(SYMBOL), generate_transform(mode.symbol));
    }
    if mode.pair != Default::default() {
        let val = generate_pair(&mode.pair);
        map.insert(symbol(PAIR), val);
    }
    if mode.call != Default::default() {
        let val = generate_call(&mode.call);
        map.insert(symbol(CALL), val);
    }
    if mode.reverse != Default::default() {
        let val = generate_reverse(&mode.reverse);
        map.insert(symbol(REVERSE), val);
    }
    if mode.list != Default::default() {
        let val = generate_list(&mode.list);
        map.insert(symbol(LIST), val);
    }
    if mode.map != Default::default() {
        let val = generate_map(&mode.map);
        map.insert(symbol(MAP), val);
    }
    Val::Map(map)
}

fn parse_pair(mode: Val) -> Option<Pair<TransformMode, TransformMode>> {
    let Val::Pair(pair) = mode else {
        return None;
    };
    let first = parse(pair.first)?;
    let second = parse(pair.second)?;
    Some(Pair::new(first, second))
}

pub(crate) fn generate_pair(mode: &Pair<TransformMode, TransformMode>) -> Val {
    let first = generate(&mode.first);
    let second = generate(&mode.second);
    Val::Pair(Box::new(Pair::new(first, second)))
}

fn parse_call(mode: Val) -> Option<Mode<Transform, Box<CallMode>>> {
    parse_mode(mode, |mode| {
        let Val::Call(call) = mode else {
            return None;
        };
        let func = parse(call.func)?;
        let input = parse(call.input)?;
        Some(Box::new(CallMode::Call(Call::new(func, input))))
    })
}

pub(crate) fn generate_call(mode: &Mode<Transform, Box<CallMode>>) -> Val {
    generate_mode(mode, |call| match &**call {
        CallMode::Call(call) => {
            let func = generate(&call.func);
            let input = generate(&call.input);
            Val::Call(Box::new(Call::new(func, input)))
        }
    })
}

fn parse_reverse(mode: Val) -> Option<Mode<Transform, Box<ReverseMode>>> {
    parse_mode(mode, |mode| {
        let Val::Reverse(reverse) = mode else {
            return None;
        };
        let func = parse(reverse.func)?;
        let output = parse(reverse.output)?;
        Some(Box::new(ReverseMode::Reverse(Reverse::new(func, output))))
    })
}

pub(crate) fn generate_reverse(mode: &Mode<Transform, Box<ReverseMode>>) -> Val {
    generate_mode(mode, |reverse| match &**reverse {
        ReverseMode::Reverse(reverse) => {
            let func = generate(&reverse.func);
            let output = generate(&reverse.output);
            Val::Reverse(Box::new(Reverse::new(func, output)))
        }
    })
}

fn parse_list(mode: Val) -> Option<ListMode> {
    match mode {
        Val::List(list) => Some(parse_list_for_some(list)?),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            if *tag != *FOR_ALL {
                return None;
            }
            Some(ListMode::ForAll(parse(call.input)?))
        }
        _ => None,
    }
}

fn parse_list_for_some(mode: ListVal) -> Option<ListMode> {
    let list = mode
        .into_iter()
        .map(parse_list_item)
        .collect::<Option<List<_>>>()?;
    let list = ListMode::ForSome(list);
    Some(list)
}

fn parse_list_item(mode: Val) -> Option<ListItemMode> {
    let Val::Call(call) = mode else {
        let mode = parse(mode)?;
        return Some(ListItemMode {
            mode,
            ellipsis: false,
        });
    };
    let Val::Symbol(tag) = call.func else {
        return None;
    };
    if &*tag != ELLIPSIS {
        return None;
    }
    let mode = parse(call.input)?;
    let mode = ListItemMode {
        mode,
        ellipsis: true,
    };
    Some(mode)
}

pub(crate) fn generate_list(mode: &ListMode) -> Val {
    match mode {
        ListMode::ForAll(mode) => {
            let mode = generate(mode);
            Val::Call(Box::new(Call::new(symbol(FOR_ALL), mode)))
        }
        ListMode::ForSome(mode_list) => {
            let list = mode_list
                .iter()
                .map(|mode| {
                    if mode.ellipsis {
                        let tag = symbol(ELLIPSIS);
                        let mode = generate(&mode.mode);
                        Val::Call(Box::new(Call::new(tag, mode)))
                    } else {
                        generate(&mode.mode)
                    }
                })
                .collect();
            Val::List(list)
        }
    }
}

fn parse_map(mode: Val) -> Option<MapMode> {
    match mode {
        Val::Map(map) => Some(parse_map_for_some(map)?),
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
            let first = parse(pair.first)?;
            let second = parse(pair.second)?;
            Some(MapMode::ForAll(Pair::new(first, second)))
        }
        _ => None,
    }
}

fn parse_map_for_some(mode: MapVal) -> Option<MapMode> {
    let map = mode
        .into_iter()
        .map(|(k, v)| {
            let mode = parse(v)?;
            Some((k, mode))
        })
        .collect::<Option<Map<_, _>>>()?;
    let map = MapMode::ForSome(map);
    Some(map)
}

pub(crate) fn generate_map(mode: &MapMode) -> Val {
    match mode {
        MapMode::ForAll(mode) => {
            let first = generate(&mode.first);
            let second = generate(&mode.second);
            let pair = Val::Pair(Box::new(Pair::new(first, second)));
            Val::Call(Box::new(Call::new(symbol(FOR_ALL), pair)))
        }
        MapMode::ForSome(mode_map) => {
            let map = mode_map
                .iter()
                .map(|(k, v)| {
                    let mode = generate(v);
                    (k.clone(), mode)
                })
                .collect();
            Val::Map(map)
        }
    }
}

fn parse_mode<T>(mode: Val, f: impl FnOnce(Val) -> Option<T>) -> Option<Mode<Transform, T>> {
    let transform_mode = match mode {
        Val::Unit(_) => Mode::Generic(Transform::Id),
        Val::Symbol(s) => Mode::Generic(transform_from_symbol(&s)?),
        mode => Mode::Specific(f(mode)?),
    };
    Some(transform_mode)
}

fn generate_mode<T>(mode: &Mode<Transform, T>, f: impl FnOnce(&T) -> Val) -> Val {
    match mode {
        Mode::Generic(mode) => generate_transform(*mode),
        Mode::Specific(mode) => f(mode),
    }
}
