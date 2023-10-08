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
        types::List,
    },
    std::mem::swap,
};

pub(crate) fn length() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxConstFn>::new(names::LIST_LENGTH, fn_length);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::List(list) = val else {
            return Val::default();
        };
        Val::Int(list.len().into())
    })
}

pub(crate) fn set() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
    DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
        let Val::List(list) = val else {
            return Val::default();
        };
        let Some(current) = list.get_mut(i) else {
            return Val::default();
        };
        swap(current, &mut value);
        value
    })
}

pub(crate) fn set_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
    DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
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
}

pub(crate) fn get() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxConstFn>::new(names::LIST_GET, fn_get);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_get(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name = name_index.first;
    if let Val::Pair(range) = name_index.second {
        let Some((from, to)) = to_range(*range) else {
            return Val::default();
        };
        DefaultCtx.get_const_ref(&ctx, name, |val| {
            let Val::List(list) = val else {
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
        let Some(i) = to_index(name_index.second) else {
            return Val::default();
        };
        DefaultCtx.get_const_ref(&ctx, name, |val| {
            let Val::List(list) = val else {
                return Val::default();
            };
            let Some(val) = list.get(i) else {
                return Val::default();
            };
            val.clone()
        })
    }
}

pub(crate) fn insert() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        if i > list.len() {
            return;
        }
        list.insert(i, value);
    })
}

pub(crate) fn insert_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        if i > list.len() {
            return;
        }
        list.splice(i..i, values);
    })
}

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
        DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
            let Val::List(list) = val else {
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
        let Some(i) = to_index(name_index.second) else {
            return Val::default();
        };
        DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
            let Val::List(list) = val else {
                return Val::default();
            };
            if i >= list.len() {
                return Val::default();
            }
            list.remove(i)
        })
    }
}

pub(crate) fn push() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        list.push(value);
    })
}

pub(crate) fn push_many() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        list.append(&mut values);
    })
}

pub(crate) fn pop() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Quote, BasicEvalMode::Eval)),
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
        Val::Unit(_) => DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
            let Val::List(list) = val else {
                return Val::default();
            };
            list.pop().unwrap_or_default()
        }),
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
                return Val::default();
            };
            DefaultCtx.get_mut_ref(&mut ctx, name, |val| {
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
        }
        _ => Val::default(),
    }
}

pub(crate) fn clear() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Quote);
    let primitive = Primitive::<CtxMutableFn>::new(names::LIST_CLEAR, fn_clear);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_clear(mut ctx: CtxForMutableFn, input: Val) -> Val {
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, input, |val| {
        let Val::List(list) = val else {
            return;
        };
        list.clear();
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
