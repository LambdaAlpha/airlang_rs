use {
    crate::{
        semantics::{
            ctx::{
                DefaultCtx,
                NameMap,
            },
            ctx_access::{
                constant::CtxForConstFn,
                mutable::CtxForMutableFn,
            },
            eval_mode::EvalMode,
            io_mode::IoMode,
            prelude::{
                named_const_fn,
                named_free_fn,
                named_mutable_fn,
                Named,
                Prelude,
            },
            val::{
                FuncVal,
                MapVal,
                Val,
            },
        },
        types::{
            Bool,
            Int,
            Pair,
        },
    },
    std::mem::swap,
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
    pub(crate) contains_many: Named<FuncVal>,
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
            contains_many: contains_many(),
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
    fn put(&self, m: &mut NameMap) {
        self.length.put(m);
        self.items.put(m);
        self.into_items.put(m);
        self.keys.put(m);
        self.into_keys.put(m);
        self.values.put(m);
        self.into_values.put(m);
        self.contains.put(m);
        self.contains_many.put(m);
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
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("map.length", input_mode, output_mode, fn_length)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::Int(map.len().into())
    })
}

fn items() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::ListForAll(Box::new(IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )))));
    named_const_fn("map.items", input_mode, output_mode, fn_items)
}

fn fn_items(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let items = map
            .iter()
            .map(|(k, v)| Val::Pair(Box::new(Pair::new(k.clone(), v.clone()))))
            .collect();
        Val::List(items)
    })
}

fn into_items() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::ListForAll(Box::new(IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )))));
    named_mutable_fn("map.into_items", input_mode, output_mode, fn_into_items)
}

fn fn_into_items(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut origin = MapVal::default();
        swap(map, &mut origin);
        let items = origin
            .into_iter()
            .map(|(k, v)| Val::Pair(Box::new(Pair::new(k, v))))
            .collect();
        Val::List(items)
    })
}

fn keys() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::List(EvalMode::More);
    named_const_fn("map.keys", input_mode, output_mode, fn_keys)
}

fn fn_keys(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::List(map.keys().cloned().collect())
    })
}

fn into_keys() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::List(EvalMode::More);
    named_mutable_fn("map.into_keys", input_mode, output_mode, fn_into_keys)
}

fn fn_into_keys(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut origin = MapVal::default();
        swap(map, &mut origin);
        Val::List(origin.into_keys().collect())
    })
}

fn values() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::List(EvalMode::More);
    named_const_fn("map.values", input_mode, output_mode, fn_values)
}

fn fn_values(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::List(map.values().cloned().collect())
    })
}

fn into_values() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::List(EvalMode::More);
    named_mutable_fn("map.into_values", input_mode, output_mode, fn_into_values)
}

fn fn_into_values(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut origin = MapVal::default();
        swap(map, &mut origin);
        Val::List(origin.into_values().collect())
    })
}

fn contains() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("map.contains", input_mode, output_mode, fn_contains)
}

fn fn_contains(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = &name_key.second;
    DefaultCtx.get_const_ref(&ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::Bool(Bool::new(map.contains_key(key)))
    })
}

fn contains_many() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::List(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn(
        "map.contains_many",
        input_mode,
        output_mode,
        fn_contains_many,
    )
}

fn fn_contains_many(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    DefaultCtx.get_const_ref(&ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let b = keys.into_iter().all(|k| map.contains_key(&k));
        Val::Bool(Bool::new(b))
    })
}

fn set() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Pair(Box::new(Pair::new(
            IoMode::Any(EvalMode::More),
            IoMode::Any(EvalMode::More),
        ))),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("map.set", input_mode, output_mode, fn_set)
}

fn fn_set(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = name_pair.first;
    let Val::Pair(key_value) = name_pair.second else {
        return Val::default();
    };
    let key = key_value.first;
    let value = key_value.second;
    DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        map.insert(key, value).unwrap_or_default()
    })
}

fn set_many() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Map(EvalMode::More),
    )));
    let output_mode = IoMode::Map(EvalMode::More);
    named_mutable_fn("map.set_many", input_mode, output_mode, fn_set_many)
}

fn fn_set_many(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = name_pair.first;
    let Val::Map(update) = name_pair.second else {
        return Val::default();
    };
    DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let ret = update
            .into_iter()
            .filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v)))
            .collect();
        Val::Map(ret)
    })
}

fn get() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("map.get", input_mode, output_mode, fn_get)
}

fn fn_get(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = &name_key.second;
    DefaultCtx.get_const_ref(&ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        map.get(key).map(Clone::clone).unwrap_or_default()
    })
}

fn get_many() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::List(EvalMode::More),
    )));
    let output_mode = IoMode::Map(EvalMode::More);
    named_const_fn("map.get_many", input_mode, output_mode, fn_get_many)
}

fn fn_get_many(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    DefaultCtx.get_const_ref(&ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let ret = keys
            .into_iter()
            .filter_map(|k| map.get(&k).map(|v| (k, v.clone())))
            .collect();
        Val::Map(ret)
    })
}

fn remove() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("map.remove", input_mode, output_mode, fn_remove)
}

fn fn_remove(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = name_key.second;
    DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        map.remove(&key).unwrap_or_default()
    })
}

fn remove_many() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Symbol(EvalMode::Value),
        IoMode::List(EvalMode::More),
    )));
    let output_mode = IoMode::Map(EvalMode::More);
    named_mutable_fn("map.remove_many", input_mode, output_mode, fn_remove_many)
}

fn fn_remove_many(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let keys = name_keys.second;
    let Val::List(keys) = keys else {
        return Val::default();
    };
    DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let ret = keys
            .into_iter()
            .filter_map(|k| map.remove(&k).map(|v| (k, v)))
            .collect();
        Val::Map(ret)
    })
}

fn clear() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("map.clear", input_mode, output_mode, fn_clear)
}

fn fn_clear(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return;
        };
        map.clear();
    })
}

fn new_map() -> Named<FuncVal> {
    let input_mode = IoMode::ListForAll(Box::new(IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::Any(EvalMode::More),
    )))));
    let output_mode = IoMode::Map(EvalMode::More);
    named_free_fn("map", input_mode, output_mode, fn_new_map)
}

fn fn_new_map(input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let map = list
        .into_iter()
        .map(|item| {
            let Val::Pair(pair) = item else {
                return None;
            };
            Some((pair.first, pair.second))
        })
        .try_collect();
    match map {
        Some(map) => Val::Map(map),
        None => Val::default(),
    }
}

fn new_set() -> Named<FuncVal> {
    let input_mode = IoMode::List(EvalMode::More);
    let output_mode = IoMode::Map(EvalMode::More);
    named_free_fn("set", input_mode, output_mode, fn_new_set)
}

fn fn_new_set(input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let map = list.into_iter().map(|k| (k, Val::default())).collect();
    Val::Map(map)
}

fn new_multiset() -> Named<FuncVal> {
    let input_mode = IoMode::List(EvalMode::More);
    let output_mode = IoMode::Map(EvalMode::More);
    named_free_fn("multiset", input_mode, output_mode, fn_new_multiset)
}

fn fn_new_multiset(input: Val) -> Val {
    let Val::List(list) = input else {
        return Val::default();
    };
    let mut multiset = MapVal::with_capacity(list.len());
    for item in list {
        let count = multiset.entry(item).or_insert(Val::Int(Int::from(0)));
        let Val::Int(count) = count else {
            unreachable!()
        };
        count.increase();
    }
    Val::Map(multiset)
}
