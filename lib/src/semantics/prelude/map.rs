use crate::{
    semantics::{
        ctx::DefaultCtx,
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
        },
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxConstFn,
            CtxMutableFn,
            Primitive,
        },
        prelude::{
            names,
            PrimitiveFunc,
        },
        val::Val,
    },
    types::{
        Bool,
        Either,
    },
};

pub(crate) fn length() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_LENGTH, fn_length);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_length(mut ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_ref_val_or_default(&mut ctx, input, |ref_or_val| {
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

pub(crate) fn keys() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_KEYS, fn_keys);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_keys(mut ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_ref_val_or_default(&mut ctx, input, |ref_or_val| match ref_or_val {
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

pub(crate) fn values() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_VALUES, fn_values);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_values(mut ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_ref_val_or_default(&mut ctx, input, |ref_or_val| match ref_or_val {
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

pub(crate) fn contains() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_CONTAINS, fn_contains);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_contains(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = &name_key.second;
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| {
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

pub(crate) fn contains_many() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_CONTAINS_MANY, fn_contains_many);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_contains_many(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| {
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

pub(crate) fn set() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_SET, fn_set);
    PrimitiveFunc::new(eval_mode, primitive)
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
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
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

pub(crate) fn set_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_SET_MANY, fn_set_many);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set_many(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = name_pair.first;
    let Val::Map(update) = name_pair.second else {
        return Val::default();
    };
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
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

pub(crate) fn get() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_GET, fn_get);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = &name_key.second;
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
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

pub(crate) fn get_many() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_GET_MANY, fn_get_many);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get_many(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = name_keys.first;
    let Val::List(keys) = name_keys.second else {
        return Val::default();
    };
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
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

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_REMOVE, fn_remove);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_remove(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = name_key.first;
    let key = name_key.second;
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
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

pub(crate) fn remove_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_REMOVE_MANY, fn_remove_many);
    PrimitiveFunc::new(eval_mode, primitive)
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
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
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

pub(crate) fn clear() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_CLEAR, fn_clear);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_clear(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_ref_val_or_default(&mut ctx, input, |ref_or_val| match ref_or_val {
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
