use {
    crate::{
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
            val::{
                PairVal,
                Val,
            },
        },
        types::{
            Either,
            List,
            Reader,
        },
    },
    std::mem::swap,
};

pub(crate) fn length() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_LENGTH),
            eval: Reader::new(fn_length),
        }),
    })
    .into()
}

fn fn_length(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_list = fn_eval_escape(ctx, input);
    ctx.eval_ref(name_or_list, |is_ref| {
        let f = |list: &Val| {
            if let Val::List(list) = list {
                Val::Int(list.len().into())
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

pub(crate) fn set() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_SET),
            eval: Reader::new(fn_set),
        }),
    })
    .into()
}

fn fn_set(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(list_pair) = input {
        if let Val::Pair(index_value) = list_pair.second {
            let name = fn_eval_escape(ctx, list_pair.first);
            let index = ctx.eval(index_value.first);
            if let Some(i) = to_index(index) {
                let mut value = ctx.eval(index_value.second);
                return ctx.eval_mut(name, |is_ref| {
                    if is_ref {
                        Either::Left(|val: &mut Val| {
                            if let Val::List(list) = val {
                                if let Some(current) = list.get_mut(i) {
                                    swap(current, &mut value);
                                    return value;
                                }
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|val| {
                            if let Val::List(mut list) = val {
                                if let Some(current) = list.get_mut(i) {
                                    *current = value;
                                    return Val::List(list);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
        }
    }
    Val::default()
}

pub(crate) fn set_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_SET_MANY),
            eval: Reader::new(fn_set_many),
        }),
    })
    .into()
}

fn fn_set_many(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(list_pair) = input {
        if let Val::Pair(index_value) = list_pair.second {
            let name = fn_eval_escape(ctx, list_pair.first);
            let index = ctx.eval(index_value.first);
            if let Some(i) = to_index(index) {
                if let Val::List(values) = ctx.eval(index_value.second) {
                    return ctx.eval_mut(name, |is_ref| {
                        if is_ref {
                            Either::Left(|val: &mut Val| {
                                if let Val::List(list) = val {
                                    let end = i + values.len();
                                    if end <= list.len() {
                                        let ret = list.splice(i..end, values).collect();
                                        return Val::List(ret);
                                    }
                                }
                                Val::default()
                            })
                        } else {
                            Either::Right(|val| {
                                if let Val::List(mut list) = val {
                                    let end = i + values.len();
                                    if end <= list.len() {
                                        list.splice(i..end, values);
                                        return Val::List(list);
                                    }
                                }
                                Val::default()
                            })
                        }
                    });
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn get() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_GET),
            eval: Reader::new(fn_get),
        }),
    })
    .into()
}

fn fn_get(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_index) = input {
        let name_or_list = fn_eval_escape(ctx, name_index.first);
        if let Val::Pair(range) = name_index.second {
            if let Some((from, to)) = to_range(ctx, *range) {
                return ctx.eval_ref(name_or_list, |is_ref| {
                    if is_ref {
                        Either::Left(|list: &Val| {
                            if let Val::List(list) = list {
                                let from = from.unwrap_or_default();
                                let to = to.unwrap_or(list.len());
                                if let Some(slice) = list.get(from..to) {
                                    return Val::List(List::from(slice.to_owned()));
                                }
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|list| {
                            if let Val::List(list) = list {
                                let from = from.unwrap_or_default();
                                let to = to.unwrap_or(list.len());
                                if from <= to && to <= list.len() {
                                    let slice =
                                        list.into_iter().skip(from).take(to - from).collect();
                                    return Val::List(slice);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
        } else {
            if let Some(i) = to_index(ctx.eval(name_index.second)) {
                return ctx.eval_ref(name_or_list, |is_ref| {
                    if is_ref {
                        Either::Left(|list: &Val| {
                            if let Val::List(list) = list {
                                if let Some(val) = list.get(i) {
                                    return val.clone();
                                }
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|list| {
                            if let Val::List(mut list) = list {
                                if i < list.len() {
                                    return list.swap_remove(i);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
        }
    }

    Val::default()
}

pub(crate) fn insert() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_INSERT),
            eval: Reader::new(fn_insert),
        }),
    })
    .into()
}

fn fn_insert(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_pair) = input {
        if let Val::Pair(index_value) = name_pair.second {
            let name = fn_eval_escape(ctx, name_pair.first);
            let index = ctx.eval(index_value.first);
            if let Some(i) = to_index(index) {
                let value = ctx.eval(index_value.second);
                return ctx.eval_mut(name, |is_ref| {
                    if is_ref {
                        Either::Left(|val: &mut Val| {
                            if let Val::List(list) = val {
                                if i <= list.len() {
                                    list.insert(i, value)
                                }
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|val| {
                            if let Val::List(mut list) = val {
                                if i <= list.len() {
                                    list.insert(i, value);
                                    return Val::List(list);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
        }
    }
    Val::default()
}

pub(crate) fn insert_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_INSERT_MANY),
            eval: Reader::new(fn_insert_many),
        }),
    })
    .into()
}

fn fn_insert_many(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_pair) = input {
        if let Val::Pair(index_value) = name_pair.second {
            let name = fn_eval_escape(ctx, name_pair.first);
            let index = ctx.eval(index_value.first);
            if let Some(i) = to_index(index) {
                if let Val::List(values) = ctx.eval(index_value.second) {
                    return ctx.eval_mut(name, |is_ref| {
                        if is_ref {
                            Either::Left(|val: &mut Val| {
                                if let Val::List(list) = val {
                                    if i <= list.len() {
                                        list.splice(i..i, values);
                                    }
                                }
                                Val::default()
                            })
                        } else {
                            Either::Right(|val| {
                                if let Val::List(mut list) = val {
                                    if i <= list.len() {
                                        list.splice(i..i, values);
                                        return Val::List(list);
                                    }
                                }
                                Val::default()
                            })
                        }
                    });
                }
            }
        }
    }
    Val::default()
}

pub(crate) fn remove() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_REMOVE),
            eval: Reader::new(fn_remove),
        }),
    })
    .into()
}

fn fn_remove(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_index) = input {
        let name_or_list = fn_eval_escape(ctx, name_index.first);
        if let Val::Pair(range) = name_index.second {
            if let Some((from, to)) = to_range(ctx, *range) {
                return ctx.eval_mut(name_or_list, |is_ref| {
                    if is_ref {
                        Either::Left(|list: &mut Val| {
                            if let Val::List(list) = list {
                                let from = from.unwrap_or_default();
                                let to = to.unwrap_or(list.len());
                                if from <= to && to <= list.len() {
                                    let ret = list.splice(from..to, Vec::new()).collect();
                                    return Val::List(ret);
                                }
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|list| {
                            if let Val::List(mut list) = list {
                                let from = from.unwrap_or_default();
                                let to = to.unwrap_or(list.len());
                                if from <= to && to <= list.len() {
                                    list.splice(from..to, Vec::new());
                                    return Val::List(list);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
        } else {
            if let Some(i) = to_index(ctx.eval(name_index.second)) {
                return ctx.eval_mut(name_or_list, |is_ref| {
                    if is_ref {
                        Either::Left(|list: &mut Val| {
                            if let Val::List(list) = list {
                                if i < list.len() {
                                    return list.remove(i);
                                }
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|list| {
                            if let Val::List(mut list) = list {
                                if i < list.len() {
                                    list.remove(i);
                                    return Val::List(list);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
        }
    }

    Val::default()
}

pub(crate) fn push() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_PUSH),
            eval: Reader::new(fn_push),
        }),
    })
    .into()
}

fn fn_push(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_value) = input {
        let name = fn_eval_escape(ctx, name_value.first);
        let value = ctx.eval(name_value.second);
        return ctx.eval_mut(name, |is_ref| {
            if is_ref {
                Either::Left(|val: &mut Val| {
                    if let Val::List(list) = val {
                        list.push(value);
                    }
                    Val::default()
                })
            } else {
                Either::Right(|val| {
                    if let Val::List(mut list) = val {
                        list.push(value);
                        return Val::List(list);
                    }
                    Val::default()
                })
            }
        });
    }
    Val::default()
}

pub(crate) fn push_many() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_PUSH_MANY),
            eval: Reader::new(fn_push_many),
        }),
    })
    .into()
}

fn fn_push_many(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_values) = input {
        let name = fn_eval_escape(ctx, name_values.first);
        let values = ctx.eval(name_values.second);
        if let Val::List(mut values) = values {
            return ctx.eval_mut(name, |is_ref| {
                if is_ref {
                    Either::Left(|val: &mut Val| {
                        if let Val::List(list) = val {
                            list.append(&mut values);
                        }
                        Val::default()
                    })
                } else {
                    Either::Right(|val| {
                        if let Val::List(mut list) = val {
                            list.append(&mut values);
                            return Val::List(list);
                        }
                        Val::default()
                    })
                }
            });
        }
    }
    Val::default()
}

pub(crate) fn pop() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_POP),
            eval: Reader::new(fn_pop),
        }),
    })
    .into()
}

fn fn_pop(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(name_count) = input {
        let name = fn_eval_escape(ctx, name_count.first);
        let count = ctx.eval(name_count.second);
        match count {
            Val::Unit(_) => {
                return ctx.eval_mut(name, |is_ref| {
                    if is_ref {
                        Either::Left(|val: &mut Val| {
                            if let Val::List(list) = val {
                                return list.pop().unwrap_or_default();
                            }
                            Val::default()
                        })
                    } else {
                        Either::Right(|val| {
                            if let Val::List(mut list) = val {
                                if list.pop().is_some() {
                                    return Val::List(list);
                                }
                            }
                            Val::default()
                        })
                    }
                });
            }
            Val::Int(i) => {
                if let Some(i) = i.to_usize() {
                    return ctx.eval_mut(name, |is_ref| {
                        if is_ref {
                            Either::Left(|val: &mut Val| {
                                if let Val::List(list) = val {
                                    if i <= list.len() {
                                        let start = list.len() - i;
                                        let ret = list.split_off(start);
                                        return Val::List(ret.into());
                                    }
                                }
                                Val::default()
                            })
                        } else {
                            Either::Right(|val| {
                                if let Val::List(mut list) = val {
                                    if i <= list.len() {
                                        let length = list.len() - i;
                                        list.truncate(length);
                                        return Val::List(list);
                                    }
                                }
                                Val::default()
                            })
                        }
                    });
                }
            }
            _ => {}
        }
    }
    Val::default()
}

pub(crate) fn clear() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::LIST_CLEAR),
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
                if let Val::List(list) = val {
                    list.clear()
                }
                Val::default()
            })
        } else {
            Either::Right(|val| {
                if let Val::List(mut list) = val {
                    list.clear();
                    return Val::List(list);
                }
                Val::default()
            })
        }
    });
}

fn to_index(val: Val) -> Option<usize> {
    if let Val::Int(i) = val {
        i.to_usize()
    } else {
        None
    }
}

fn to_range(ctx: &mut Ctx, pair: PairVal) -> Option<(Option<usize>, Option<usize>)> {
    let from = match ctx.eval(pair.first) {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        _ => return None,
    };
    let to = match ctx.eval(pair.second) {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        _ => return None,
    };
    return Some((from, to));
}
