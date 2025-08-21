use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::Prelude;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use crate::semantics::ctx::Ctx;
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
pub struct MapPrelude {
    pub length: ConstPrimFuncVal,
    pub items: ConstPrimFuncVal,
    pub into_items: MutPrimFuncVal,
    pub keys: ConstPrimFuncVal,
    pub into_keys: MutPrimFuncVal,
    pub values: ConstPrimFuncVal,
    pub into_values: MutPrimFuncVal,
    pub contains: ConstPrimFuncVal,
    pub contains_all: ConstPrimFuncVal,
    pub contains_any: ConstPrimFuncVal,
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

impl Default for MapPrelude {
    fn default() -> Self {
        MapPrelude {
            length: length(),
            items: items(),
            into_items: into_items(),
            keys: keys(),
            into_keys: into_keys(),
            values: values(),
            into_values: into_values(),
            contains: contains(),
            contains_all: contains_all(),
            contains_any: contains_any(),
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

impl Prelude for MapPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.length.put(ctx);
        self.items.put(ctx);
        self.into_items.put(ctx);
        self.keys.put(ctx);
        self.into_keys.put(ctx);
        self.values.put(ctx);
        self.into_values.put(ctx);
        self.contains.put(ctx);
        self.contains_all.put(ctx);
        self.contains_any.put(ctx);
        self.set.put(ctx);
        self.set_many.put(ctx);
        self.get.put(ctx);
        self.get_many.put(ctx);
        self.remove.put(ctx);
        self.remove_many.put(ctx);
        self.clear.put(ctx);
        self.new_map.put(ctx);
        self.new_set.put(ctx);
        self.new_multiset.put(ctx);
    }
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.length", f: const_impl(fn_length), mode: default_dyn_mode() }.const_()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let len: Int = map.len().into();
    Val::Int(len.into())
}

pub fn items() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.items", f: const_impl(fn_items), mode: default_dyn_mode() }.const_()
}

fn fn_items(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let items: List<Val> =
        map.iter().map(|(k, v)| Val::Pair(Pair::new(k.clone(), v.clone()).into())).collect();
    Val::List(items.into())
}

pub fn into_items() -> MutPrimFuncVal {
    DynPrimFn { id: "map.into_items", f: mut_impl(fn_into_items), mode: default_dyn_mode() }.mut_()
}

fn fn_into_items(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let items: List<Val> =
        origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(k, v).into())).collect();
    Val::List(items.into())
}

pub fn keys() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.keys", f: const_impl(fn_keys), mode: default_dyn_mode() }.const_()
}

fn fn_keys(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let keys: List<Val> = map.keys().cloned().collect();
    Val::List(keys.into())
}

pub fn into_keys() -> MutPrimFuncVal {
    DynPrimFn { id: "map.into_keys", f: mut_impl(fn_into_keys), mode: default_dyn_mode() }.mut_()
}

fn fn_into_keys(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let keys: List<Val> = origin.into_keys().collect();
    Val::List(keys.into())
}

pub fn values() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.values", f: const_impl(fn_values), mode: default_dyn_mode() }.const_()
}

fn fn_values(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let values: List<Val> = map.values().cloned().collect();
    Val::List(values.into())
}

pub fn into_values() -> MutPrimFuncVal {
    DynPrimFn { id: "map.into_values", f: mut_impl(fn_into_values), mode: default_dyn_mode() }
        .mut_()
}

fn fn_into_values(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let values: List<Val> = origin.into_values().collect();
    Val::List(values.into())
}

pub fn contains() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.contains", f: const_impl(fn_contains), mode: default_dyn_mode() }.const_()
}

fn fn_contains(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    Val::Bit(Bit::from(map.contains_key(&input)))
}

pub fn contains_all() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.contains_all", f: const_impl(fn_contains_all), mode: default_dyn_mode() }
        .const_()
}

fn fn_contains_all(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    let keys = List::from(keys);
    let b = keys.into_iter().all(|k| map.contains_key(&k));
    Val::Bit(Bit::from(b))
}

pub fn contains_any() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.contains_any", f: const_impl(fn_contains_many), mode: default_dyn_mode() }
        .const_()
}

fn fn_contains_many(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    let keys = List::from(keys);
    let b = keys.into_iter().any(|k| map.contains_key(&k));
    Val::Bit(Bit::from(b))
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { id: "map.set", f: mut_impl(fn_set), mode: default_dyn_mode() }.mut_()
}

fn fn_set(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let Val::Pair(key_value) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let key_value = Pair::from(key_value);
    let key = key_value.first;
    let value = key_value.second;
    map.insert(key, value).unwrap_or_default()
}

pub fn set_many() -> MutPrimFuncVal {
    DynPrimFn { id: "map.set_many", f: mut_impl(fn_set_many), mode: default_dyn_mode() }.mut_()
}

fn fn_set_many(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let Val::Map(update) = input else {
        error!("input {input:?} should be a map");
        return Val::default();
    };
    let update = Map::from(update);
    let map: Map<Val, Val> =
        update.into_iter().filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.get", f: const_impl(fn_get), mode: default_dyn_mode() }.const_()
}

fn fn_get(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    map.get(&input).cloned().unwrap_or_default()
}

pub fn get_many() -> ConstPrimFuncVal {
    DynPrimFn { id: "map.get_many", f: const_impl(fn_get_many), mode: default_dyn_mode() }.const_()
}

fn fn_get_many(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    let keys = List::from(keys);
    let map: Map<Val, Val> =
        keys.into_iter().filter_map(|k| map.get(&k).map(|v| (k, v.clone()))).collect();
    Val::Map(map.into())
}

pub fn remove() -> MutPrimFuncVal {
    DynPrimFn { id: "map.remove", f: mut_impl(fn_remove), mode: default_dyn_mode() }.mut_()
}

fn fn_remove(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    map.remove(&input).unwrap_or_default()
}

pub fn remove_many() -> MutPrimFuncVal {
    DynPrimFn { id: "map.remove_many", f: mut_impl(fn_remove_many), mode: default_dyn_mode() }
        .mut_()
}

fn fn_remove_many(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    let Val::List(keys) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    let keys = List::from(keys);
    let map: Map<Val, Val> =
        keys.into_iter().filter_map(|k| map.remove(&k).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

pub fn clear() -> MutPrimFuncVal {
    DynPrimFn { id: "map.clear", f: mut_impl(fn_clear), mode: default_dyn_mode() }.mut_()
}

fn fn_clear(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        error!("ctx {ctx:?} should be a map");
        return Val::default();
    };
    map.clear();
    Val::default()
}

pub fn new_map() -> FreePrimFuncVal {
    FreePrimFn { id: "map", f: free_impl(fn_new_map), mode: default_free_mode() }.free()
}

fn fn_new_map(input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
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
        None => Val::default(),
    }
}

pub fn new_set() -> FreePrimFuncVal {
    FreePrimFn { id: "set", f: free_impl(fn_new_set), mode: default_free_mode() }.free()
}

fn fn_new_set(input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    let list = List::from(list);
    let map: Map<Val, Val> = list.into_iter().map(|k| (k, Val::default())).collect();
    Val::Map(map.into())
}

pub fn new_multiset() -> FreePrimFuncVal {
    FreePrimFn { id: "multiset", f: free_impl(fn_new_multiset), mode: default_free_mode() }.free()
}

fn fn_new_multiset(input: Val) -> Val {
    let Val::List(list) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
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
