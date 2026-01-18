use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Cell;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct MapLib {
    pub get_length: ConstPrimFuncVal,
    pub get_items: ConstPrimFuncVal,
    pub into_items: MutPrimFuncVal,
    pub get_keys: ConstPrimFuncVal,
    pub into_keys: MutPrimFuncVal,
    pub get_values: ConstPrimFuncVal,
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
    pub move_: MutPrimFuncVal,
    pub clear: MutPrimFuncVal,
    pub new: FreePrimFuncVal,
    pub new_set: FreePrimFuncVal,
}

impl Default for MapLib {
    fn default() -> Self {
        MapLib {
            get_length: get_length(),
            get_items: get_items(),
            into_items: into_items(),
            get_keys: get_keys(),
            into_keys: into_keys(),
            get_values: get_values(),
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
            move_: move_(),
            clear: clear(),
            new: new(),
            new_set: new_set(),
        }
    }
}

impl CfgMod for MapLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_map.get_length", self.get_length);
        extend_func(cfg, "_map.get_items", self.get_items);
        extend_func(cfg, "_map.into_items", self.into_items);
        extend_func(cfg, "_map.get_keys", self.get_keys);
        extend_func(cfg, "_map.into_keys", self.into_keys);
        extend_func(cfg, "_map.get_values", self.get_values);
        extend_func(cfg, "_map.into_values", self.into_values);
        extend_func(cfg, "_map.contain", self.contain);
        extend_func(cfg, "_map.contain_all", self.contain_all);
        extend_func(cfg, "_map.contain_any", self.contain_any);
        extend_func(cfg, "_map.set", self.set);
        extend_func(cfg, "_map.set_many", self.set_many);
        extend_func(cfg, "_map.get", self.get);
        extend_func(cfg, "_map.get_many", self.get_many);
        extend_func(cfg, "_map.remove", self.remove);
        extend_func(cfg, "_map.remove_many", self.remove_many);
        extend_func(cfg, "_map.move", self.move_);
        extend_func(cfg, "_map.clear", self.clear);
        extend_func(cfg, "_map.new", self.new);
        extend_func(cfg, "_map.new_set", self.new_set);
    }
}

pub fn get_length() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_length) }.const_()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let len: Int = map.len().into();
    Val::Int(len.into())
}

pub fn get_items() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_items) }.const_()
}

fn fn_get_items(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let items: List<Val> = map
        .iter()
        .map(|(k, v)| Val::Pair(Pair::new(Val::Key(k.clone()), v.clone()).into()))
        .collect();
    Val::List(items.into())
}

pub fn into_items() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_into_items) }.mut_()
}

fn fn_into_items(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let items: List<Val> =
        origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(Val::Key(k), v).into())).collect();
    Val::List(items.into())
}

pub fn get_keys() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_keys) }.const_()
}

fn fn_get_keys(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let keys: List<Val> = map.keys().map(|k| Val::Key(k.clone())).collect();
    Val::List(keys.into())
}

pub fn into_keys() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_into_keys) }.mut_()
}

fn fn_into_keys(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let keys: List<Val> = origin.into_keys().map(Val::Key).collect();
    Val::List(keys.into())
}

pub fn get_values() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_values) }.const_()
}

fn fn_get_values(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let values: List<Val> = map.values().cloned().collect();
    Val::List(values.into())
}

pub fn into_values() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_into_values) }.mut_()
}

fn fn_into_values(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let values: List<Val> = origin.into_values().collect();
    Val::List(values.into())
}

pub fn contain() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_contain) }.const_()
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
    DynPrimFn { raw_input: false, f: const_impl(fn_contain_all) }.const_()
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
    DynPrimFn { raw_input: false, f: const_impl(fn_contain_any) }.const_()
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
    DynPrimFn { raw_input: false, f: mut_impl(fn_set) }.mut_()
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
    let Val::Key(key) = key_value.left else {
        error!("input.left {:?} should be a key", key_value.left);
        return illegal_input(cfg);
    };
    let value = key_value.right;
    let Some(value) = map.insert(key, value) else {
        return Val::default();
    };
    Val::Cell(Cell::new(value).into())
}

pub fn set_many() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_many) }.mut_()
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
    DynPrimFn { raw_input: false, f: const_impl(fn_get) }.const_()
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
    let Some(value) = map.get(&key) else {
        return Val::default();
    };
    Val::Cell(Cell::new(value.clone()).into())
}

pub fn get_many() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_many) }.const_()
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
    DynPrimFn { raw_input: false, f: mut_impl(fn_remove) }.mut_()
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
    let Some(value) = map.remove(&key) else {
        return Val::default();
    };
    Val::Cell(Cell::new(value).into())
}

pub fn remove_many() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_remove_many) }.mut_()
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

pub fn move_() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_move) }.mut_()
}

fn fn_move(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::Key(key) = input else {
        error!("input {input:?} should be key");
        return illegal_input(cfg);
    };
    let Some(value) = map.remove(&key) else {
        error!("key not exist");
        return abort_bug_with_msg(cfg, "key should exist in the map");
    };
    value
}

pub fn clear() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_clear) }.mut_()
}

fn fn_clear(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    map.clear();
    Val::default()
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
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
        let Val::Key(key) = pair.left else {
            error!("input.item.left {:?} should be a key", pair.left);
            return illegal_input(cfg);
        };
        map.insert(key, pair.right);
    }
    Val::Map(map.into())
}

pub fn new_set() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new_set) }.free()
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
