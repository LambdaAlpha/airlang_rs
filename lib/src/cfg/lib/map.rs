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
    pub new_multiset: FreePrimFuncVal,
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
            new_multiset: new_multiset(),
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
        self.new_multiset.extend(cfg);
    }
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.length", f: const_impl(fn_length) }.const_()
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
    DynPrimFn { id: "map.items", f: const_impl(fn_items) }.const_()
}

fn fn_items(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let items: List<Val> =
        map.iter().map(|(k, v)| Val::Pair(Pair::new(k.clone(), v.clone()).into())).collect();
    Val::List(items.into())
}

pub fn into_items() -> MutPrimFuncVal {
    DynPrimFn { id: "map.into_items", f: mut_impl(fn_into_items) }.mut_()
}

fn fn_into_items(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let items: List<Val> =
        origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(k, v).into())).collect();
    Val::List(items.into())
}

pub fn keys() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.keys", f: const_impl(fn_keys) }.const_()
}

fn fn_keys(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let keys: List<Val> = map.keys().cloned().collect();
    Val::List(keys.into())
}

pub fn into_keys() -> MutPrimFuncVal {
    DynPrimFn { id: "map.into_keys", f: mut_impl(fn_into_keys) }.mut_()
}

fn fn_into_keys(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let keys: List<Val> = origin.into_keys().collect();
    Val::List(keys.into())
}

pub fn values() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.values", f: const_impl(fn_values) }.const_()
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
    DynPrimFn { id: "map.into_values", f: mut_impl(fn_into_values) }.mut_()
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
    DynPrimFn { id: "map.contain", f: const_impl(fn_contain) }.const_()
}

fn fn_contain(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    Val::Bit(Bit::from(map.contains_key(&input)))
}

pub fn contain_all() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.contain_all", f: const_impl(fn_contain_all) }.const_()
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
    let b = keys.into_iter().all(|k| map.contains_key(&k));
    Val::Bit(Bit::from(b))
}

pub fn contain_any() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.contain_any", f: const_impl(fn_contain_many) }.const_()
}

fn fn_contain_many(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let keys = List::from(keys);
    let b = keys.into_iter().any(|k| map.contains_key(&k));
    Val::Bit(Bit::from(b))
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { id: "map.set", f: mut_impl(fn_set) }.mut_()
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
    let key = key_value.first;
    let value = key_value.second;
    map.insert(key, value).unwrap_or_default()
}

pub fn set_many() -> MutPrimFuncVal {
    DynPrimFn { id: "map.set_many", f: mut_impl(fn_set_many) }.mut_()
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
    let map: Map<Val, Val> =
        update.into_iter().filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.get", f: const_impl(fn_get) }.const_()
}

fn fn_get(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    map.get(&input).cloned().unwrap_or_default()
}

pub fn get_many() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.get_many", f: const_impl(fn_get_many) }.const_()
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
    let map: Map<Val, Val> =
        keys.into_iter().filter_map(|k| map.get(&k).map(|v| (k, v.clone()))).collect();
    Val::Map(map.into())
}

pub fn remove() -> MutPrimFuncVal {
    DynPrimFn { id: "map.remove", f: mut_impl(fn_remove) }.mut_()
}

fn fn_remove(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return illegal_ctx(cfg);
    };
    map.remove(&input).unwrap_or_default()
}

pub fn remove_many() -> MutPrimFuncVal {
    DynPrimFn { id: "map.remove_many", f: mut_impl(fn_remove_many) }.mut_()
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
    let map: Map<Val, Val> =
        keys.into_iter().filter_map(|k| map.remove(&k).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

pub fn clear() -> MutPrimFuncVal {
    DynPrimFn { id: "map.clear", f: mut_impl(fn_clear) }.mut_()
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
    FreePrimFn { id: "map.new", f: free_impl(fn_new_map) }.free()
}

fn fn_new_map(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let list = List::from(list);
    let map: Option<Map<Val, Val>> = list
        .into_iter()
        .map(|item| {
            let Val::Pair(pair) = item else {
                return None;
            };
            let pair = Pair::from(pair);
            Some((pair.first, pair.second))
        })
        .collect();
    match map {
        Some(map) => Val::Map(map.into()),
        None => illegal_input(cfg),
    }
}

pub fn new_set() -> FreePrimFuncVal {
    FreePrimFn { id: "map.new_set", f: free_impl(fn_new_set) }.free()
}

fn fn_new_set(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let list = List::from(list);
    let map: Map<Val, Val> = list.into_iter().map(|k| (k, Val::default())).collect();
    Val::Map(map.into())
}

pub fn new_multiset() -> FreePrimFuncVal {
    FreePrimFn { id: "map.new_multiset", f: free_impl(fn_new_multiset) }.free()
}

fn fn_new_multiset(cfg: &mut Cfg, input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    let list = List::from(list);
    let mut multiset = Map::with_capacity(list.len());
    for item in list {
        let count = multiset.entry(item).or_insert(Val::Int(Int::from(0).into()));
        let Val::Int(count) = count else { unreachable!() };
        ***count += 1;
    }
    Val::Map(multiset.into())
}
