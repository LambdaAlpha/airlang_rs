use std::mem::swap;

use crate::ConstRef;
use crate::FuncMode;
use crate::List;
use crate::Map;
use crate::bit::Bit;
use crate::int::Int;
use crate::pair::Pair;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_free_fn;
use crate::prelude::named_mut_fn;
use crate::val::Val;
use crate::val::func::FuncVal;

#[derive(Clone)]
pub(crate) struct MapPrelude {
    pub(crate) length: Named<FuncVal>,
    pub(crate) items: Named<FuncVal>,
    pub(crate) into_items: Named<FuncVal>,
    pub(crate) keys: Named<FuncVal>,
    pub(crate) into_keys: Named<FuncVal>,
    pub(crate) values: Named<FuncVal>,
    pub(crate) into_values: Named<FuncVal>,
    pub(crate) contains: Named<FuncVal>,
    pub(crate) contains_all: Named<FuncVal>,
    pub(crate) contains_any: Named<FuncVal>,
    pub(crate) set: Named<FuncVal>,
    pub(crate) set_many: Named<FuncVal>,
    pub(crate) get: Named<FuncVal>,
    pub(crate) get_many: Named<FuncVal>,
    pub(crate) remove: Named<FuncVal>,
    pub(crate) remove_many: Named<FuncVal>,
    pub(crate) clear: Named<FuncVal>,
    pub(crate) new_map: Named<FuncVal>,
    pub(crate) new_set: Named<FuncVal>,
    pub(crate) new_multiset: Named<FuncVal>,
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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
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

fn length() -> Named<FuncVal> {
    let id = "map.length";
    let f = const_impl(fn_length);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let len: Int = map.len().into();
    Val::Int(len.into())
}

fn items() -> Named<FuncVal> {
    let id = "map.items";
    let f = const_impl(fn_items);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_items(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let items: List<Val> =
        map.iter().map(|(k, v)| Val::Pair(Pair::new(k.clone(), v.clone()).into())).collect();
    Val::List(items.into())
}

fn into_items() -> Named<FuncVal> {
    let id = "map.into_items";
    let f = mut_impl(fn_into_items);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_into_items(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let items: List<Val> =
        origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(k, v).into())).collect();
    Val::List(items.into())
}

fn keys() -> Named<FuncVal> {
    let id = "map.keys";
    let f = const_impl(fn_keys);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_keys(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let keys: List<Val> = map.keys().cloned().collect();
    Val::List(keys.into())
}

fn into_keys() -> Named<FuncVal> {
    let id = "map.into_keys";
    let f = mut_impl(fn_into_keys);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_into_keys(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let keys: List<Val> = origin.into_keys().collect();
    Val::List(keys.into())
}

fn values() -> Named<FuncVal> {
    let id = "map.values";
    let f = const_impl(fn_values);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_values(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let values: List<Val> = map.values().cloned().collect();
    Val::List(values.into())
}

fn into_values() -> Named<FuncVal> {
    let id = "map.into_values";
    let f = mut_impl(fn_into_values);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_into_values(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    let mut origin = Map::default();
    swap(&mut **map, &mut origin);
    let values: List<Val> = origin.into_values().collect();
    Val::List(values.into())
}

fn contains() -> Named<FuncVal> {
    let id = "map.contains";
    let f = const_impl(fn_contains);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_contains(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(map.contains_key(&input)))
}

fn contains_all() -> Named<FuncVal> {
    let id = "map.contains_all";
    let f = const_impl(fn_contains_all);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_contains_all(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let Val::List(keys) = input else {
        return Val::default();
    };
    let keys = List::from(keys);
    let b = keys.into_iter().all(|k| map.contains_key(&k));
    Val::Bit(Bit::new(b))
}

fn contains_any() -> Named<FuncVal> {
    let id = "map.contains_any";
    let f = const_impl(fn_contains_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_contains_many(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let Val::List(keys) = input else {
        return Val::default();
    };
    let keys = List::from(keys);
    let b = keys.into_iter().any(|k| map.contains_key(&k));
    Val::Bit(Bit::new(b))
}

fn set() -> Named<FuncVal> {
    let id = "map.set";
    let f = mut_impl(fn_set);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_set(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    let Val::Pair(key_value) = input else {
        return Val::default();
    };
    let key_value = Pair::from(key_value);
    let key = key_value.first;
    let value = key_value.second;
    map.insert(key, value).unwrap_or_default()
}

fn set_many() -> Named<FuncVal> {
    let id = "map.set_many";
    let f = mut_impl(fn_set_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_set_many(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    let Val::Map(update) = input else {
        return Val::default();
    };
    let update = Map::from(update);
    let map: Map<Val, Val> =
        update.into_iter().filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

fn get() -> Named<FuncVal> {
    let id = "map.get";
    let f = const_impl(fn_get);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_get(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    map.get(&input).cloned().unwrap_or_default()
}

fn get_many() -> Named<FuncVal> {
    let id = "map.get_many";
    let f = const_impl(fn_get_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_get_many(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let Val::List(keys) = input else {
        return Val::default();
    };
    let keys = List::from(keys);
    let map: Map<Val, Val> =
        keys.into_iter().filter_map(|k| map.get(&k).map(|v| (k, v.clone()))).collect();
    Val::Map(map.into())
}

fn remove() -> Named<FuncVal> {
    let id = "map.remove";
    let f = mut_impl(fn_remove);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_remove(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    map.remove(&input).unwrap_or_default()
}

fn remove_many() -> Named<FuncVal> {
    let id = "map.remove_many";
    let f = mut_impl(fn_remove_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_remove_many(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    let Val::List(keys) = input else {
        return Val::default();
    };
    let keys = List::from(keys);
    let map: Map<Val, Val> =
        keys.into_iter().filter_map(|k| map.remove(&k).map(|v| (k, v))).collect();
    Val::Map(map.into())
}

fn clear() -> Named<FuncVal> {
    let id = "map.clear";
    let f = mut_impl(fn_clear);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_clear(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    map.clear();
    Val::default()
}

fn new_map() -> Named<FuncVal> {
    let id = "map";
    let f = free_impl(fn_new_map);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new_map(input: Val) -> Val {
    let Val::List(list) = input else {
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

fn new_set() -> Named<FuncVal> {
    let id = "set";
    let f = free_impl(fn_new_set);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new_set(input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let list = List::from(list);
    let map: Map<Val, Val> = list.into_iter().map(|k| (k, Val::default())).collect();
    Val::Map(map.into())
}

fn new_multiset() -> Named<FuncVal> {
    let id = "multiset";
    let f = free_impl(fn_new_multiset);
    let mode = FuncMode::default();
    named_free_fn(id, f, mode)
}

fn fn_new_multiset(input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let list = List::from(list);
    let mut multiset = Map::with_capacity(list.len());
    for item in list {
        let count = multiset.entry(item).or_insert(Val::Int(Int::from(0).into()));
        let Val::Int(count) = count else { unreachable!() };
        count.increase();
    }
    Val::Map(multiset.into())
}
