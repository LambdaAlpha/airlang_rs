use {
    crate::{
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

pub(crate) fn length() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::LIST_LENGTH, fn_length);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_length(mut ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_ref_val_or_default(&mut ctx, input, |ref_or_val| {
        let f = |list: &Val| {
            let Val::List(list) = list else {
                return Val::default();
            };
            Val::Int(list.len().into())
        };
        match ref_or_val {
            Either::Left(list) => f(list.as_const()),
            Either::Right(list) => f(&list),
        }
    })
}

pub(crate) fn set() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_SET, fn_set);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = list_pair.first;
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let mut value = index_value.second;
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn set_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_SET_MANY, fn_set_many);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set_many(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = list_pair.first;
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        return Val::default();
    };
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn get() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::LIST_GET, fn_get);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get(mut ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name = name_index.first;
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(*range) else {
            return Val::default();
        };
        DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(list) => {
                let Val::List(list) = list.as_const() else {
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
        let Some(i) = to_index(name_index.second) else {
            return Val::default();
        };
        DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(list) => {
                let Val::List(list) = list.as_const() else {
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

pub(crate) fn insert() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_INSERT, fn_insert);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_insert(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = name_pair.first;
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let value = index_value.second;
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn insert_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_INSERT_MANY, fn_insert_many);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_insert_many(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = name_pair.first;
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        return Val::default();
    };
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_REMOVE, fn_remove);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_remove(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name = name_index.first;
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(*range) else {
            return Val::default();
        };
        DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(mut list) => {
                let Some(Val::List(list)) = list.as_mut() else {
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
        let Some(i) = to_index(name_index.second) else {
            return Val::default();
        };
        DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
            Either::Left(mut list) => {
                let Some(Val::List(list)) = list.as_mut() else {
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

pub(crate) fn push() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_PUSH, fn_push);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_push(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_value) = input else {
        return Val::default();
    };
    let name = name_value.first;
    let value = name_value.second;
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn push_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_PUSH_MANY, fn_push_many);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_push_many(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_values) = input else {
        return Val::default();
    };
    let name = name_values.first;
    let values = name_values.second;
    let Val::List(mut values) = values else {
        return Val::default();
    };
    DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn pop() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_POP, fn_pop);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_pop(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(name_count) = input else {
        return Val::default();
    };
    let name = name_count.first;
    let count = name_count.second;
    match count {
        Val::Unit(_) => {
            DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
                Either::Left(mut val) => {
                    let Some(Val::List(list)) = val.as_mut() else {
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
            })
        }
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
                return Val::default();
            };
            DefaultCtx.get_ref_val_or_default(&mut ctx, name, |ref_or_val| match ref_or_val {
                Either::Left(mut val) => {
                    let Some(Val::List(list)) = val.as_mut() else {
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

pub(crate) fn clear() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_CLEAR, fn_clear);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_clear(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_ref_val_or_default(&mut ctx, input, |ref_or_val| match ref_or_val {
        Either::Left(mut val) => {
            let Some(Val::List(list)) = val.as_mut() else {
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

fn to_range(pair: PairVal) -> Option<(Option<usize>, Option<usize>)> {
    let from = match pair.first {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        _ => return None,
    };
    let to = match pair.second {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        _ => return None,
    };
    Some((from, to))
}
