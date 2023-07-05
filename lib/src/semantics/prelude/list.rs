use {
    crate::{
        semantics::{
            eval::{
                strategy::{
                    eval::DefaultStrategy,
                    inline::InlineStrategy,
                    EvalStrategy,
                },
                Ctx,
                EvalMode,
                Func,
                Primitive,
            },
            prelude::{
                names,
                prelude_func,
            },
            val::{
                PairVal,
                Val,
            },
        },
        types::{
            Either,
            List,
        },
    },
    std::mem::swap,
};

pub(crate) fn length() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::LIST_LENGTH,
        EvalMode::Inline,
        fn_length,
    )))
}

fn fn_length(ctx: &Ctx, input: Val) -> Val {
    ctx.get_ref_or_val(input, |ref_or_val| {
        let f = |list: &Val| {
            let Val::List(list) = list else {
                return Val::default();
            };
            Val::Int(list.len().into())
        };
        match ref_or_val {
            Either::Left(list) => f(list),
            Either::Right(list) => f(&list),
        }
    })
}

pub(crate) fn set() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_SET,
        EvalMode::Value,
        fn_set,
    )))
}

fn fn_set(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, list_pair.first);
    let index = DefaultStrategy::eval(ctx, index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let mut value = DefaultStrategy::eval(ctx, index_value.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val else {
                return Val::default();
            };
            let Some(current) = list.get_mut(i) else {
                return Val::default();
            };
            swap(current, &mut value);
            value
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            let Some(current) = list.get_mut(i) else {
                return Val::default();
            };
            *current = value;
            Val::List(list)
        }
    })
}

pub(crate) fn set_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_SET_MANY,
        EvalMode::Value,
        fn_set_many,
    )))
}

fn fn_set_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, list_pair.first);
    let index = DefaultStrategy::eval(ctx, index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = DefaultStrategy::eval(ctx, index_value.second) else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val else {
                return Val::default();
            };
            let end = i + values.len();
            if end > list.len() {
                return Val::default();
            }
            let ret = list.splice(i..end, values).collect();
            Val::List(ret)
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            let end = i + values.len();
            if end > list.len() {
                return Val::default();
            };
            list.splice(i..end, values);
            Val::List(list)
        }
    })
}

pub(crate) fn get() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_GET,
        EvalMode::Value,
        fn_get,
    )))
}

fn fn_get(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name_or_list = InlineStrategy::eval(ctx, name_index.first);
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(ctx, *range) else {
            return Val::default();
        };
        ctx.get_ref_or_val(name_or_list, |ref_or_val| match ref_or_val {
            Either::Left(list) => {
                let Val::List(list) = list else {
                    return Val::default();
                };
                let from = from.unwrap_or_default();
                let to = to.unwrap_or(list.len());
                let Some(slice) = list.get(from..to) else {
                    return Val::default();
                };
                Val::List(List::from(slice.to_owned()))
            }
            Either::Right(list) => {
                let Val::List(list) = list else {
                    return Val::default();
                };
                let from = from.unwrap_or_default();
                let to = to.unwrap_or(list.len());
                if from > to || to > list.len() {
                    return Val::default();
                }
                let slice = list.into_iter().skip(from).take(to - from).collect();
                Val::List(slice)
            }
        })
    } else {
        let Some(i) = to_index(DefaultStrategy::eval(ctx, name_index.second)) else {
            return Val::default();
        };
        ctx.get_ref_or_val(name_or_list, |ref_or_val| match ref_or_val {
            Either::Left(list) => {
                let Val::List(list) = list else {
                    return Val::default();
                };
                let Some(val) = list.get(i) else {
                    return Val::default();
                };
                val.clone()
            }
            Either::Right(list) => {
                let Val::List(mut list) = list else {
                    return Val::default();
                };
                if i >= list.len() {
                    return Val::default();
                }
                list.swap_remove(i)
            }
        })
    }
}

pub(crate) fn insert() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_INSERT,
        EvalMode::Value,
        fn_insert,
    )))
}

fn fn_insert(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_pair.first);
    let index = DefaultStrategy::eval(ctx, index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let value = DefaultStrategy::eval(ctx, index_value.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val else {
                return Val::default();
            };
            if i > list.len() {
                return Val::default();
            }
            list.insert(i, value);
            Val::default()
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            if i > list.len() {
                return Val::default();
            }
            list.insert(i, value);
            Val::List(list)
        }
    })
}

pub(crate) fn insert_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_INSERT_MANY,
        EvalMode::Value,
        fn_insert_many,
    )))
}

fn fn_insert_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_pair.first);
    let index = DefaultStrategy::eval(ctx, index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = DefaultStrategy::eval(ctx, index_value.second) else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val  else {
                return Val::default();
            };
            if i > list.len() {
                return Val::default();
            }
            list.splice(i..i, values);
            Val::default()
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            if i > list.len() {
                return Val::default();
            }
            list.splice(i..i, values);
            Val::List(list)
        }
    })
}

pub(crate) fn remove() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_REMOVE,
        EvalMode::Value,
        fn_remove,
    )))
}

fn fn_remove(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name_or_list = InlineStrategy::eval(ctx, name_index.first);
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(ctx, *range) else {
            return Val::default();
        };
        ctx.get_mut_or_val(name_or_list, |ref_or_val| match ref_or_val {
            Either::Left(list) => {
                let Val::List(list) = list  else {
                    return Val::default();
                };
                let from = from.unwrap_or_default();
                let to = to.unwrap_or(list.len());
                if from > to || to > list.len() {
                    return Val::default();
                }
                let ret = list.splice(from..to, Vec::new()).collect();
                Val::List(ret)
            }
            Either::Right(list) => {
                let Val::List(mut list) = list else {
                    return Val::default();
                };
                let from = from.unwrap_or_default();
                let to = to.unwrap_or(list.len());
                if from > to || to > list.len() {
                    return Val::default();
                }
                list.splice(from..to, Vec::new());
                Val::List(list)
            }
        })
    } else {
        let Some(i) = to_index(DefaultStrategy::eval(ctx, name_index.second))else {
            return Val::default();
        };
        ctx.get_mut_or_val(name_or_list, |ref_or_val| match ref_or_val {
            Either::Left(list) => {
                let Val::List(list) = list else {
                    return Val::default();
                };
                if i >= list.len() {
                    return Val::default();
                }
                list.remove(i)
            }
            Either::Right(list) => {
                let Val::List(mut list) = list else {
                    return Val::default();
                };
                if i >= list.len() {
                    return Val::default();
                }
                list.remove(i);
                Val::List(list)
            }
        })
    }
}

pub(crate) fn push() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_PUSH,
        EvalMode::Value,
        fn_push,
    )))
}

fn fn_push(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_value) = input else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_value.first);
    let value = DefaultStrategy::eval(ctx, name_value.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val else {
                return Val::default();
            };
            list.push(value);
            Val::default()
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            list.push(value);
            Val::List(list)
        }
    })
}

pub(crate) fn push_many() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_PUSH_MANY,
        EvalMode::Value,
        fn_push_many,
    )))
}

fn fn_push_many(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_values) = input else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_values.first);
    let values = DefaultStrategy::eval(ctx, name_values.second);
    let Val::List(mut values) = values else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val else {
                return Val::default();
            };
            list.append(&mut values);
            Val::default()
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            list.append(&mut values);
            Val::List(list)
        }
    })
}

pub(crate) fn pop() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_POP,
        EvalMode::Value,
        fn_pop,
    )))
}

fn fn_pop(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_count) = input else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_count.first);
    let count = DefaultStrategy::eval(ctx, name_count.second);
    match count {
        Val::Unit(_) => ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
            Either::Left(val) => {
                let Val::List(list) = val else {
                    return Val::default();
                };
                list.pop().unwrap_or_default()
            }
            Either::Right(val) => {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                if list.pop().is_none() {
                    return Val::default();
                }
                Val::List(list)
            }
        }),
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
                return Val::default();
            };
            ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
                Either::Left(val) => {
                    let Val::List(list) = val else {
                        return Val::default();
                    };
                    if i > list.len() {
                        return Val::default();
                    }
                    let start = list.len() - i;
                    let ret = list.split_off(start);
                    Val::List(ret.into())
                }
                Either::Right(val) => {
                    let Val::List(mut list) = val else {
                        return Val::default();
                    };
                    if i > list.len() {
                        return Val::default();
                    }
                    let length = list.len() - i;
                    list.truncate(length);
                    Val::List(list)
                }
            })
        }
        _ => Val::default(),
    }
}

pub(crate) fn clear() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::LIST_CLEAR,
        EvalMode::Inline,
        fn_clear,
    )))
}

fn fn_clear(ctx: &mut Ctx, input: Val) -> Val {
    ctx.get_mut_or_val(input, |ref_or_val| match ref_or_val {
        Either::Left(val) => {
            let Val::List(list) = val else {
                return Val::default();
            };
            list.clear();
            Val::default()
        }
        Either::Right(val) => {
            let Val::List(mut list) = val else {
                return Val::default();
            };
            list.clear();
            Val::List(list)
        }
    })
}

fn to_index(val: Val) -> Option<usize> {
    let Val::Int(i) = val else {
        return None;
    };
    i.to_usize()
}

fn to_range(ctx: &mut Ctx, pair: PairVal) -> Option<(Option<usize>, Option<usize>)> {
    let from = match DefaultStrategy::eval(ctx, pair.first) {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        _ => return None,
    };
    let to = match DefaultStrategy::eval(ctx, pair.second) {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        _ => return None,
    };
    Some((from, to))
}
