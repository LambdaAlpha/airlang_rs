use crate::{
    mode::{
        basic::{
            BasicMode,
            EVAL,
            FORM,
            ID,
        },
        list::ListMode,
        map::MapMode,
        Mode,
        ValMode,
    },
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        LIST,
        MAP,
        PAIR,
    },
    List,
    ListVal,
    Map,
    MapVal,
    Pair,
    Val,
};

const DEFAULT: &str = "default";

pub(crate) fn parse(mode: Val) -> Option<Mode> {
    let mode = match mode {
        Val::Unit(_) => Mode::default(),
        Val::Symbol(s) => Mode {
            default: parse_basic_mode(&s)?,
            specialized: None,
        },
        Val::Map(mut map) => {
            let default = parse_default(&mut map)?;
            let specialized = Some(Box::new(parse_specialized(&mut map, default)?));
            Mode {
                default,
                specialized,
            }
        }
        _ => return None,
    };
    Some(mode)
}

pub(crate) fn generate(mode: &Mode) -> Val {
    let Some(specialized) = &mode.specialized else {
        return generate_basic_mode(mode.default);
    };
    let mut map = Map::default();
    generate_default(mode.default, &mut map);
    generate_specialized(specialized, mode.default, &mut map);
    Val::Map(map.into())
}

fn parse_basic_mode(s: &str) -> Option<BasicMode> {
    let basic_mode = match s {
        ID => BasicMode::Id,
        FORM => BasicMode::Form,
        EVAL => BasicMode::Eval,
        _ => return None,
    };
    Some(basic_mode)
}

pub(crate) fn generate_basic_mode(basic_mode: BasicMode) -> Val {
    let s = match basic_mode {
        BasicMode::Id => ID,
        BasicMode::Form => FORM,
        BasicMode::Eval => EVAL,
    };
    symbol(s)
}

fn parse_default(map: &mut MapVal) -> Option<BasicMode> {
    let transform = match map_remove(map, DEFAULT) {
        Val::Unit(_) => BasicMode::default(),
        Val::Symbol(s) => parse_basic_mode(&s)?,
        _ => return None,
    };
    Some(transform)
}

fn generate_default(default: BasicMode, map: &mut Map<Val, Val>) {
    if default != BasicMode::default() {
        let default = generate_basic_mode(default);
        map.insert(symbol(DEFAULT), default);
    }
}

fn parse_specialized(map: &mut MapVal, default: BasicMode) -> Option<ValMode> {
    let pair = parse_pair(map_remove(map, PAIR), default)?;
    let list = parse_list(map_remove(map, LIST), default)?;
    let map = parse_map(map_remove(map, MAP), default)?;
    Some(ValMode { pair, list, map })
}

pub(crate) fn generate_specialized(mode: &ValMode, default: BasicMode, map: &mut Map<Val, Val>) {
    if let Some(val) = generate_pair(&mode.pair, default) {
        map.insert(symbol(PAIR), val);
    }
    if let Some(val) = generate_list(&mode.list, default) {
        map.insert(symbol(LIST), val);
    }
    if let Some(val) = generate_map(&mode.map, default) {
        map.insert(symbol(MAP), val);
    }
}

fn parse_pair(mode: Val, default: BasicMode) -> Option<Pair<Mode, Mode>> {
    if mode.is_unit() {
        return Some(default_pair(default));
    }
    let Val::Pair(pair) = mode else {
        return None;
    };
    let pair = Pair::from(pair);
    let first = parse(pair.first)?;
    let second = parse(pair.second)?;
    Some(Pair::new(first, second))
}

fn generate_pair(mode: &Pair<Mode, Mode>, default: BasicMode) -> Option<Val> {
    let default = Mode {
        default,
        specialized: None,
    };
    if mode.first == default && mode.second == default {
        return None;
    }
    let first = generate(&mode.first);
    let second = generate(&mode.second);
    Some(Val::Pair(Pair::new(first, second).into()))
}

fn parse_list(mode: Val, default: BasicMode) -> Option<ListMode> {
    match mode {
        Val::Unit(_) => Some(ListMode {
            head: List::default(),
            tail: Mode {
                default,
                specialized: None,
            },
        }),
        Val::List(head) => {
            let head = parse_list_head(head)?;
            let tail = Mode {
                default,
                specialized: None,
            };
            Some(ListMode { head, tail })
        }
        Val::Pair(head_tail) => {
            let head_tail = Pair::from(head_tail);
            let Val::List(head) = head_tail.first else {
                return None;
            };
            let head = parse_list_head(head)?;
            let tail = parse(head_tail.second)?;
            Some(ListMode { head, tail })
        }
        _ => None,
    }
}

fn parse_list_head(head: ListVal) -> Option<List<Mode>> {
    List::from(head).into_iter().map(parse).collect()
}

pub(crate) fn generate_list(mode: &ListMode, default: BasicMode) -> Option<Val> {
    let default = Mode {
        default,
        specialized: None,
    };
    if mode.head.is_empty() && mode.tail == default {
        return None;
    }
    let head: List<Val> = mode.head.iter().map(generate).collect();
    let head = Val::List(head.into());
    if mode.tail == default {
        return Some(head);
    }
    let tail = generate(&mode.tail);
    let pair = Pair::new(head, tail);
    Some(Val::Pair(pair.into()))
}

fn parse_map(mode: Val, default: BasicMode) -> Option<MapMode> {
    match mode {
        Val::Unit(_) => {
            let default = Mode {
                default,
                specialized: None,
            };
            Some(MapMode {
                some: Map::default(),
                else1: Pair::new(default.clone(), default),
            })
        }
        Val::Map(some) => {
            let some = parse_map_some(some)?;
            let else1 = default_pair(default);
            Some(MapMode { some, else1 })
        }
        Val::Pair(some_else) => {
            let some_else = Pair::from(some_else);
            let Val::Map(some) = some_else.first else {
                return None;
            };
            let Val::Pair(else1) = some_else.second else {
                return None;
            };
            let some = parse_map_some(some)?;
            let else1 = Pair::from(else1);
            let key = parse(else1.first)?;
            let value = parse(else1.second)?;
            let else1 = Pair::new(key, value);
            Some(MapMode { some, else1 })
        }
        _ => None,
    }
}

fn parse_map_some(some: MapVal) -> Option<Map<Val, Mode>> {
    Map::from(some)
        .into_iter()
        .map(|(k, v)| {
            let mode = parse(v)?;
            Some((k, mode))
        })
        .collect()
}

pub(crate) fn generate_map(mode: &MapMode, default: BasicMode) -> Option<Val> {
    let default = Mode {
        default,
        specialized: None,
    };
    if mode.some.is_empty() && mode.else1.first == default && mode.else1.second == default {
        return None;
    }
    let some: Map<Val, Val> = mode
        .some
        .iter()
        .map(|(k, v)| {
            let mode = generate(v);
            (k.clone(), mode)
        })
        .collect();
    let some = Val::Map(some.into());
    if mode.else1.first == default && mode.else1.second == default {
        return Some(some);
    }
    let first = generate(&mode.else1.first);
    let second = generate(&mode.else1.second);
    let else1 = Val::Pair(Pair::new(first, second).into());
    let some_else = Pair::new(some, else1);
    Some(Val::Pair(some_else.into()))
}

fn default_pair(default: BasicMode) -> Pair<Mode, Mode> {
    let default = Mode {
        default,
        specialized: None,
    };
    Pair::new(default.clone(), default)
}
