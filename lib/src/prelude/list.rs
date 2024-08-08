use std::mem::swap;

use crate::{
    ctx::{
        const1::ConstFnCtx,
        default::DefaultCtx,
        mut1::MutFnCtx,
        CtxValue,
    },
    list::List,
    prelude::{
        named_const_fn,
        named_mut_fn,
        Named,
        Prelude,
    },
    val::{
        func::FuncVal,
        Val,
    },
    Int,
    Map,
    Mode,
    Pair,
    Symbol,
};

#[derive(Clone)]
pub(crate) struct ListPrelude {
    pub(crate) length: Named<FuncVal>,
    pub(crate) set: Named<FuncVal>,
    pub(crate) set_many: Named<FuncVal>,
    pub(crate) get: Named<FuncVal>,
    pub(crate) insert: Named<FuncVal>,
    pub(crate) insert_many: Named<FuncVal>,
    pub(crate) remove: Named<FuncVal>,
    pub(crate) push: Named<FuncVal>,
    pub(crate) push_many: Named<FuncVal>,
    pub(crate) pop: Named<FuncVal>,
    pub(crate) clear: Named<FuncVal>,
}

impl Default for ListPrelude {
    fn default() -> Self {
        ListPrelude {
            length: length(),
            set: set(),
            set_many: set_many(),
            get: get(),
            insert: insert(),
            insert_many: insert_many(),
            remove: remove(),
            push: push(),
            push_many: push_many(),
            pop: pop(),
            clear: clear(),
        }
    }
}

impl Prelude for ListPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.length.put(m);
        self.set.put(m);
        self.set_many.put(m);
        self.get.put(m);
        self.insert.put(m);
        self.insert_many.put(m);
        self.remove.put(m);
        self.push.put(m);
        self.push_many.put(m);
        self.pop.put(m);
        self.clear.put(m);
    }
}

fn length() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("list.length", input_mode, output_mode, true, fn_length)
}

fn fn_length(ctx: ConstFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_lossless(ctx, input, |val| {
        let Val::List(list) = val else {
            return Val::default();
        };
        let len: Int = list.len().into();
        Val::Int(len.into())
    })
}

fn set() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.set", input_mode, output_mode, true, fn_set)
}

fn fn_set(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let list_pair = Pair::from(list_pair);
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = list_pair.first;
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let mut value = index_value.second;
    DefaultCtx.with_ref_mut_lossless(ctx, name, |val| {
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

fn set_many() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.set_many", input_mode, output_mode, true, fn_set_many)
}

fn fn_set_many(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(list_pair) = input else {
        return Val::default();
    };
    let list_pair = Pair::from(list_pair);
    let Val::Pair(index_value) = list_pair.second else {
        return Val::default();
    };
    let name = list_pair.first;
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        return Val::default();
    };
    let values = List::from(values);
    DefaultCtx.with_ref_mut_lossless(ctx, name, |val| {
        let Val::List(list) = val else {
            return Val::default();
        };
        let end = i + values.len();
        if end > list.len() {
            return Val::default();
        }
        let ret: List<Val> = list.splice(i..end, values).collect();
        Val::List(ret.into())
    })
}

fn get() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("list.get", input_mode, output_mode, true, fn_get)
}

fn fn_get(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name_index = Pair::from(name_index);
    let name = name_index.first;
    if let Val::Pair(range) = name_index.second {
        let range = Pair::from(range);
        let Some((from, to)) = to_range(range) else {
            return Val::default();
        };
        DefaultCtx.with_ref_lossless(ctx, name, |val| {
            let Val::List(list) = val else {
                return Val::default();
            };
            let from = from.unwrap_or_default();
            let to = to.unwrap_or(list.len());
            let Some(slice) = list.get(from..to) else {
                return Val::default();
            };
            Val::List(List::from(slice.to_owned()).into())
        })
    } else {
        let Some(i) = to_index(name_index.second) else {
            return Val::default();
        };
        DefaultCtx.with_ref_lossless(ctx, name, |val| {
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

fn insert() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.insert", input_mode, output_mode, true, fn_insert)
}

fn fn_insert(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name_pair = Pair::from(name_pair);
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = name_pair.first;
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let value = index_value.second;
    DefaultCtx.with_ref_mut_no_ret(ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        if i > list.len() {
            return;
        }
        list.insert(i, value);
    })
}

fn insert_many() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn(
        "list.insert_many",
        input_mode,
        output_mode,
        true,
        fn_insert_many,
    )
}

fn fn_insert_many(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_pair) = input else {
        return Val::default();
    };
    let name_pair = Pair::from(name_pair);
    let Val::Pair(index_value) = name_pair.second else {
        return Val::default();
    };
    let name = name_pair.first;
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        return Val::default();
    };
    let values = List::from(values);
    DefaultCtx.with_ref_mut_no_ret(ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        if i > list.len() {
            return;
        }
        list.splice(i..i, values);
    })
}

fn remove() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.remove", input_mode, output_mode, true, fn_remove)
}

fn fn_remove(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_index) = input else {
        return Val::default();
    };
    let name_index = Pair::from(name_index);
    let name = name_index.first;
    if let Val::Pair(range) = name_index.second {
        let range = Pair::from(range);
        let Some((from, to)) = to_range(range) else {
            return Val::default();
        };
        DefaultCtx.with_ref_mut_lossless(ctx, name, |val| {
            let Val::List(list) = val else {
                return Val::default();
            };
            let from = from.unwrap_or_default();
            let to = to.unwrap_or(list.len());
            if from > to || to > list.len() {
                return Val::default();
            }
            let ret: List<Val> = list.splice(from..to, Vec::new()).collect();
            Val::List(ret.into())
        })
    } else {
        let Some(i) = to_index(name_index.second) else {
            return Val::default();
        };
        DefaultCtx.with_ref_mut_lossless(ctx, name, |val| {
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

fn push() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.push", input_mode, output_mode, true, fn_push)
}

fn fn_push(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_value) = input else {
        return Val::default();
    };
    let name_value = Pair::from(name_value);
    let name = name_value.first;
    let value = name_value.second;
    DefaultCtx.with_ref_mut_no_ret(ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        list.push(value);
    })
}

fn push_many() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn(
        "list.push_many",
        input_mode,
        output_mode,
        true,
        fn_push_many,
    )
}

fn fn_push_many(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_values) = input else {
        return Val::default();
    };
    let name_values = Pair::from(name_values);
    let name = name_values.first;
    let values = name_values.second;
    let Val::List(mut values) = values else {
        return Val::default();
    };
    DefaultCtx.with_ref_mut_no_ret(ctx, name, |val| {
        let Val::List(list) = val else {
            return;
        };
        list.append(&mut values);
    })
}

fn pop() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.pop", input_mode, output_mode, true, fn_pop)
}

fn fn_pop(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(name_count) = input else {
        return Val::default();
    };
    let name_count = Pair::from(name_count);
    let name = name_count.first;
    let count = name_count.second;
    match count {
        Val::Unit(_) => DefaultCtx.with_ref_mut_lossless(ctx, name, |val| {
            let Val::List(list) = val else {
                return Val::default();
            };
            list.pop().unwrap_or_default()
        }),
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
                return Val::default();
            };
            DefaultCtx.with_ref_mut_lossless(ctx, name, |val| {
                let Val::List(list) = val else {
                    return Val::default();
                };
                let list = &mut **list;
                if i > list.len() {
                    return Val::default();
                }
                let start = list.len() - i;
                let list = list.split_off(start);
                let list: List<Val> = list.into();
                Val::List(list.into())
            })
        }
        _ => Val::default(),
    }
}

fn clear() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("list.clear", input_mode, output_mode, true, fn_clear)
}

fn fn_clear(ctx: MutFnCtx, input: Val) -> Val {
    DefaultCtx.with_ref_mut_no_ret(ctx, input, |val| {
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

fn to_range(pair: Pair<Val, Val>) -> Option<(Option<usize>, Option<usize>)> {
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
