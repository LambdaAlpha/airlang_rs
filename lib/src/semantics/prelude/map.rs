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
            input_mode::InputMode,
            prelude::{
                named_const_fn,
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
            Pair,
        },
    },
    std::mem::swap,
};

#[derive(Clone)]
pub(crate) struct MapPrelude {
    length: Named<FuncVal>,
    keys: Named<FuncVal>,
    into_keys: Named<FuncVal>,
    values: Named<FuncVal>,
    into_values: Named<FuncVal>,
    contains: Named<FuncVal>,
    contains_many: Named<FuncVal>,
    set: Named<FuncVal>,
    set_many: Named<FuncVal>,
    get: Named<FuncVal>,
    get_many: Named<FuncVal>,
    remove: Named<FuncVal>,
    remove_many: Named<FuncVal>,
    clear: Named<FuncVal>,
}

impl Default for MapPrelude {
    fn default() -> Self {
        MapPrelude {
            length: length(),
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
        }
    }
}

impl Prelude for MapPrelude {
    fn put(&self, m: &mut NameMap) {
        self.length.put(m);
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
    }
}

fn length() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("map_length", input_mode, fn_length)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::Int(map.len().into())
    })
}

fn keys() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("map_keys", input_mode, fn_keys)
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
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_mutable_fn("map_into_keys", input_mode, fn_into_keys)
}

fn fn_into_keys(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut map1 = MapVal::default();
        swap(map, &mut map1);
        Val::List(map1.into_keys().collect())
    })
}

fn values() -> Named<FuncVal> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_const_fn("map_values", input_mode, fn_values)
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
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_mutable_fn("map_into_values", input_mode, fn_into_values)
}

fn fn_into_values(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        let mut map1 = MapVal::default();
        swap(map, &mut map1);
        Val::List(map1.into_values().collect())
    })
}

fn contains() -> Named<FuncVal> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    named_const_fn("map_contains", input_mode, fn_contains)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Eval),
    )));
    named_const_fn("map_contains_many", input_mode, fn_contains_many)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Pair(Box::new(Pair::new(
            InputMode::Any(EvalMode::Eval),
            InputMode::Any(EvalMode::Eval),
        ))),
    )));
    named_mutable_fn("map_set", input_mode, fn_set)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Map(EvalMode::Eval),
    )));
    named_mutable_fn("map_set_many", input_mode, fn_set_many)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    named_const_fn("map_get", input_mode, fn_get)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Eval),
    )));
    named_const_fn("map_get_many", input_mode, fn_get_many)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    named_mutable_fn("map_remove", input_mode, fn_remove)
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
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Eval),
    )));
    named_mutable_fn("map_remove_many", input_mode, fn_remove_many)
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
    let input_mode = InputMode::Symbol(EvalMode::Value);
    named_mutable_fn("map_clear", input_mode, fn_clear)
}

fn fn_clear(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return;
        };
        map.clear();
    })
}
