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
            prelude::names,
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
    let name_or_list = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_list, |is_ref| {
        let f = |list: &Val| {
            let Val::List(list) = list else {
                return Val::default();
            };
            Val::Int(list.len().into())
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
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = ctx.eval_escape(list_pair.first);
    let index = ctx.eval(index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let mut value = ctx.eval(index_value.second);
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                let Some(current) = list.get_mut(i) else {
                    return Val::default();
                };
                swap(current, &mut value);
                value
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                let Some(current) = list.get_mut(i) else {
                    return Val::default();
                };
                *current = value;
                Val::List(list)
            })
        }
    })
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
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = ctx.eval_escape(list_pair.first);
    let index = ctx.eval(index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = ctx.eval(index_value.second) else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                let end = i + values.len();
                if end > list.len() {
                    return Val::default();
                }
                let ret = list.splice(i..end, values).collect();
                Val::List(ret)
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                let end = i + values.len();
                if end > list.len() {
                    return Val::default();
                };
                list.splice(i..end, values);
                Val::List(list)
            })
        }
    })
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
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name_or_list = ctx.eval_escape(name_index.first);
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(ctx, *range) else {
            return Val::default();
        };
        ctx.get_ref_or_val(name_or_list, |is_ref| {
            if is_ref {
                Either::Left(|list: &Val| {
                    let Val::List(list) = list else {
                        return Val::default();
                    };
                    let from = from.unwrap_or_default();
                    let to = to.unwrap_or(list.len());
                    let Some(slice) = list.get(from..to) else {
                        return Val::default();
                    };
                    Val::List(List::from(slice.to_owned()))
                })
            } else {
                Either::Right(|list| {
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
                })
            }
        })
    } else {
        let Some(i) = to_index(ctx.eval(name_index.second)) else {
            return Val::default();
        };
        ctx.get_ref_or_val(name_or_list, |is_ref| {
            if is_ref {
                Either::Left(|list: &Val| {
                    let Val::List(list) = list else {
                        return Val::default();
                    };
                    let Some(val) = list.get(i) else {
                        return Val::default();
                    };
                    val.clone()
                })
            } else {
                Either::Right(|list| {
                    let Val::List(mut list) = list else {
                        return Val::default();
                    };
                    if i >= list.len() {
                        return Val::default();
                    }
                    list.swap_remove(i)
                })
            }
        })
    }
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
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_pair.first);
    let index = ctx.eval(index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let value = ctx.eval(index_value.second);
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                if i > list.len() {
                    return Val::default();
                }
                list.insert(i, value);
                Val::default()
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                if i > list.len() {
                    return Val::default();
                }
                list.insert(i, value);
                Val::List(list)
            })
        }
    })
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
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_pair.first);
    let index = ctx.eval(index_value.first);
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = ctx.eval(index_value.second) else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val  else {
                    return Val::default();
                };
                if i > list.len() {
                    return Val::default();
                }
                list.splice(i..i, values);
                Val::default()
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                if i > list.len() {
                    return Val::default();
                }
                list.splice(i..i, values);
                Val::List(list)
            })
        }
    })
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
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name_or_list = ctx.eval_escape(name_index.first);
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(ctx, *range) else {
            return Val::default();
        };
        ctx.get_mut_or_val(name_or_list, |is_ref| {
            if is_ref {
                Either::Left(|list: &mut Val| {
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
                })
            } else {
                Either::Right(|list| {
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
                })
            }
        })
    } else {
        let Some(i) = to_index(ctx.eval(name_index.second))else {
            return Val::default();
        };
        ctx.get_mut_or_val(name_or_list, |is_ref| {
            if is_ref {
                Either::Left(|list: &mut Val| {
                    let Val::List(list) = list else {
                        return Val::default();
                    };
                    if i >= list.len() {
                        return Val::default();
                    }
                    list.remove(i)
                })
            } else {
                Either::Right(|list| {
                    let Val::List(mut list) = list else {
                        return Val::default();
                    };
                    if i >= list.len() {
                        return Val::default();
                    }
                    list.remove(i);
                    Val::List(list)
                })
            }
        })
    }
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
    let Val::Pair(name_value) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_value.first);
    let value = ctx.eval(name_value.second);
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                list.push(value);
                Val::default()
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                list.push(value);
                Val::List(list)
            })
        }
    })
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
    let Val::Pair(name_values) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_values.first);
    let values = ctx.eval(name_values.second);
    let Val::List(mut values) = values else {
        return Val::default();
    };
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                list.append(&mut values);
                Val::default()
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                list.append(&mut values);
                Val::List(list)
            })
        }
    })
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
    let Val::Pair(name_count) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_count.first);
    let count = ctx.eval(name_count.second);
    match count {
        Val::Unit(_) => ctx.get_mut_or_val(name, |is_ref| {
            if is_ref {
                Either::Left(|val: &mut Val| {
                    let Val::List(list) = val else {
                        return Val::default();
                    };
                    list.pop().unwrap_or_default()
                })
            } else {
                Either::Right(|val| {
                    let Val::List(mut list) = val else {
                        return Val::default();
                    };
                    if list.pop().is_none() {
                        return Val::default();
                    }
                    Val::List(list)
                })
            }
        }),
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
                return Val::default();
            };
            ctx.get_mut_or_val(name, |is_ref| {
                if is_ref {
                    Either::Left(|val: &mut Val| {
                        let Val::List(list) = val else {
                            return Val::default();
                        };
                        if i > list.len() {
                            return Val::default();
                        }
                        let start = list.len() - i;
                        let ret = list.split_off(start);
                        Val::List(ret.into())
                    })
                } else {
                    Either::Right(|val| {
                        let Val::List(mut list) = val else {
                            return Val::default();
                        };
                        if i > list.len() {
                            return Val::default();
                        }
                        let length = list.len() - i;
                        list.truncate(length);
                        Val::List(list)
                    })
                }
            })
        }
        _ => Val::default(),
    }
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
    let name = ctx.eval_escape(input);
    ctx.get_mut_or_val(name, |is_ref| {
        if is_ref {
            Either::Left(|val: &mut Val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                list.clear();
                Val::default()
            })
        } else {
            Either::Right(|val| {
                let Val::List(mut list) = val else {
                    return Val::default();
                };
                list.clear();
                Val::List(list)
            })
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
    Some((from, to))
}
