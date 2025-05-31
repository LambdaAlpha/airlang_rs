use std::mem::swap;

use crate::ConstRef;
use crate::FuncMode;
use crate::Int;
use crate::Pair;
use crate::list::List;
use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::const_impl;
use crate::prelude::ctx_default_mode;
use crate::prelude::mut_impl;
use crate::prelude::named_const_fn;
use crate::prelude::named_mut_fn;
use crate::val::Val;
use crate::val::func::FuncVal;

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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.length.put(ctx);
        self.set.put(ctx);
        self.set_many.put(ctx);
        self.get.put(ctx);
        self.insert.put(ctx);
        self.insert_many.put(ctx);
        self.remove.put(ctx);
        self.push.put(ctx);
        self.push_many.put(ctx);
        self.pop.put(ctx);
        self.clear.put(ctx);
    }
}

fn length() -> Named<FuncVal> {
    let id = "list.length";
    let f = const_impl(fn_length);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        return Val::default();
    };
    let len: Int = list.len().into();
    Val::Int(len.into())
}

fn set() -> Named<FuncVal> {
    let id = "list.set";
    let f = mut_impl(fn_set);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_set(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let mut value = index_value.second;
    let Some(current) = list.get_mut(i) else {
        return Val::default();
    };
    swap(current, &mut value);
    value
}

fn set_many() -> Named<FuncVal> {
    let id = "list.set_many";
    let f = mut_impl(fn_set_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_set_many(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        return Val::default();
    };
    let values = List::from(values);
    let end = i + values.len();
    if end > list.len() {
        return Val::default();
    }
    let ret: List<Val> = list.splice(i .. end, values).collect();
    Val::List(ret.into())
}

fn get() -> Named<FuncVal> {
    let id = "list.get";
    let f = const_impl(fn_get);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_const_fn(id, f, mode, ctx_explicit)
}

fn fn_get(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        return Val::default();
    };
    if let Val::Pair(range) = input {
        let range = Pair::from(range);
        let Some((from, to)) = to_range(range) else {
            return Val::default();
        };
        let from = from.unwrap_or_default();
        let to = to.unwrap_or(list.len());
        let Some(slice) = list.get(from .. to) else {
            return Val::default();
        };
        Val::List(List::from(slice.to_owned()).into())
    } else {
        let Some(i) = to_index(input) else {
            return Val::default();
        };
        let Some(val) = list.get(i) else {
            return Val::default();
        };
        val.clone()
    }
}

fn insert() -> Named<FuncVal> {
    let id = "list.insert";
    let f = mut_impl(fn_insert);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_insert(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let value = index_value.second;
    if i > list.len() {
        return Val::default();
    }
    list.insert(i, value);
    Val::default()
}

fn insert_many() -> Named<FuncVal> {
    let id = "list.insert_many";
    let f = mut_impl(fn_insert_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_insert_many(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        return Val::default();
    };
    let values = List::from(values);
    if i > list.len() {
        return Val::default();
    }
    list.splice(i .. i, values);
    Val::default()
}

fn remove() -> Named<FuncVal> {
    let id = "list.remove";
    let f = mut_impl(fn_remove);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_remove(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    if let Val::Pair(range) = input {
        let range = Pair::from(range);
        let Some((from, to)) = to_range(range) else {
            return Val::default();
        };
        let from = from.unwrap_or_default();
        let to = to.unwrap_or(list.len());
        if from > to || to > list.len() {
            return Val::default();
        }
        let ret: List<Val> = list.splice(from .. to, Vec::new()).collect();
        Val::List(ret.into())
    } else {
        let Some(i) = to_index(input) else {
            return Val::default();
        };
        if i >= list.len() {
            return Val::default();
        }
        list.remove(i)
    }
}

fn push() -> Named<FuncVal> {
    let id = "list.push";
    let f = mut_impl(fn_push);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_push(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    list.push(input);
    Val::default()
}

fn push_many() -> Named<FuncVal> {
    let id = "list.push_many";
    let f = mut_impl(fn_push_many);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_push_many(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    let Val::List(mut values) = input else {
        return Val::default();
    };
    list.append(&mut values);
    Val::default()
}

fn pop() -> Named<FuncVal> {
    let id = "list.pop";
    let f = mut_impl(fn_pop);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_pop(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    match input {
        Val::Unit(_) => list.pop().unwrap_or_default(),
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
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
        }
        _ => Val::default(),
    }
}

fn clear() -> Named<FuncVal> {
    let id = "list.clear";
    let f = mut_impl(fn_clear);
    let forward = ctx_default_mode();
    let reverse = FuncMode::default_mode();
    let mode = FuncMode { forward, reverse };
    let ctx_explicit = true;
    named_mut_fn(id, f, mode, ctx_explicit)
}

fn fn_clear(ctx: &mut Val, _input: Val) -> Val {
    let Val::List(list) = ctx else {
        return Val::default();
    };
    list.clear();
    Val::default()
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
