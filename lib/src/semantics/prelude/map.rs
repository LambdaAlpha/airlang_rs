use crate::{
    semantics::{
        eval::{
            ctx::Ctx,
            BasicEvalMode,
            EvalMode,
            Func,
            IsConst,
            Primitive,
        },
        prelude::{
            names,
            prelude_func,
        },
        val::Val,
    },
    types::{
        Bool,
        Either,
    },
};

pub(crate) fn length() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_LENGTH,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_length,
    )))
}

fn fn_length(ctx: &mut Ctx, input: Val) -> Val {
    ctx.get_ref_or_val_or_default(true, input, |ref_or_val| {
        let f = |map: &Val| {
            let Val::Map(map) = map else {
                return Val::default();
            };
            Val::Int(map.len().into())
        };
        match ref_or_val {
            Either::Left(map) => f(map.as_const()),
            Either::Right(map) => f(&map),
        }
    })
}

pub(crate) fn keys() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_KEYS,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_keys,
    )))
}

fn fn_keys(ctx: &mut Ctx, input: Val) -> Val {
    ctx.get_ref_or_val_or_default(true, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val.as_const() else {
                return Val::default();
            };
            Val::List(map.keys().cloned().collect())
        }
        Either::Right(val) => {
            let Val::Map(map) = val else {
                return Val::default();
            };
            Val::List(map.into_keys().collect())
        }
    })
}

pub(crate) fn values() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_VALUES,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_values,
    )))
}

fn fn_values(ctx: &mut Ctx, input: Val) -> Val {
    ctx.get_ref_or_val_or_default(true, input, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val.as_const() else {
                return Val::default();
            };
            Val::List(map.values().cloned().collect())
        }
        Either::Right(val) => {
            let Val::Map(map) = val else {
                return Val::default();
            };
            Val::List(map.into_values().collect())
        }
    })
}

pub(crate) fn contains() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_CONTAINS,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_contains,
    )))
}

fn fn_contains(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = &name_key.second;
    ctx.get_ref_or_val_or_default(true, name, |ref_or_val| {
        let f = |val: &Val| {
            let Val::Map(map) = val else {
                return Val::default();
            };
            Val::Bool(Bool::new(map.contains_key(key)))
        };
        match ref_or_val {
            Either::Left(val) => f(val.as_const()),
            Either::Right(val) => f(&val),
        }
    })
}

pub(crate) fn contains_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_CONTAINS_MANY,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_contains_many,
    )))
}

fn fn_contains_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    ctx.get_ref_or_val_or_default(true, name, |ref_or_val| {
        let f = |val: &Val| {
            let Val::Map(map) = val else {
                return Val::default();
            };
            let b = keys.into_iter().all(|k| map.contains_key(&k));
            Val::Bool(Bool::new(b))
        };
        match ref_or_val {
            Either::Left(val) => f(val.as_const()),
            Either::Right(val) => f(&val),
        }
    })
}

pub(crate) fn set() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::MAP_SET,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Eval,
            non_pair: BasicEvalMode::Value,
        },
        fn_set,
    )))
}

fn fn_set(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = name_pair.first;
    let Val::Pair(key_value) = name_pair.second else {
        return Val::default();
    };
    let key = key_value.first;
    let value = key_value.second;
    ctx.get_ref_or_val_or_default(is_const, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::Map(map)) = val.as_mut() else {
                return Val::default();
            };
            map.insert(key, value).unwrap_or_default()
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            map.insert(key, value);
            Val::Map(map)
        }
    })
}

pub(crate) fn set_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::MAP_SET_MANY,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Eval,
            non_pair: BasicEvalMode::Value,
        },
        fn_set_many,
    )))
}

fn fn_set_many(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = name_pair.first;
    let Val::Map(update) = name_pair.second else {
        return Val::default();
    };
    ctx.get_ref_or_val_or_default(is_const, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::Map(map)) = val.as_mut() else {
                return Val::default();
            };
            let ret = update
                .into_iter()
                .filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v)))
                .collect();
            Val::Map(ret)
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            map.extend(update);
            Val::Map(map)
        }
    })
}

pub(crate) fn get() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_GET,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_get,
    )))
}

fn fn_get(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = &name_key.second;
    ctx.get_ref_or_val_or_default(true, name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val.as_const() else {
                return Val::default();
            };
            map.get(key).map(Clone::clone).unwrap_or_default()
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            map.remove(key).unwrap_or_default()
        }
    })
}

pub(crate) fn get_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::MAP_GET_MANY,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_get_many,
    )))
}

fn fn_get_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    ctx.get_ref_or_val_or_default(true, name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val.as_const() else {
                return Val::default();
            };
            let ret = keys
                .into_iter()
                .filter_map(|k| map.get(&k).map(|v| (k, v.clone())))
                .collect();
            Val::Map(ret)
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            let ret = keys
                .into_iter()
                .filter_map(|k| map.remove(&k).map(|v| (k, v)))
                .collect();
            Val::Map(ret)
        }
    })
}

pub(crate) fn remove() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::MAP_REMOVE,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_remove,
    )))
}

fn fn_remove(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = name_key.second;
    ctx.get_ref_or_val_or_default(is_const, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::Map(map)) = val.as_mut() else {
                return Val::default();
            };
            map.remove(&key).unwrap_or_default()
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            map.remove(&key);
            Val::Map(map)
        }
    })
}

pub(crate) fn remove_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::MAP_REMOVE_MANY,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_remove_many,
    )))
}

fn fn_remove_many(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let keys = name_keys.second;
    let Val::List(keys) = keys else {
        return Val::default();
    };
    ctx.get_ref_or_val_or_default(is_const, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::Map(map)) = val.as_mut() else {
                return Val::default();
            };
            let ret = keys
                .into_iter()
                .filter_map(|k| map.remove(&k).map(|v| (k, v)))
                .collect();
            Val::Map(ret)
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            for k in keys {
                map.remove(&k);
            }
            Val::Map(map)
        }
    })
}

pub(crate) fn clear() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::MAP_CLEAR,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_clear,
    )))
}

fn fn_clear(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    ctx.get_ref_or_val_or_default(is_const, input, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::Map(map)) = val.as_mut() else {
                return Val::default();
            };
            map.clear();
            Val::default()
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            map.clear();
            Val::Map(map)
        }
    })
}
