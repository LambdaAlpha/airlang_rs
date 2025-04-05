use std::mem::swap;

use crate::{
    ConstFnCtx,
    FuncMode,
    List,
    Map,
    Symbol,
    bit::Bit,
    ctx::{
        default::DefaultCtx,
        map::CtxValue,
        mut1::MutFnCtx,
    },
    int::Int,
    pair::Pair,
    prelude::{
        Named,
        Prelude,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        ref_pair_mode,
    },
    val::{
        Val,
        func::FuncVal,
    },
};

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
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.length.put(m);
        self.items.put(m);
        self.into_items.put(m);
        self.keys.put(m);
        self.into_keys.put(m);
        self.values.put(m);
        self.into_values.put(m);
        self.contains.put(m);
        self.contains_all.put(m);
        self.contains_any.put(m);
        self.set.put(m);
        self.set_many.put(m);
        self.get.put(m);
        self.get_many.put(m);
        self.remove.put(m);
        self.remove_many.put(m);
        self.clear.put(m);
        self.new_map.put(m);
        self.new_set.put(m);
        self.new_multiset.put(m);
    }
}

fn length() -> Named<FuncVal> {
    let id = "map.length";
    let f = fn_length;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let len: Int = map.len().into();
        Val::Int(len.into())
    })
}

fn items() -> Named<FuncVal> {
    let id = "map.items";
    let f = fn_items;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_items(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let items: List<Val> =
            map.iter().map(|(k, v)| Val::Pair(Pair::new(k.clone(), v.clone()).into())).collect();
        Val::List(items.into())
    })
}

fn into_items() -> Named<FuncVal> {
    let id = "map.into_items";
    let f = fn_into_items;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_into_items(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_mut_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut origin = Map::default();
        swap(&mut **map, &mut origin);
        let items: List<Val> =
            origin.into_iter().map(|(k, v)| Val::Pair(Pair::new(k, v).into())).collect();
        Val::List(items.into())
    })
}

fn keys() -> Named<FuncVal> {
    let id = "map.keys";
    let f = fn_keys;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_keys(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let keys: List<Val> = map.keys().cloned().collect();
        Val::List(keys.into())
    })
}

fn into_keys() -> Named<FuncVal> {
    let id = "map.into_keys";
    let f = fn_into_keys;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_into_keys(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_mut_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut origin = Map::default();
        swap(&mut **map, &mut origin);
        let keys: List<Val> = origin.into_keys().collect();
        Val::List(keys.into())
    })
}

fn values() -> Named<FuncVal> {
    let id = "map.values";
    let f = fn_values;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_values(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let values: List<Val> = map.values().cloned().collect();
        Val::List(values.into())
    })
}

fn into_values() -> Named<FuncVal> {
    let id = "map.into_values";
    let f = fn_into_values;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_into_values(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_mut_lossless(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut origin = Map::default();
        swap(&mut **map, &mut origin);
        let values: List<Val> = origin.into_values().collect();
        Val::List(values.into())
    })
}

fn contains() -> Named<FuncVal> {
    let id = "map.contains";
    let f = fn_contains;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_contains(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name_key = Pair::from(name_key);
    let name = name_key.first;
    let key = &name_key.second;
    DefaultCtx::with_ref_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::Bit(Bit::new(map.contains_key(key)))
    })
}

fn contains_all() -> Named<FuncVal> {
    let id = "map.contains_all";
    let f = fn_contains_all;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_contains_all(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name_keys = Pair::from(name_keys);
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    let keys = List::from(keys);
    DefaultCtx::with_ref_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let b = keys.into_iter().all(|k| map.contains_key(&k));
        Val::Bit(Bit::new(b))
    })
}

fn contains_any() -> Named<FuncVal> {
    let id = "map.contains_any";
    let f = fn_contains_many;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_contains_many(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name_keys = Pair::from(name_keys);
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    let keys = List::from(keys);
    DefaultCtx::with_ref_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let b = keys.into_iter().any(|k| map.contains_key(&k));
        Val::Bit(Bit::new(b))
    })
}

fn set() -> Named<FuncVal> {
    let id = "map.set";
    let f = fn_set;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_set(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name_pair = Pair::from(name_pair);
    let name = name_pair.first;
    let Val::Pair(key_value) = name_pair.second else {
        return Val::default();
    };
    let key_value = Pair::from(key_value);
    let key = key_value.first;
    let value = key_value.second;
    DefaultCtx::with_ref_mut_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        map.insert(key, value).unwrap_or_default()
    })
}

fn set_many() -> Named<FuncVal> {
    let id = "map.set_many";
    let f = fn_set_many;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_set_many(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name_pair = Pair::from(name_pair);
    let name = name_pair.first;
    let Val::Map(update) = name_pair.second else {
        return Val::default();
    };
    let update = Map::from(update);
    DefaultCtx::with_ref_mut_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let map: Map<Val, Val> = update
            .into_iter()
            .filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v)))
            .collect();
        Val::Map(map.into())
    })
}

fn get() -> Named<FuncVal> {
    let id = "map.get";
    let f = fn_get;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_get(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name_key = Pair::from(name_key);
    let name = name_key.first;
    let key = &name_key.second;
    DefaultCtx::with_ref_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        map.get(key).cloned().unwrap_or_default()
    })
}

fn get_many() -> Named<FuncVal> {
    let id = "map.get_many";
    let f = fn_get_many;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_const_fn(id, f, mode)
}

fn fn_get_many(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name_keys = Pair::from(name_keys);
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    let keys = List::from(keys);
    DefaultCtx::with_ref_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let map: Map<Val, Val> =
            keys.into_iter().filter_map(|k| map.get(&k).map(|v| (k, v.clone()))).collect();
        Val::Map(map.into())
    })
}

fn remove() -> Named<FuncVal> {
    let id = "map.remove";
    let f = fn_remove;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_remove(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name_key = Pair::from(name_key);
    let name = name_key.first;
    let key = name_key.second;
    DefaultCtx::with_ref_mut_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        map.remove(&key).unwrap_or_default()
    })
}

fn remove_many() -> Named<FuncVal> {
    let id = "map.remove_many";
    let f = fn_remove_many;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_remove_many(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name_keys = Pair::from(name_keys);
    let name = name_keys.first;
    let keys = name_keys.second;
    let Val::List(keys) = keys else {
        return Val::default();
    };
    let keys = List::from(keys);
    DefaultCtx::with_ref_mut_lossless(ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let map: Map<Val, Val> =
            keys.into_iter().filter_map(|k| map.remove(&k).map(|v| (k, v))).collect();
        Val::Map(map.into())
    })
}

fn clear() -> Named<FuncVal> {
    let id = "map.clear";
    let f = fn_clear;
    let call = ref_pair_mode();
    let class = call.clone();
    let inverse = FuncMode::default_mode();
    let mode = FuncMode { call, class, inverse };
    named_mut_fn(id, f, mode)
}

fn fn_clear(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    DefaultCtx::with_ref_mut_no_ret(ctx, pair.first, |val| {
        let Val::Map(map) = val else {
            return;
        };
        map.clear();
    })
}

fn new_map() -> Named<FuncVal> {
    let id = "map";
    let f = fn_new_map;
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
    let f = fn_new_set;
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
    let f = fn_new_multiset;
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
