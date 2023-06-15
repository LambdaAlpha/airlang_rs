use crate::{
    semantics::{
        eval::{
            Ctx,
            Func,
            Primitive,
        },
        prelude::names,
        val::Val,
    },
    types::{
        Bool,
        Either,
    },
};

pub(crate) fn length() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_LENGTH,
        fn_length,
    )))
    .into()
}

fn fn_length(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_map = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_map, |ref_or_val| {
        let f = |map: &Val| {
            let Val::Map(map) = map else {
                return Val::default();
            };
            Val::Int(map.len().into())
        };
        match ref_or_val {
            Either::Left(map) => f(map),
            Either::Right(map) => f(&map),
        }
    })
}

pub(crate) fn keys() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_KEYS,
        fn_keys,
    )))
    .into()
}

fn fn_keys(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_map = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_map, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_VALUES,
        fn_values,
    )))
    .into()
}

fn fn_values(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_map = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_map, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_CONTAINS,
        fn_contains,
    )))
    .into()
}

fn fn_contains(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_key.first);
    let key = ctx.eval_escape(name_key.second);
    ctx.get_ref_or_val(name, |ref_or_val| {
        let f = |val: &Val| {
            let Val::Map(map) = val  else {
                return Val::default();
            };
            Val::Bool(Bool::new(map.contains_key(&key)))
        };
        match ref_or_val {
            Either::Left(val) => f(val),
            Either::Right(val) => f(&val),
        }
    })
}

pub(crate) fn contains_many() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_CONTAINS_MANY,
        fn_contains_many,
    )))
    .into()
}

fn fn_contains_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_keys.first);
    let keys = ctx.eval_escape(name_keys.second);
    let Val::List(keys) = keys  else {
        return Val::default();
    };
    ctx.get_ref_or_val(name, |ref_or_val| {
        let f = |val: &Val| {
            let Val::Map(map) = val else {
                return Val::default();
            };
            let b = keys.into_iter().all(|k| map.contains_key(&k));
            Val::Bool(Bool::new(b))
        };
        match ref_or_val {
            Either::Left(val) => f(val),
            Either::Right(val) => f(&val),
        }
    })
}

pub(crate) fn set() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_SET,
        fn_set,
    )))
    .into()
}

fn fn_set(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_pair.first);
    let Val::Pair(key_value) = name_pair.second else {
        return Val::default();
    };
    let key = ctx.eval_escape(key_value.first);
    let value = ctx.eval(key_value.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_SET_MANY,
        fn_set_many,
    )))
    .into()
}

fn fn_set_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_pair.first);
    let Val::Map(update) = ctx.eval_bind(name_pair.second) else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_GET,
        fn_get,
    )))
    .into()
}

fn fn_get(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_key.first);
    let key = ctx.eval_escape(name_key.second);
    ctx.get_ref_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
                return Val::default();
            };
            map.get(&key).map(Clone::clone).unwrap_or_default()
        }
        Either::Right(val) => {
            let Val::Map(mut map) = val else {
                return Val::default();
            };
            map.remove(&key).unwrap_or_default()
        }
    })
}

pub(crate) fn get_many() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_GET_MANY,
        fn_get_many,
    )))
    .into()
}

fn fn_get_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_keys.first);
    let keys = ctx.eval_escape(name_keys.second);
    let Val::List(keys) = keys else {
        return Val::default();
    };
    ctx.get_ref_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_REMOVE,
        fn_remove,
    )))
    .into()
}

fn fn_remove(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_key.first);
    let key = ctx.eval_escape(name_key.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_REMOVE_MANY,
        fn_remove_many,
    )))
    .into()
}

fn fn_remove_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_keys.first);
    let keys = ctx.eval_escape(name_keys.second);
    let Val::List(keys) = keys else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::MAP_CLEAR,
        fn_clear,
    )))
    .into()
}

fn fn_clear(ctx: &mut Ctx, input: Val) -> Val {
    let name = ctx.eval_escape(input);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::Map(map) = val else {
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
