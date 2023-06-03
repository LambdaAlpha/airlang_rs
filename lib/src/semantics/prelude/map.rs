use crate::{
    semantics::{
        eval::{
            Ctx,
            Func,
            FuncImpl,
            FuncTrait,
            Name,
            Primitive,
        },
        prelude::{
            eval::fn_eval_escape,
            names,
        },
        val::Val,
    },
    types::{
        Bool,
        Either,
        Pair,
        Reader,
    },
};

pub(crate) fn map_new() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_NEW),
            eval: Reader::new(fn_map_new),
        }),
    })
    .into()
}

pub(crate) fn fn_map_new(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Map(m) => {
            let map = m
                .into_iter()
                .map(|(k, v)| {
                    let key = fn_eval_escape(ctx, k);
                    let value = fn_map_new(ctx, v);
                    (key, value)
                })
                .collect();
            Val::Map(map)
        }
        Val::Pair(p) => {
            let first = fn_map_new(ctx, p.first);
            let second = fn_map_new(ctx, p.second);
            let pair = Box::new(Pair::new(first, second));
            Val::Pair(pair)
        }
        Val::List(l) => {
            let list = l.into_iter().map(|v| fn_map_new(ctx, v)).collect();
            Val::List(list)
        }
        i => ctx.eval(i),
    }
}

pub(crate) fn length() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_LENGTH),
            eval: Reader::new(fn_length),
        }),
    })
    .into()
}

fn fn_length(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_map = fn_eval_escape(ctx, input);
    ctx.eval_ref(name_or_map, |is_ref| {
        let f = |map: &Val| {
            let Val::Map(map) = map else {
                return Val::default();
            };
            Val::Int(map.len().into())
        };
        if is_ref {
            Either::Left(f)
        } else {
            Either::Right(move |v| f(&v))
        }
    })
}

pub(crate) fn keys() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_KEYS),
            eval: Reader::new(fn_keys),
        }),
    })
    .into()
}

fn fn_keys(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_map = fn_eval_escape(ctx, input);
    ctx.eval_ref(name_or_map, |is_ref| {
        if is_ref {
            Either::Left(|val: &Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                Val::List(map.keys().cloned().collect())
            })
        } else {
            Either::Right(|val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                Val::List(map.into_keys().collect())
            })
        }
    })
}

pub(crate) fn values() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_VALUES),
            eval: Reader::new(fn_values),
        }),
    })
    .into()
}

fn fn_values(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_map = fn_eval_escape(ctx, input);
    ctx.eval_ref(name_or_map, |is_ref| {
        if is_ref {
            Either::Left(|val: &Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                Val::List(map.values().cloned().collect())
            })
        } else {
            Either::Right(|val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                Val::List(map.into_values().collect())
            })
        }
    })
}

pub(crate) fn contains() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_CONTAINS),
            eval: Reader::new(fn_contains),
        }),
    })
    .into()
}

fn fn_contains(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_key.first);
    let key = ctx.eval(name_key.second);
    ctx.eval_ref(name, |is_ref| {
        let f = |val: &Val| {
            let Val::Map(map) = val  else {
                return Val::default();
            };
            Val::Bool(Bool::new(map.contains_key(&key)))
        };
        if is_ref {
            Either::Left(f)
        } else {
            Either::Right(move |v| f(&v))
        }
    })
}

pub(crate) fn contains_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_CONTAINS_MANY),
            eval: Reader::new(fn_contains_many),
        }),
    })
    .into()
}

fn fn_contains_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_keys.first);
    let keys = ctx.eval(name_keys.second);
    let Val::List(keys) = keys  else {
        return Val::default();
    };
    ctx.eval_ref(name, |is_ref| {
        let f = |val: &Val| {
            let Val::Map(map) = val else {
                return Val::default();
            };
            let b = keys.into_iter().all(|k| map.contains_key(&k));
            Val::Bool(Bool::new(b))
        };
        if is_ref {
            Either::Left(f)
        } else {
            Either::Right(|v| f(&v))
        }
    })
}

pub(crate) fn set() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_SET),
            eval: Reader::new(fn_set),
        }),
    })
    .into()
}

fn fn_set(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_pair.first);
    let Val::Pair(key_value) = name_pair.second else {
        return Val::default();
    };
    let key = ctx.eval(key_value.first);
    let value = ctx.eval(key_value.second);
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                map.insert(key, value).unwrap_or_default()
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                map.insert(key, value);
                Val::Map(map)
            })
        }
    })
}

pub(crate) fn set_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_SET_MANY),
            eval: Reader::new(fn_set_many),
        }),
    })
    .into()
}

fn fn_set_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_pair.first);
    let Val::Map(update) = name_pair.second else {
        return Val::default();
    };
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                let ret = update
                    .into_iter()
                    .filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v)))
                    .collect();
                Val::Map(ret)
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                map.extend(update);
                Val::Map(map)
            })
        }
    })
}

pub(crate) fn get() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_GET),
            eval: Reader::new(fn_get),
        }),
    })
    .into()
}

fn fn_get(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_key.first);
    let key = ctx.eval(name_key.second);
    ctx.eval_ref(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                map.get(&key).map(Clone::clone).unwrap_or_default()
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                map.remove(&key).unwrap_or_default()
            })
        }
    })
}

pub(crate) fn get_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_GET_MANY),
            eval: Reader::new(fn_get_many),
        }),
    })
    .into()
}

fn fn_get_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_keys.first);
    let keys = ctx.eval(name_keys.second);
    let Val::List(keys) = keys else {
        return Val::default();
    };
    ctx.eval_ref(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                let ret = keys
                    .into_iter()
                    .filter_map(|k| map.get(&k).map(|v| (k, v.clone())))
                    .collect();
                Val::Map(ret)
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                let ret = keys
                    .into_iter()
                    .filter_map(|k| map.remove(&k).map(|v| (k, v)))
                    .collect();
                Val::Map(ret)
            })
        }
    })
}

pub(crate) fn remove() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_REMOVE),
            eval: Reader::new(fn_remove),
        }),
    })
    .into()
}

fn fn_remove(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_key) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_key.first);
    let key = ctx.eval(name_key.second);
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                map.remove(&key).unwrap_or_default()
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                map.remove(&key);
                Val::Map(map)
            })
        }
    })
}

pub(crate) fn remove_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_REMOVE_MANY),
            eval: Reader::new(fn_remove_many),
        }),
    })
    .into()
}

fn fn_remove_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_keys) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_keys.first);
    let keys = ctx.eval(name_keys.second);
    let Val::List(keys) = keys else {
        return Val::default();
    };
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                let ret = keys
                    .into_iter()
                    .filter_map(|k| map.remove(&k).map(|v| (k, v)))
                    .collect();
                Val::Map(ret)
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                for k in keys {
                    map.remove(&k);
                }
                Val::Map(map)
            })
        }
    })
}

pub(crate) fn clear() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MAP_CLEAR),
            eval: Reader::new(fn_clear),
        }),
    })
    .into()
}

fn fn_clear(ctx: &mut Ctx, input: Val) -> Val {
    let name = fn_eval_escape(ctx, input);
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::Map(map) = val else {
                    return Val::default();
                };
                map.clear();
                Val::default()
            })
        } else {
            Either::Right(|val| {
                let Val::Map(mut map) = val else {
                    return Val::default();
                };
                map.clear();
                Val::Map(map)
            })
        }
    })
}
