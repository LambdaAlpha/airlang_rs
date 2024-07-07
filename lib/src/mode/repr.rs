use crate::{
    mode::{
        basic::{
            BasicMode,
            EVAL,
            FORM,
            ID,
        },
        list::{
            ListItemMode,
            ListMode,
        },
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
    Call,
    List,
    ListVal,
    Map,
    MapVal,
    Pair,
    Val,
};

const ELLIPSIS: &str = "..";
const DEFAULT: &str = "..";

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
        let default = Mode {
            default,
            specialized: None,
        };
        return Some(Pair::new(default.clone(), default));
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
        Val::Unit(_) => Some(ListMode::All(Mode {
            default,
            specialized: None,
        })),
        Val::List(list) => Some(parse_list_some(list)?),
        mode => Some(ListMode::All(parse(mode)?)),
    }
}

fn parse_list_some(mode: ListVal) -> Option<ListMode> {
    let mode = List::from(mode);
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
    let call = Call::from(call);
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

pub(crate) fn generate_list(mode: &ListMode, default: BasicMode) -> Option<Val> {
    match mode {
        ListMode::All(mode) => {
            let default = Mode {
                default,
                specialized: None,
            };
            if *mode == default {
                return None;
            }
            Some(generate(mode))
        }
        ListMode::Some(mode_list) => {
            let list: List<Val> = mode_list
                .iter()
                .map(|mode| {
                    if mode.ellipsis {
                        let tag = symbol(ELLIPSIS);
                        let mode = generate(&mode.mode);
                        Val::Call(Call::new(tag, mode).into())
                    } else {
                        generate(&mode.mode)
                    }
                })
                .collect();
            Some(Val::List(list.into()))
        }
    }
}

fn parse_map(mode: Val, default: BasicMode) -> Option<MapMode> {
    match mode {
        Val::Unit(_) => {
            let default = Mode {
                default,
                specialized: None,
            };
            Some(MapMode::All(Pair::new(default.clone(), default)))
        }
        Val::Map(map) => Some(parse_map_some(map)?),
        Val::Pair(pair) => {
            let pair = Pair::from(pair);
            let first = parse(pair.first)?;
            let second = parse(pair.second)?;
            Some(MapMode::All(Pair::new(first, second)))
        }
        _ => None,
    }
}

fn parse_map_some(mode: MapVal) -> Option<MapMode> {
    let mode = Map::from(mode);
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

pub(crate) fn generate_map(mode: &MapMode, default: BasicMode) -> Option<Val> {
    match mode {
        MapMode::All(mode) => {
            let default = Mode {
                default,
                specialized: None,
            };
            if mode.first == default && mode.second == default {
                return None;
            }
            let first = generate(&mode.first);
            let second = generate(&mode.second);
            let pair = Val::Pair(Pair::new(first, second).into());
            Some(pair)
        }
        MapMode::Some(mode_map) => {
            let map: Map<Val, Val> = mode_map
                .iter()
                .map(|(k, v)| {
                    let mode = generate(v);
                    (k.clone(), mode)
                })
                .collect();
            Some(Val::Map(map.into()))
        }
    }
}
