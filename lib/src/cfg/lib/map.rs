use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct MapLib {
    pub length: ConstPrimFuncVal,
    pub items: ConstPrimFuncVal,
    pub into_items: MutPrimFuncVal,
    pub keys: ConstPrimFuncVal,
    pub into_keys: MutPrimFuncVal,
    pub values: ConstPrimFuncVal,
    pub into_values: MutPrimFuncVal,
    pub contain: ConstPrimFuncVal,
    pub contain_all: ConstPrimFuncVal,
    pub contain_any: ConstPrimFuncVal,
    pub set: MutPrimFuncVal,
    pub set_many: MutPrimFuncVal,
    pub get: ConstPrimFuncVal,
    pub get_many: ConstPrimFuncVal,
    pub remove: MutPrimFuncVal,
    pub remove_many: MutPrimFuncVal,
    pub clear: MutPrimFuncVal,
    pub new_map: FreePrimFuncVal,
    pub new_set: FreePrimFuncVal,
}

impl Default for MapLib {
    fn default() -> Self {
        MapLib {
            length: length(),
            items: items(),
            into_items: into_items(),
            keys: keys(),
            into_keys: into_keys(),
            values: values(),
            into_values: into_values(),
            contain: contain(),
            contain_all: contain_all(),
            contain_any: contain_any(),
            set: set(),
            set_many: set_many(),
            get: get(),
            get_many: get_many(),
            remove: remove(),
            remove_many: remove_many(),
            clear: clear(),
            new_map: new_map(),
            new_set: new_set(),
        }
    }
}

impl CfgMod for MapLib {
    fn extend(self, cfg: &Cfg) {
        self.length.extend(cfg);
        self.items.extend(cfg);
        self.into_items.extend(cfg);
        self.keys.extend(cfg);
        self.into_keys.extend(cfg);
        self.values.extend(cfg);
        self.into_values.extend(cfg);
        self.contain.extend(cfg);
        self.contain_all.extend(cfg);
        self.contain_any.extend(cfg);
        self.set.extend(cfg);
        self.set_many.extend(cfg);
        self.get.extend(cfg);
        self.get_many.extend(cfg);
        self.remove.extend(cfg);
        self.remove_many.extend(cfg);
        self.clear.extend(cfg);
        self.new_map.extend(cfg);
        self.new_set.extend(cfg);
    }
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.length", raw_input: false, f: const_impl(fn_length) }.const_()
}

fn fn_length(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let len: Int = map.len().into();
    Val::Int(len.into())
}

pub fn items() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.items", raw_input: false, f: const_impl(fn_items) }.const_()
}

fn fn_items(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let items: List<Val> = map
        .iter()
        .map(|(k, v)| Val::Pair(Pair::new(Val::Key(k.clone()), v.clone()).into()))
        .collect();
    Val::List(items.into())
}

pub fn into_items() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.into_items", raw_input: false, f: mut_impl(fn_into_items) }.mut_()
}

fn fn_into_items(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let items: List<Val> =
        origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(Val::Key(k), v).into())).collect();
    Val::List(items.into())
}

pub fn keys() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.keys", raw_input: false, f: const_impl(fn_keys) }.const_()
}

fn fn_keys(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let keys: List<Val> = map.keys().map(|k| Val::Key(k.clone())).collect();
    Val::List(keys.into())
}

pub fn into_keys() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.into_keys", raw_input: false, f: mut_impl(fn_into_keys) }.mut_()
}

fn fn_into_keys(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let keys: List<Val> = origin.into_keys().map(Val::Key).collect();
    Val::List(keys.into())
}

pub fn values() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.values", raw_input: false, f: const_impl(fn_values) }.const_()
}

fn fn_values(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let values: List<Val> = map.values().cloned().collect();
    Val::List(values.into())
}

pub fn into_values() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.into_values", raw_input: false, f: mut_impl(fn_into_values) }.mut_()
}

fn fn_into_values(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let values: List<Val> = origin.into_values().collect();
    Val::List(values.into())
}

pub fn contain() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.contain", raw_input: false, f: const_impl(fn_contain) }.const_()
}

fn fn_contain(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::Key(key) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    Val::Bit(Bit::from(map.contains_key(&key)))
}

pub fn contain_all() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.contain_all", raw_input: false, f: const_impl(fn_contain_all) }.const_()
}

fn fn_contain_all(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let keys = List::from(keys);
    for key in keys {
        let Val::Key(key) = key else {
            error!("input.item {key:?} should be a key");
            return illegal_input(cfg);
        };
        if !map.contains_key(&key) {
            return Val::Bit(Bit::from(false));
        }
    }
    Val::Bit(Bit::from(true))
}

pub fn contain_any() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.contain_any", raw_input: false, f: const_impl(fn_contain_any) }.const_()
}

fn fn_contain_any(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let keys = List::from(keys);
    for key in keys {
        let Val::Key(key) = key else {
            error!("input.item {key:?} should be a key");
            return illegal_input(cfg);
        };
        if map.contains_key(&key) {
            return Val::Bit(Bit::from(true));
        }
    }
    Val::Bit(Bit::from(false))
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.set", raw_input: false, f: mut_impl(fn_set) }.mut_()
}

fn fn_set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::Pair(key_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let key_value = Pair::from(key_value);
    let Val::Key(key) = key_value.first else {
        error!("input.first {:?} should be a key", key_value.first);
        return illegal_input(cfg);
    };
    let value = key_value.second;
    map.insert(key, value).unwrap_or_default()
}

pub fn set_many() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.set_many", raw_input: false, f: mut_impl(fn_set_many) }.mut_()
}

fn fn_set_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::Map(update) = input else {
        error!("input {input:?} should be a map");
        return illegal_input(cfg);
    };
    let update = Map::from(update);
    let map: Map<Key, Val> =
        update.into_iter().filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.get", raw_input: false, f: const_impl(fn_get) }.const_()
}

fn fn_get(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::Key(key) = input else {
        error!("input {input:?} should be key");
        return illegal_input(cfg);
    };
    map.get(&key).cloned().unwrap_or_default()
}

pub fn get_many() -> ConstPrimFuncVal {
    DynPrimFn { id: "_map.get_many", raw_input: false, f: const_impl(fn_get_many) }.const_()
}

fn fn_get_many(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let keys = List::from(keys);
    let mut new_map: Map<Key, Val> = Map::with_capacity(keys.len());
    for key in keys {
        let Val::Key(key) = key else {
            error!("input.item {key:?} should be a key");
            return illegal_input(cfg);
        };
        if let Some(val) = map.get(&key) {
            new_map.insert(key, val.clone());
        }
    }
    Val::Map(new_map.into())
}

pub fn remove() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.remove", raw_input: false, f: mut_impl(fn_remove) }.mut_()
}

fn fn_remove(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::Key(key) = input else {
        error!("input {input:?} should be key");
        return illegal_input(cfg);
    };
    map.remove(&key).unwrap_or_default()
}

pub fn remove_many() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.remove_many", raw_input: false, f: mut_impl(fn_remove_many) }.mut_()
}

fn fn_remove_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let keys = List::from(keys);
    let mut new_map: Map<Key, Val> = Map::with_capacity(keys.len());
    for key in keys {
        let Val::Key(key) = key else {
            error!("input.item {key:?} should be a key");
            return illegal_input(cfg);
        };
        if let Some(val) = map.remove(&key) {
            new_map.insert(key, val);
        }
    }
    Val::Map(new_map.into())
}

pub fn clear() -> MutPrimFuncVal {
    DynPrimFn { id: "_map.clear", raw_input: false, f: mut_impl(fn_clear) }.mut_()
}

fn fn_clear(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    map.clear();
    Val::default()
}

pub fn new_map() -> FreePrimFuncVal {
    FreePrimFn { id: "_map.new", raw_input: false, f: free_impl(fn_new_map) }.free()
}

fn fn_new_map(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let list = List::from(list);
    let mut map: Map<Key, Val> = Map::with_capacity(list.len());
    for item in list {
        let Val::Pair(pair) = item else {
            error!("input.item {item:?} should be a pair");
            return illegal_input(cfg);
        };
        let pair = Pair::from(pair);
        let Val::Key(key) = pair.first else {
            error!("input.item.first {:?} should be a key", pair.first);
            return illegal_input(cfg);
        };
        map.insert(key, pair.second);
    }
    Val::Map(map.into())
}

pub fn new_set() -> FreePrimFuncVal {
    FreePrimFn { id: "_map.new_set", raw_input: false, f: free_impl(fn_new_set) }.free()
}

fn fn_new_set(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let list = List::from(list);
    let mut map: Map<Key, Val> = Map::with_capacity(list.len());
    for item in list {
        let Val::Key(key) = item else {
            error!("input.item {item:?} should be a key");
            return illegal_input(cfg);
        };
        map.insert(key, Val::default());
    }
    Val::Map(map.into())
}
