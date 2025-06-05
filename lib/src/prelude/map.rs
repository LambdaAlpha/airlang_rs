use std::mem::swap;

use crate::ConstRef;
use crate::ConstStaticPrimFuncVal;
use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::List;
use crate::Map;
use crate::MutStaticPrimFuncVal;
use crate::bit::Bit;
use crate::int::Int;
use crate::pair::Pair;
use crate::prelude::DynFn;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::free_impl;
use crate::prelude::mut_impl;
use crate::val::Val;

// todo design
#[derive(Clone)]
pub(crate) struct MapPrelude {
    pub(crate) length: ConstStaticPrimFuncVal,
    pub(crate) items: ConstStaticPrimFuncVal,
    pub(crate) into_items: MutStaticPrimFuncVal,
    pub(crate) keys: ConstStaticPrimFuncVal,
    pub(crate) into_keys: MutStaticPrimFuncVal,
    pub(crate) values: ConstStaticPrimFuncVal,
    pub(crate) into_values: MutStaticPrimFuncVal,
    pub(crate) contains: ConstStaticPrimFuncVal,
    pub(crate) contains_all: ConstStaticPrimFuncVal,
    pub(crate) contains_any: ConstStaticPrimFuncVal,
    pub(crate) set: MutStaticPrimFuncVal,
    pub(crate) set_many: MutStaticPrimFuncVal,
    pub(crate) get: ConstStaticPrimFuncVal,
    pub(crate) get_many: ConstStaticPrimFuncVal,
    pub(crate) remove: MutStaticPrimFuncVal,
    pub(crate) remove_many: MutStaticPrimFuncVal,
    pub(crate) clear: MutStaticPrimFuncVal,
    pub(crate) new_map: FreeStaticPrimFuncVal,
    pub(crate) new_set: FreeStaticPrimFuncVal,
    pub(crate) new_multiset: FreeStaticPrimFuncVal,
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

fn length() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.length",
        f: const_impl(fn_length),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let len: Int = map.len().into();
    Val::Int(len.into())
}

fn items() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.items",
        f: const_impl(fn_items),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_items(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let items: List<Val> =
        map.iter().map(|(k, v)| Val::Pair(Pair::new(k.clone(), v.clone()).into())).collect();
    Val::List(items.into())
}

fn into_items() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.into_items",
        f: mut_impl(fn_into_items),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

fn keys() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.keys",
        f: const_impl(fn_keys),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_keys(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let keys: List<Val> = map.keys().cloned().collect();
    Val::List(keys.into())
}

fn into_keys() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.into_keys",
        f: mut_impl(fn_into_keys),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

fn values() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.values",
        f: const_impl(fn_values),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_values(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    let values: List<Val> = map.values().cloned().collect();
    Val::List(values.into())
}

fn into_values() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.into_values",
        f: mut_impl(fn_into_values),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

fn contains() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.contains",
        f: const_impl(fn_contains),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_contains(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    Val::Bit(Bit::new(map.contains_key(&input)))
}

fn contains_all() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.contains_all",
        f: const_impl(fn_contains_all),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
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

fn contains_any() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.contains_any",
        f: const_impl(fn_contains_many),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
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

fn set() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.set",
        f: mut_impl(fn_set),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

fn set_many() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.set_many",
        f: mut_impl(fn_set_many),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

fn get() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.get",
        f: const_impl(fn_get),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_get(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Map(map) = &*ctx else {
        return Val::default();
    };
    map.get(&input).cloned().unwrap_or_default()
}

fn get_many() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "map.get_many",
        f: const_impl(fn_get_many),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
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

fn remove() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.remove",
        f: mut_impl(fn_remove),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_remove(ctx: &mut Val, input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    map.remove(&input).unwrap_or_default()
}

fn remove_many() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.remove_many",
        f: mut_impl(fn_remove_many),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
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

fn clear() -> MutStaticPrimFuncVal {
    DynFn {
        id: "map.clear",
        f: mut_impl(fn_clear),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_clear(ctx: &mut Val, _input: Val) -> Val {
    let Val::Map(map) = ctx else {
        return Val::default();
    };
    map.clear();
    Val::default()
}

fn new_map() -> FreeStaticPrimFuncVal {
    FreeFn { id: "map", f: free_impl(fn_new_map), mode: FuncMode::default() }.free_static()
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

fn new_set() -> FreeStaticPrimFuncVal {
    FreeFn { id: "set", f: free_impl(fn_new_set), mode: FuncMode::default() }.free_static()
}

fn fn_new_set(input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let list = List::from(list);
    let map: Map<Val, Val> = list.into_iter().map(|k| (k, Val::default())).collect();
    Val::Map(map.into())
}

fn new_multiset() -> FreeStaticPrimFuncVal {
    FreeFn { id: "multiset", f: free_impl(fn_new_multiset), mode: FuncMode::default() }
        .free_static()
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
