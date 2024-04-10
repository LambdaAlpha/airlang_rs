use crate::{
    mode::{
        CallDepMode,
        Mode,
        ReverseDepMode,
    },
    transform::{
        EVAL,
        ID,
        LAZY,
    },
    utils::val::symbol,
    val::{
        BOOL,
        BYTES,
        CALL,
        FLOAT,
        INT,
        LIST,
        MAP,
        PAIR,
        REVERSE,
        STRING,
        SYMBOL,
        UNIT,
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
    SymbolMode,
    Transform,
    Val,
    ValMode,
};

const ELLIPSIS: &str = "...";
const FOR_ALL: &str = "all";

pub(crate) fn parse(mode: Val) -> Option<Mode> {
    let mode = match mode {
        Val::Unit(_) => Mode::Predefined(Default::default()),
        Val::Symbol(s) => Mode::Predefined(parse_transform(&s)?),
        Val::Map(map) => Mode::Custom(Box::new(parse_val(map)?)),
        _ => return None,
    };
    Some(mode)
}

pub(crate) fn generate(mode: &Mode) -> Val {
    match mode {
        Mode::Predefined(t) => generate_transform(*t),
        Mode::Custom(m) => generate_val(m),
    }
}

fn parse_transform(s: &str) -> Option<Transform> {
    match s {
        ID => Some(Transform::Id),
        LAZY => Some(Transform::Lazy),
        EVAL => Some(Transform::Eval),
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
        mode.symbol = parse_symbol(symbol_mode)?;
    }
    if let Some(pair_mode) = map.remove(&symbol(PAIR)) {
        mode.pair = Box::new(parse_pair(pair_mode)?);
    }
    if let Some(call_mode) = map.remove(&symbol(CALL)) {
        mode.call = Box::new(parse_call(call_mode)?);
    }
    if let Some(reverse_mode) = map.remove(&symbol(REVERSE)) {
        mode.reverse = Box::new(parse_reverse(reverse_mode)?);
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
        map.insert(symbol(SYMBOL), generate_symbol(&mode.symbol));
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

fn parse_symbol(mode: Val) -> Option<SymbolMode> {
    match mode {
        Val::Unit(_) => Some(SymbolMode::Eval),
        Val::Symbol(s) => match &*s {
            ID => Some(SymbolMode::Id),
            EVAL => Some(SymbolMode::Eval),
            _ => None,
        },
        _ => None,
    }
}

fn generate_symbol(mode: &SymbolMode) -> Val {
    let s = match mode {
        SymbolMode::Id => ID,
        SymbolMode::Eval => EVAL,
    };
    symbol(s)
}

fn parse_pair(mode: Val) -> Option<Pair<Mode, Mode>> {
    let Val::Pair(pair) = mode else {
        return None;
    };
    let first = parse(pair.first)?;
    let second = parse(pair.second)?;
    Some(Pair::new(first, second))
}

fn generate_pair(mode: &Pair<Mode, Mode>) -> Val {
    let first = generate(&mode.first);
    let second = generate(&mode.second);
    Val::Pair(Box::new(Pair::new(first, second)))
}

fn parse_call(mode: Val) -> Option<CallMode> {
    let mode = match mode {
        Val::Unit(_) => CallMode::Eval,
        Val::Symbol(s) => {
            if &*s != EVAL {
                return None;
            }
            CallMode::Eval
        }
        Val::Call(call) => {
            let func = parse(call.func)?;
            let input = parse(call.input)?;
            CallMode::Struct(Call::new(func, input))
        }
        Val::Map(map) => {
            let call_dep_mode = parse_call_dep(map)?;
            CallMode::Dependent(call_dep_mode)
        }
        _ => return None,
    };
    Some(mode)
}

fn parse_call_dep(mut map: MapVal) -> Option<CallDepMode> {
    let mut mode = CallDepMode::default();
    if let Some(unit_mode) = map.remove(&symbol(UNIT)) {
        mode.unit = parse(unit_mode)?;
    }
    if let Some(bool_mode) = map.remove(&symbol(BOOL)) {
        mode.bool = parse(bool_mode)?;
    }
    if let Some(int_mode) = map.remove(&symbol(INT)) {
        mode.int = parse(int_mode)?;
    }
    if let Some(float_mode) = map.remove(&symbol(FLOAT)) {
        mode.float = parse(float_mode)?;
    }
    if let Some(bytes_mode) = map.remove(&symbol(BYTES)) {
        mode.bytes = parse(bytes_mode)?;
    }
    if let Some(string_mode) = map.remove(&symbol(STRING)) {
        mode.string = parse(string_mode)?;
    }
    if let Some(symbol_mode) = map.remove(&symbol(SYMBOL)) {
        mode.symbol = parse(symbol_mode)?;
    }
    Some(mode)
}

pub(crate) fn generate_call(mode: &CallMode) -> Val {
    match mode {
        CallMode::Eval => symbol(EVAL),
        CallMode::Struct(call) => {
            let func = generate(&call.func);
            let input = generate(&call.input);
            Val::Call(Box::new(Call::new(func, input)))
        }
        CallMode::Dependent(call) => generate_call_dep(call),
    }
}

fn generate_call_dep(mode: &CallDepMode) -> Val {
    let mut map = Map::default();
    if mode.unit != Default::default() {
        map.insert(symbol(UNIT), generate(&mode.unit));
    }
    if mode.bool != Default::default() {
        map.insert(symbol(BOOL), generate(&mode.bool));
    }
    if mode.int != Default::default() {
        map.insert(symbol(INT), generate(&mode.int));
    }
    if mode.float != Default::default() {
        map.insert(symbol(FLOAT), generate(&mode.float));
    }
    if mode.bytes != Default::default() {
        map.insert(symbol(BYTES), generate(&mode.bytes));
    }
    if mode.string != Default::default() {
        map.insert(symbol(STRING), generate(&mode.string));
    }
    if mode.symbol != Default::default() {
        map.insert(symbol(SYMBOL), generate(&mode.symbol));
    }
    Val::Map(map)
}

fn parse_reverse(mode: Val) -> Option<ReverseMode> {
    let mode = match mode {
        Val::Unit(_) => ReverseMode::Eval,
        Val::Symbol(s) => {
            if &*s != EVAL {
                return None;
            }
            ReverseMode::Eval
        }
        Val::Reverse(reverse) => {
            let func = parse(reverse.func)?;
            let output = parse(reverse.output)?;
            ReverseMode::Struct(Reverse::new(func, output))
        }
        Val::Map(map) => {
            let reverse_dep_mode = parse_reverse_dep(map)?;
            ReverseMode::Dependent(reverse_dep_mode)
        }
        _ => return None,
    };
    Some(mode)
}

fn parse_reverse_dep(mut map: MapVal) -> Option<ReverseDepMode> {
    let mut mode = ReverseDepMode::default();
    if let Some(unit_mode) = map.remove(&symbol(UNIT)) {
        mode.unit = parse(unit_mode)?;
    }
    if let Some(bool_mode) = map.remove(&symbol(BOOL)) {
        mode.bool = parse(bool_mode)?;
    }
    if let Some(int_mode) = map.remove(&symbol(INT)) {
        mode.int = parse(int_mode)?;
    }
    if let Some(float_mode) = map.remove(&symbol(FLOAT)) {
        mode.float = parse(float_mode)?;
    }
    if let Some(bytes_mode) = map.remove(&symbol(BYTES)) {
        mode.bytes = parse(bytes_mode)?;
    }
    if let Some(string_mode) = map.remove(&symbol(STRING)) {
        mode.string = parse(string_mode)?;
    }
    if let Some(symbol_mode) = map.remove(&symbol(SYMBOL)) {
        mode.symbol = parse(symbol_mode)?;
    }
    Some(mode)
}

pub(crate) fn generate_reverse(mode: &ReverseMode) -> Val {
    match mode {
        ReverseMode::Eval => symbol(EVAL),
        ReverseMode::Struct(reverse) => {
            let func = generate(&reverse.func);
            let output = generate(&reverse.output);
            Val::Reverse(Box::new(Reverse::new(func, output)))
        }
        ReverseMode::Dependent(reverse) => generate_reverse_dep(reverse),
    }
}

fn generate_reverse_dep(mode: &ReverseDepMode) -> Val {
    let mut map = Map::default();
    if mode.unit != Default::default() {
        map.insert(symbol(UNIT), generate(&mode.unit));
    }
    if mode.bool != Default::default() {
        map.insert(symbol(BOOL), generate(&mode.bool));
    }
    if mode.int != Default::default() {
        map.insert(symbol(INT), generate(&mode.int));
    }
    if mode.float != Default::default() {
        map.insert(symbol(FLOAT), generate(&mode.float));
    }
    if mode.bytes != Default::default() {
        map.insert(symbol(BYTES), generate(&mode.bytes));
    }
    if mode.string != Default::default() {
        map.insert(symbol(STRING), generate(&mode.string));
    }
    if mode.symbol != Default::default() {
        map.insert(symbol(SYMBOL), generate(&mode.symbol));
    }
    Val::Map(map)
}

fn parse_list(mode: Val) -> Option<ListMode> {
    match mode {
        Val::List(list) => Some(parse_list_some(list)?),
        Val::Call(call) => {
            let Val::Symbol(tag) = call.func else {
                return None;
            };
            if *tag != *FOR_ALL {
                return None;
            }
            Some(ListMode::All(parse(call.input)?))
        }
        _ => None,
    }
}

fn parse_list_some(mode: ListVal) -> Option<ListMode> {
    let list = mode
        .into_iter()
        .map(parse_list_item)
        .collect::<Option<List<_>>>()?;
    let list = ListMode::Some(list);
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
        ListMode::All(mode) => {
            let mode = generate(mode);
            Val::Call(Box::new(Call::new(symbol(FOR_ALL), mode)))
        }
        ListMode::Some(mode_list) => {
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
        Val::Map(map) => Some(parse_map_some(map)?),
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
            Some(MapMode::All(Pair::new(first, second)))
        }
        _ => None,
    }
}

fn parse_map_some(mode: MapVal) -> Option<MapMode> {
    let map = mode
        .into_iter()
        .map(|(k, v)| {
            let mode = parse(v)?;
            Some((k, mode))
        })
        .collect::<Option<Map<_, _>>>()?;
    let map = MapMode::Some(map);
    Some(map)
}

pub(crate) fn generate_map(mode: &MapMode) -> Val {
    match mode {
        MapMode::All(mode) => {
            let first = generate(&mode.first);
            let second = generate(&mode.second);
            let pair = Val::Pair(Box::new(Pair::new(first, second)));
            Val::Call(Box::new(Call::new(symbol(FOR_ALL), pair)))
        }
        MapMode::Some(mode_map) => {
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
