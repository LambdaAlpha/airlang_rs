use {
    crate::{
        semantics::{
            ctx::DefaultCtx,
            ctx_access::{
                constant::CtxForConstFn,
                mutable::CtxForMutableFn,
            },
            eval_mode::EvalMode,
            func::{
                CtxConstFn,
                CtxMutableFn,
                Primitive,
            },
            input_mode::InputMode,
            prelude::{
                names,
                PrimitiveFunc,
            },
            val::{
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

pub(crate) fn length() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_LENGTH, fn_length);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::Int(map.len().into())
    })
}

pub(crate) fn keys() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_KEYS, fn_keys);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_keys(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::List(map.keys().cloned().collect())
    })
}

pub(crate) fn into_keys() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_INTO_KEYS, fn_into_keys);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn values() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_VALUES, fn_values);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_values(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::Map(map) = val else {
            return Val::default();
        };
        Val::List(map.values().cloned().collect())
    })
}

pub(crate) fn into_values() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_INTO_VALUES, fn_into_values);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn contains() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_CONTAINS, fn_contains);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn contains_many() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_CONTAINS_MANY, fn_contains_many);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn set() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Pair(Box::new(Pair::new(
            InputMode::Any(EvalMode::Eval),
            InputMode::Any(EvalMode::Eval),
        ))),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_SET, fn_set);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn set_many() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Map(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_SET_MANY, fn_set_many);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn get() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_GET, fn_get);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn get_many() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxConstFn>::new(names::MAP_GET_MANY, fn_get_many);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_REMOVE, fn_remove);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn remove_many() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Symbol(EvalMode::Value),
        InputMode::List(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_REMOVE_MANY, fn_remove_many);
    PrimitiveFunc::new(input_mode, primitive)
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

pub(crate) fn clear() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::MAP_CLEAR, fn_clear);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_clear(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, input, |val| {
        let Val::Map(map) = val else {
            return;
        };
        map.clear();
    })
}
