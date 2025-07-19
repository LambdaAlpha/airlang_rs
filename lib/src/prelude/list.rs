use std::mem::swap;

use log::error;
use num_traits::ToPrimitive;

use super::DynFn;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::mut_impl;
use super::setup::default_dyn_mode;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct ListPrelude {
    pub length: ConstStaticPrimFuncVal,
    pub set: MutStaticPrimFuncVal,
    pub set_many: MutStaticPrimFuncVal,
    pub get: ConstStaticPrimFuncVal,
    pub insert: MutStaticPrimFuncVal,
    pub insert_many: MutStaticPrimFuncVal,
    pub remove: MutStaticPrimFuncVal,
    pub push: MutStaticPrimFuncVal,
    pub push_many: MutStaticPrimFuncVal,
    pub pop: MutStaticPrimFuncVal,
    pub clear: MutStaticPrimFuncVal,
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

pub fn length() -> ConstStaticPrimFuncVal {
    DynFn { id: "list.length", f: const_impl(fn_length), mode: default_dyn_mode() }.const_static()
}

fn fn_length(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    let len: Int = list.len().into();
    Val::Int(len.into())
}

pub fn set() -> MutStaticPrimFuncVal {
    DynFn { id: "list.set", f: mut_impl(fn_set), mode: default_dyn_mode() }.mut_static()
}

fn fn_set(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return Val::default();
    };
    let mut value = index_value.second;
    let Some(current) = list.get_mut(i) else {
        error!("index {i:?} should < list.len {}", list.len());
        return Val::default();
    };
    swap(current, &mut value);
    value
}

pub fn set_many() -> MutStaticPrimFuncVal {
    DynFn { id: "list.set_many", f: mut_impl(fn_set_many), mode: default_dyn_mode() }.mut_static()
}

fn fn_set_many(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        error!("input.second {:?} should be a list", index_value.second);
        return Val::default();
    };
    let values = List::from(values);
    let end = i + values.len();
    if end > list.len() {
        error!("end {end} should <= list.len {}", list.len());
        return Val::default();
    }
    let ret: List<Val> = list.splice(i .. end, values).collect();
    Val::List(ret.into())
}

pub fn get() -> ConstStaticPrimFuncVal {
    DynFn { id: "list.get", f: const_impl(fn_get), mode: default_dyn_mode() }.const_static()
}

fn fn_get(ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    if let Val::Pair(range) = input {
        let range = Pair::from(range);
        let Some((from, to)) = to_range(range) else {
            error!("input should be a valid range");
            return Val::default();
        };
        let from = from.unwrap_or_default();
        let to = to.unwrap_or(list.len());
        let Some(slice) = list.get(from .. to) else {
            error!("range {from} : {to} should be in 0 : {}", list.len());
            return Val::default();
        };
        Val::List(List::from(slice.to_owned()).into())
    } else {
        let Some(i) = to_index(input) else {
            error!("input should be a valid index");
            return Val::default();
        };
        let Some(val) = list.get(i) else {
            error!("index {i} should < list.len {}", list.len());
            return Val::default();
        };
        val.clone()
    }
}

pub fn insert() -> MutStaticPrimFuncVal {
    DynFn { id: "list.insert", f: mut_impl(fn_insert), mode: default_dyn_mode() }.mut_static()
}

fn fn_insert(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return Val::default();
    };
    let value = index_value.second;
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return Val::default();
    }
    list.insert(i, value);
    Val::default()
}

pub fn insert_many() -> MutStaticPrimFuncVal {
    DynFn { id: "list.insert_many", f: mut_impl(fn_insert_many), mode: default_dyn_mode() }
        .mut_static()
}

fn fn_insert_many(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return Val::default();
    };
    let Val::List(values) = index_value.second else {
        error!("input.second {:?} should be a list", index_value.second);
        return Val::default();
    };
    let values = List::from(values);
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return Val::default();
    }
    list.splice(i .. i, values);
    Val::default()
}

pub fn remove() -> MutStaticPrimFuncVal {
    DynFn { id: "list.remove", f: mut_impl(fn_remove), mode: default_dyn_mode() }.mut_static()
}

fn fn_remove(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    if let Val::Pair(range) = input {
        let range = Pair::from(range);
        let Some((from, to)) = to_range(range) else {
            error!("input should be a valid range");
            return Val::default();
        };
        let from = from.unwrap_or_default();
        let to = to.unwrap_or(list.len());
        if from > to || to > list.len() {
            error!("range {from} : {to} should be in 0 : {}", list.len());
            return Val::default();
        }
        let ret: List<Val> = list.splice(from .. to, Vec::new()).collect();
        Val::List(ret.into())
    } else {
        let Some(i) = to_index(input) else {
            error!("input should be a valid index");
            return Val::default();
        };
        if i >= list.len() {
            error!("index {i} should < list.len {}", list.len());
            return Val::default();
        }
        list.remove(i)
    }
}

pub fn push() -> MutStaticPrimFuncVal {
    DynFn { id: "list.push", f: mut_impl(fn_push), mode: default_dyn_mode() }.mut_static()
}

fn fn_push(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    list.push(input);
    Val::default()
}

pub fn push_many() -> MutStaticPrimFuncVal {
    DynFn { id: "list.push_many", f: mut_impl(fn_push_many), mode: default_dyn_mode() }.mut_static()
}

fn fn_push_many(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    let Val::List(mut values) = input else {
        error!("input {input:?} should be a list");
        return Val::default();
    };
    list.append(&mut values);
    Val::default()
}

pub fn pop() -> MutStaticPrimFuncVal {
    DynFn { id: "list.pop", f: mut_impl(fn_pop), mode: default_dyn_mode() }.mut_static()
}

fn fn_pop(ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    match input {
        Val::Unit(_) => list.pop().unwrap_or_default(),
        Val::Int(i) => {
            let Some(i) = i.to_usize() else {
                error!("index {i:?} should <= list.len {}", list.len());
                return Val::default();
            };
            let list = &mut **list;
            if i > list.len() {
                error!("index {i} should <= list.len {}", list.len());
                return Val::default();
            }
            let start = list.len() - i;
            let list = list.split_off(start);
            let list: List<Val> = list.into();
            Val::List(list.into())
        }
        _ => {
            error!("input should be an int or a unit");
            Val::default()
        }
    }
}

pub fn clear() -> MutStaticPrimFuncVal {
    DynFn { id: "list.clear", f: mut_impl(fn_clear), mode: default_dyn_mode() }.mut_static()
}

fn fn_clear(ctx: &mut Val, _input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return Val::default();
    };
    list.clear();
    Val::default()
}

fn to_index(val: Val) -> Option<usize> {
    let Val::Int(i) = val else {
        error!("index {val:?} should be a int");
        return None;
    };
    i.to_usize()
}

fn to_range(pair: Pair<Val, Val>) -> Option<(Option<usize>, Option<usize>)> {
    let from = match pair.first {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        v => {
            error!("from {v:?} should be an int or a unit");
            return None;
        }
    };
    let to = match pair.second {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        v => {
            error!("to {v:?} should be an int or a unit");
            return None;
        }
    };
    Some((from, to))
}
