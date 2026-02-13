use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputEvalFunc;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::func::CtxMutInputEvalFunc;
use crate::semantics::func::CtxMutInputFreeFunc;
use crate::semantics::val::MAP;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Cell;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct MapLib {
    pub make: PrimFuncVal,
    pub make_set: PrimFuncVal,
    pub get_length: PrimFuncVal,
    pub get_items: PrimFuncVal,
    pub into_items: PrimFuncVal,
    pub get_keys: PrimFuncVal,
    pub into_keys: PrimFuncVal,
    pub get_values: PrimFuncVal,
    pub into_values: PrimFuncVal,
    pub contain: PrimFuncVal,
    pub contain_all: PrimFuncVal,
    pub contain_any: PrimFuncVal,
    pub set: PrimFuncVal,
    pub set_many: PrimFuncVal,
    pub get: PrimFuncVal,
    pub get_many: PrimFuncVal,
    pub remove: PrimFuncVal,
    pub remove_many: PrimFuncVal,
    pub move_: PrimFuncVal,
    pub clear: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_ID, MAP, ".make");
pub const MAKE_SET: &str = concatcp!(PREFIX_ID, MAP, ".make_set");
pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, MAP, ".get_length");
pub const GET_ITEMS: &str = concatcp!(PREFIX_ID, MAP, ".get_items");
pub const INTO_ITEMS: &str = concatcp!(PREFIX_ID, MAP, ".into_items");
pub const GET_KEYS: &str = concatcp!(PREFIX_ID, MAP, ".get_keys");
pub const INTO_KEYS: &str = concatcp!(PREFIX_ID, MAP, ".into_keys");
pub const GET_VALUES: &str = concatcp!(PREFIX_ID, MAP, ".get_values");
pub const INTO_VALUES: &str = concatcp!(PREFIX_ID, MAP, ".into_values");
pub const CONTAIN: &str = concatcp!(PREFIX_ID, MAP, ".contain");
pub const CONTAIN_ALL: &str = concatcp!(PREFIX_ID, MAP, ".contain_all");
pub const CONTAIN_ANY: &str = concatcp!(PREFIX_ID, MAP, ".contain_any");
pub const SET: &str = concatcp!(PREFIX_ID, MAP, ".set");
pub const SET_MANY: &str = concatcp!(PREFIX_ID, MAP, ".set_many");
pub const GET: &str = concatcp!(PREFIX_ID, MAP, ".get");
pub const GET_MANY: &str = concatcp!(PREFIX_ID, MAP, ".get_many");
pub const REMOVE: &str = concatcp!(PREFIX_ID, MAP, ".remove");
pub const REMOVE_MANY: &str = concatcp!(PREFIX_ID, MAP, ".remove_many");
pub const MOVE: &str = concatcp!(PREFIX_ID, MAP, ".move");
pub const CLEAR: &str = concatcp!(PREFIX_ID, MAP, ".clear");

impl Default for MapLib {
    fn default() -> Self {
        MapLib {
            make: CtxFreeInputEvalFunc { fn_: make }.build(),
            make_set: CtxFreeInputEvalFunc { fn_: make_set }.build(),
            get_length: CtxConstInputFreeFunc { fn_: get_length }.build(),
            get_items: CtxConstInputFreeFunc { fn_: get_items }.build(),
            into_items: CtxMutInputFreeFunc { fn_: into_items }.build(),
            get_keys: CtxConstInputFreeFunc { fn_: get_keys }.build(),
            into_keys: CtxMutInputFreeFunc { fn_: into_keys }.build(),
            get_values: CtxConstInputFreeFunc { fn_: get_values }.build(),
            into_values: CtxMutInputFreeFunc { fn_: into_values }.build(),
            contain: CtxConstInputEvalFunc { fn_: contain }.build(),
            contain_all: CtxConstInputEvalFunc { fn_: contain_all }.build(),
            contain_any: CtxConstInputEvalFunc { fn_: contain_any }.build(),
            set: CtxMutInputEvalFunc { fn_: set }.build(),
            set_many: CtxMutInputEvalFunc { fn_: set_many }.build(),
            get: CtxConstInputEvalFunc { fn_: get }.build(),
            get_many: CtxConstInputEvalFunc { fn_: get_many }.build(),
            remove: CtxMutInputEvalFunc { fn_: remove }.build(),
            remove_many: CtxMutInputEvalFunc { fn_: remove_many }.build(),
            move_: CtxMutInputEvalFunc { fn_: move_ }.build(),
            clear: CtxMutInputFreeFunc { fn_: clear }.build(),
        }
    }
}

impl CfgMod for MapLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, MAKE_SET, self.make_set);
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, GET_ITEMS, self.get_items);
        extend_func(cfg, INTO_ITEMS, self.into_items);
        extend_func(cfg, GET_KEYS, self.get_keys);
        extend_func(cfg, INTO_KEYS, self.into_keys);
        extend_func(cfg, GET_VALUES, self.get_values);
        extend_func(cfg, INTO_VALUES, self.into_values);
        extend_func(cfg, CONTAIN, self.contain);
        extend_func(cfg, CONTAIN_ALL, self.contain_all);
        extend_func(cfg, CONTAIN_ANY, self.contain_any);
        extend_func(cfg, SET, self.set);
        extend_func(cfg, SET_MANY, self.set_many);
        extend_func(cfg, GET, self.get);
        extend_func(cfg, GET_MANY, self.get_many);
        extend_func(cfg, REMOVE, self.remove);
        extend_func(cfg, REMOVE_MANY, self.remove_many);
        extend_func(cfg, MOVE, self.move_);
        extend_func(cfg, CLEAR, self.clear);
    }
}

pub fn make(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        return bug!(cfg, "{MAKE}: expected input to be a list, but got {input}");
    };
    let list = List::from(list);
    let mut map: Map<Key, Val> = Map::with_capacity(list.len());
    for item in list {
        let Val::Pair(pair) = item else {
            return bug!(cfg, "{MAKE}: expected input.item to be a pair, but got {item}");
        };
        let pair = Pair::from(pair);
        let Val::Key(key) = pair.left else {
            return bug!(cfg, "{MAKE}: expected input.item.left to be a key, \
                but got {}", pair.left);
        };
        map.insert(key, pair.right);
    }
    Val::Map(map.into())
}

pub fn make_set(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        return bug!(cfg, "{MAKE_SET}: expected input to be a list, but got {input}");
    };
    let list = List::from(list);
    let mut map: Map<Key, Val> = Map::with_capacity(list.len());
    for item in list {
        let Val::Key(key) = item else {
            return bug!(cfg, "{MAKE_SET}: expected input.item to be a key, but got {item}");
        };
        map.insert(key, Val::default());
    }
    Val::Map(map.into())
}

pub fn get_length(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a map, but got {ctx}");
    };
    let len: Int = map.len().into();
    Val::Int(len.into())
}

pub fn get_items(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{GET_ITEMS}: expected context to be a map, but got {ctx}");
    };
    let items: List<Val> = map
        .iter()
        .map(|(k, v)| Val::Pair(Pair::new(Val::Key(k.clone()), v.clone()).into()))
        .collect();
    Val::List(items.into())
}

pub fn into_items(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{INTO_ITEMS}: expected context to be a map, but got {ctx}");
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let items: List<Val> =
        origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(Val::Key(k), v).into())).collect();
    Val::List(items.into())
}

pub fn get_keys(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{GET_KEYS}: expected context to be a map, but got {ctx}");
    };
    let keys: List<Val> = map.keys().map(|k| Val::Key(k.clone())).collect();
    Val::List(keys.into())
}

pub fn into_keys(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{INTO_KEYS}: expected context to be a map, but got {ctx}");
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let keys: List<Val> = origin.into_keys().map(Val::Key).collect();
    Val::List(keys.into())
}

pub fn get_values(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{GET_VALUES}: expected context to be a map, but got {ctx}");
    };
    let values: List<Val> = map.values().cloned().collect();
    Val::List(values.into())
}

pub fn into_values(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{INTO_VALUES}: expected context to be a map, but got {ctx}");
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let values: List<Val> = origin.into_values().collect();
    Val::List(values.into())
}

pub fn contain(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{CONTAIN}: expected context to be a map, but got {ctx}");
    };
    let Val::Key(key) = input else {
        return bug!(cfg, "{CONTAIN}: expected input to be a key, but got {input}");
    };
    Val::Bit(Bit::from(map.contains_key(&key)))
}

pub fn contain_all(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{CONTAIN_ALL}: expected context to be a map, but got {ctx}");
    };
    let Val::List(keys) = input else {
        return bug!(cfg, "{CONTAIN_ALL}: expected input to be a list, but got {input}");
    };
    let keys = List::from(keys);
    for key in keys {
        let Val::Key(key) = key else {
            return bug!(cfg, "{CONTAIN_ALL}: expected input.item to be a key, but got {key}");
        };
        if !map.contains_key(&key) {
            return Val::Bit(Bit::from(false));
        }
    }
    Val::Bit(Bit::from(true))
}

pub fn contain_any(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{CONTAIN_ANY}: expected context to be a map, but got {ctx}");
    };
    let Val::List(keys) = input else {
        return bug!(cfg, "{CONTAIN_ANY}: expected input to be a list, but got {input}");
    };
    let keys = List::from(keys);
    for key in keys {
        let Val::Key(key) = key else {
            return bug!(cfg, "{CONTAIN_ANY}: expected input.item to be a key, but got {key}");
        };
        if map.contains_key(&key) {
            return Val::Bit(Bit::from(true));
        }
    }
    Val::Bit(Bit::from(false))
}

pub fn set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{SET}: expected context to be a map, but got {ctx}");
    };
    let Val::Pair(key_value) = input else {
        return bug!(cfg, "{SET}: expected input to be a pair, but got {input}");
    };
    let key_value = Pair::from(key_value);
    let Val::Key(key) = key_value.left else {
        return bug!(cfg, "{SET}: expected input.left to be a key, but got {}", key_value.left);
    };
    let value = key_value.right;
    let Some(value) = map.insert(key, value) else {
        return Val::default();
    };
    Val::Cell(Cell::new(value).into())
}

pub fn set_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{SET_MANY}: expected context to be a map, but got {ctx}");
    };
    let Val::Map(update) = input else {
        return bug!(cfg, "{SET_MANY}: expected input to be a map, but got {input}");
    };
    let update = Map::from(update);
    let map: Map<Key, Val> =
        update.into_iter().filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

pub fn get(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{GET}: expected context to be a map, but got {ctx}");
    };
    let Val::Key(key) = input else {
        return bug!(cfg, "{GET}: expected input to be a key, but got {input}");
    };
    let Some(value) = map.get(&key) else {
        return Val::default();
    };
    Val::Cell(Cell::new(value.clone()).into())
}

pub fn get_many(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{GET_MANY}: expected context to be a map, but got {ctx}");
    };
    let Val::List(keys) = input else {
        return bug!(cfg, "{GET_MANY}: expected input to be a list, but got {input}");
    };
    let keys = List::from(keys);
    let mut new_map: Map<Key, Val> = Map::with_capacity(keys.len());
    for key in keys {
        let Val::Key(key) = key else {
            return bug!(cfg, "{GET_MANY}: expected input.item to be a key, but got {key}");
        };
        if let Some(val) = map.get(&key) {
            new_map.insert(key, val.clone());
        }
    }
    Val::Map(new_map.into())
}

pub fn remove(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{REMOVE}: expected context to be a map, but got {ctx}");
    };
    let Val::Key(key) = input else {
        return bug!(cfg, "{REMOVE}: expected input to be a key, but got {input}");
    };
    let Some(value) = map.remove(&key) else {
        return Val::default();
    };
    Val::Cell(Cell::new(value).into())
}

pub fn remove_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{REMOVE_MANY}: expected context to be a map, but got {ctx}");
    };
    let Val::List(keys) = input else {
        return bug!(cfg, "{REMOVE_MANY}: expected input to be a list, but got {input}");
    };
    let keys = List::from(keys);
    let mut new_map: Map<Key, Val> = Map::with_capacity(keys.len());
    for key in keys {
        let Val::Key(key) = key else {
            return bug!(cfg, "{REMOVE_MANY}: expected input.item to be a key, but got {key}");
        };
        if let Some(val) = map.remove(&key) {
            new_map.insert(key, val);
        }
    }
    Val::Map(new_map.into())
}

pub fn move_(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{MOVE}: expected context to be a map, but got {ctx}");
    };
    let Val::Key(key) = input else {
        return bug!(cfg, "{MOVE}: expected input to be a key, but got {input}");
    };
    let Some(value) = map.remove(&key) else {
        return bug!(cfg, "{MOVE}: value not found for key {key} in the map {map}");
    };
    value
}

pub fn clear(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::Map(map) = ctx else {
        return bug!(cfg, "{CLEAR}: expected context to be a map, but got {ctx}");
    };
    map.clear();
    Val::default()
}
