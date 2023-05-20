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
            if let Val::Map(map) = map {
                Val::Int(map.len().into())
            } else {
                Val::default()
            }
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
                if let Val::Map(map) = val {
                    Val::List(map.keys().map(|v| v.clone()).collect())
                } else {
                    Val::default()
                }
            })
        } else {
            Either::Right(|val| {
                if let Val::Map(map) = val {
                    Val::List(map.into_keys().collect())
                } else {
                    Val::default()
                }
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
                if let Val::Map(map) = val {
                    Val::List(map.values().map(Clone::clone).collect())
                } else {
                    Val::default()
                }
            })
        } else {
            Either::Right(|val| {
                if let Val::Map(map) = val {
                    Val::List(map.into_values().collect())
                } else {
                    Val::default()
                }
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
    if let Val::Pair(name_key) = input {
        let name = fn_eval_escape(ctx, name_key.first);
        let key = ctx.eval(name_key.second);
        return ctx.eval_ref(name, |is_ref| {
            let f = |val: &Val| {
                if let Val::Map(map) = val {
                    return Val::Bool(Bool::new(map.contains_key(&key)));
                }
                Val::default()
            };
            if is_ref {
                Either::Left(f)
            } else {
                Either::Right(move |v| f(&v))
            }
        });
    }
    Val::default()
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
    if let Val::Pair(name_keys) = input {
        let name = fn_eval_escape(ctx, name_keys.first);
        let keys = ctx.eval(name_keys.second);
        if let Val::List(keys) = keys {
            return ctx.eval_ref(name, |is_ref| {
                let f = |val: &Val| {
                    if let Val::Map(map) = val {
                        let b = keys.into_iter().all(|k| map.contains_key(&k));
                        return Val::Bool(Bool::new(b));
                    }
                    Val::default()
                };
                if is_ref {
                    Either::Left(f)
                } else {
                    Either::Right(|v| f(&v))
                }
            });
        }
    }
    Val::default()
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
    if let Val::Pair(name_pair) = input {
        let name = fn_eval_escape(ctx, name_pair.first);
        if let Val::Pair(key_value) = name_pair.second {
            let key = ctx.eval(key_value.first);
            let value = ctx.eval(key_value.second);
            return ctx.eval_mut(name, |is_ref| {
                if is_ref {
                    Either::Left(|val: &mut Val| {
                        if let Val::Map(map) = val {
                            return map.insert(key, value).unwrap_or_default();
                        }
                        Val::default()
                    })
                } else {
                    Either::Right(|val| {
                        if let Val::Map(mut map) = val {
                            map.insert(key, value);
                            return Val::Map(map);
                        }
                        Val::default()
                    })
                }
            });
        }
    }
    Val::default()
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
    if let Val::Pair(name_pair) = input {
        let name = fn_eval_escape(ctx, name_pair.first);
        if let Val::Map(update) = name_pair.second {
            return ctx.eval_mut(name, |is_ref| {
                if is_ref {
                    Either::Left(|val: &mut Val| {
                        if let Val::Map(map) = val {
                            let ret = update
                                .into_iter()
                                .filter_map(|(k, v)| map.insert(k.clone(), v).map(|v| (k, v)))
                                .collect();
                            return Val::Map(ret);
                        }
                        Val::default()
                    })
                } else {
                    Either::Right(|val| {
                        if let Val::Map(mut map) = val {
                            map.extend(update);
                            return Val::Map(map);
                        }
                        Val::default()
                    })
                }
            });
        }
    }
    Val::default()
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
    if let Val::Pair(name_key) = input {
        let name = fn_eval_escape(ctx, name_key.first);
        let key = ctx.eval(name_key.second);
        return ctx.eval_ref(name, |is_ref| {
            if is_ref {
                Either::Left(|val: &Val| {
                    if let Val::Map(map) = val {
                        return map.get(&key).map(Clone::clone).unwrap_or_default();
                    }
                    Val::default()
                })
            } else {
                Either::Right(|val| {
                    if let Val::Map(mut map) = val {
                        return map.remove(&key).unwrap_or_default();
                    }
                    Val::default()
                })
            }
        });
    }
    Val::default()
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
    if let Val::Pair(name_keys) = input {
        let name = fn_eval_escape(ctx, name_keys.first);
        let keys = ctx.eval(name_keys.second);
        if let Val::List(keys) = keys {
            return ctx.eval_ref(name, |is_ref| {
                if is_ref {
                    Either::Left(|val: &Val| {
                        if let Val::Map(map) = val {
                            let ret = keys
                                .into_iter()
                                .filter_map(|k| map.get(&k).map(|v| (k, v.clone())))
                                .collect();
                            return Val::Map(ret);
                        }
                        Val::default()
                    })
                } else {
                    Either::Right(|val| {
                        if let Val::Map(mut map) = val {
                            let ret = keys
                                .into_iter()
                                .filter_map(|k| map.remove(&k).map(|v| (k, v)))
                                .collect();
                            return Val::Map(ret);
                        }
                        Val::default()
                    })
                }
            });
        }
    }
    Val::default()
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
    if let Val::Pair(name_key) = input {
        let name = fn_eval_escape(ctx, name_key.first);
        let key = ctx.eval(name_key.second);
        return ctx.eval_mut(name, |is_ref| {
            if is_ref {
                Either::Left(|val: &mut Val| {
                    if let Val::Map(map) = val {
                        return map.remove(&key).unwrap_or_default();
                    }
                    Val::default()
                })
            } else {
                Either::Right(|val| {
                    if let Val::Map(mut map) = val {
                        map.remove(&key);
                        return Val::Map(map);
                    }
                    Val::default()
                })
            }
        });
    }
    Val::default()
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
    if let Val::Pair(name_keys) = input {
        let name = fn_eval_escape(ctx, name_keys.first);
        let keys = ctx.eval(name_keys.second);
        if let Val::List(keys) = keys {
            return ctx.eval_mut(name, |is_ref| {
                if is_ref {
                    Either::Left(|val: &mut Val| {
                        if let Val::Map(map) = val {
                            let ret = keys
                                .into_iter()
                                .filter_map(|k| map.remove(&k).map(|v| (k, v)))
                                .collect();
                            return Val::Map(ret);
                        }
                        Val::default()
                    })
                } else {
                    Either::Right(|val| {
                        if let Val::Map(mut map) = val {
                            for k in keys {
                                map.remove(&k);
                            }
                            return Val::Map(map);
                        }
                        Val::default()
                    })
                }
            });
        }
    }
    Val::default()
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
    return ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                if let Val::Map(map) = val {
                    map.clear()
                }
                Val::default()
            })
        } else {
            Either::Right(|val| {
                if let Val::Map(mut map) = val {
                    map.clear();
                    return Val::Map(map);
                }
                Val::default()
            })
        }
    });
}
