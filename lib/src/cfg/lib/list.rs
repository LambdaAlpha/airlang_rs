use std::mem::swap;

use log::error;
use num_traits::ToPrimitive;

use super::DynPrimFn;
use super::const_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::fail;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct ListLib {
    pub length: ConstPrimFuncVal,
    pub set: MutPrimFuncVal,
    pub set_many: MutPrimFuncVal,
    pub get: ConstPrimFuncVal,
    pub get_many: ConstPrimFuncVal,
    pub insert: MutPrimFuncVal,
    pub insert_many: MutPrimFuncVal,
    pub remove: MutPrimFuncVal,
    pub remove_many: MutPrimFuncVal,
    pub push: MutPrimFuncVal,
    pub push_many: MutPrimFuncVal,
    pub pop: MutPrimFuncVal,
    pub pop_many: MutPrimFuncVal,
    pub clear: MutPrimFuncVal,
}

impl Default for ListLib {
    fn default() -> Self {
        ListLib {
            length: length(),
            set: set(),
            set_many: set_many(),
            get: get(),
            get_many: get_many(),
            insert: insert(),
            insert_many: insert_many(),
            remove: remove(),
            remove_many: remove_many(),
            push: push(),
            push_many: push_many(),
            pop: pop(),
            pop_many: pop_many(),
            clear: clear(),
        }
    }
}

impl CfgMod for ListLib {
    fn extend(self, cfg: &Cfg) {
        self.length.extend(cfg);
        self.set.extend(cfg);
        self.set_many.extend(cfg);
        self.get.extend(cfg);
        self.get_many.extend(cfg);
        self.insert.extend(cfg);
        self.insert_many.extend(cfg);
        self.remove.extend(cfg);
        self.remove_many.extend(cfg);
        self.push.extend(cfg);
        self.push_many.extend(cfg);
        self.pop.extend(cfg);
        self.pop_many.extend(cfg);
        self.clear.extend(cfg);
    }
}

pub fn length() -> ConstPrimFuncVal {
    DynPrimFn { id: "list.length", f: const_impl(fn_length) }.const_()
}

fn fn_length(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let len: Int = list.len().into();
    Val::Int(len.into())
}

pub fn set() -> MutPrimFuncVal {
    DynPrimFn { id: "list.set", f: mut_impl(fn_set) }.mut_()
}

fn fn_set(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return illegal_input();
    };
    let mut value = index_value.second;
    let Some(current) = list.get_mut(i) else {
        error!("index {i:?} should < list.len {}", list.len());
        return fail();
    };
    swap(current, &mut value);
    value
}

pub fn set_many() -> MutPrimFuncVal {
    DynPrimFn { id: "list.set_many", f: mut_impl(fn_set_many) }.mut_()
}

fn fn_set_many(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return illegal_input();
    };
    let Val::List(values) = index_value.second else {
        error!("input.second {:?} should be a list", index_value.second);
        return illegal_input();
    };
    let values = List::from(values);
    let end = i + values.len();
    if end > list.len() {
        error!("end {end} should <= list.len {}", list.len());
        return fail();
    }
    let ret: List<Val> = list.splice(i .. end, values).collect();
    Val::List(ret.into())
}

pub fn get() -> ConstPrimFuncVal {
    DynPrimFn { id: "list.get", f: const_impl(fn_get) }.const_()
}

fn fn_get(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Some(i) = to_index(input) else {
        error!("input should be a valid index");
        return illegal_input();
    };
    let Some(val) = list.get(i) else {
        error!("index {i} should < list.len {}", list.len());
        return fail();
    };
    val.clone()
}

pub fn get_many() -> ConstPrimFuncVal {
    DynPrimFn { id: "list.get_many", f: const_impl(fn_get_many) }.const_()
}

fn fn_get_many(_cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Pair(range) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input();
    };
    let range = Pair::from(range);
    let Some((from, to)) = to_range(range) else {
        error!("input should be a valid range");
        return illegal_input();
    };
    let from = from.unwrap_or_default();
    let to = to.unwrap_or(list.len());
    let Some(slice) = list.get(from .. to) else {
        error!("range {from} : {to} should be in 0 : {}", list.len());
        return fail();
    };
    Val::List(List::from(slice.to_owned()).into())
}

pub fn insert() -> MutPrimFuncVal {
    DynPrimFn { id: "list.insert", f: mut_impl(fn_insert) }.mut_()
}

fn fn_insert(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return illegal_input();
    };
    let value = index_value.second;
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return fail();
    }
    list.insert(i, value);
    Val::default()
}

pub fn insert_many() -> MutPrimFuncVal {
    DynPrimFn { id: "list.insert_many", f: mut_impl(fn_insert_many) }.mut_()
}

fn fn_insert_many(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input();
    };
    let index_value = Pair::from(index_value);
    let index = index_value.first;
    let Some(i) = to_index(index) else {
        error!("input.first should be a valid index");
        return illegal_input();
    };
    let Val::List(values) = index_value.second else {
        error!("input.second {:?} should be a list", index_value.second);
        return illegal_input();
    };
    let values = List::from(values);
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return fail();
    }
    list.splice(i .. i, values);
    Val::default()
}

pub fn remove() -> MutPrimFuncVal {
    DynPrimFn { id: "list.remove", f: mut_impl(fn_remove) }.mut_()
}

fn fn_remove(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Some(i) = to_index(input) else {
        error!("input should be a valid index");
        return illegal_input();
    };
    if i >= list.len() {
        error!("index {i} should < list.len {}", list.len());
        return fail();
    }
    list.remove(i)
}

pub fn remove_many() -> MutPrimFuncVal {
    DynPrimFn { id: "list.remove_many", f: mut_impl(fn_remove_many) }.mut_()
}

fn fn_remove_many(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Pair(range) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input();
    };
    let range = Pair::from(range);
    let Some((from, to)) = to_range(range) else {
        error!("input should be a valid range");
        return illegal_input();
    };
    let from = from.unwrap_or_default();
    let to = to.unwrap_or(list.len());
    if from > to || to > list.len() {
        error!("range {from} : {to} should be in 0 : {}", list.len());
        return fail();
    }
    let ret: List<Val> = list.splice(from .. to, Vec::new()).collect();
    Val::List(ret.into())
}

pub fn push() -> MutPrimFuncVal {
    DynPrimFn { id: "list.push", f: mut_impl(fn_push) }.mut_()
}

fn fn_push(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    list.push(input);
    Val::default()
}

pub fn push_many() -> MutPrimFuncVal {
    DynPrimFn { id: "list.push_many", f: mut_impl(fn_push_many) }.mut_()
}

fn fn_push_many(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::List(mut values) = input else {
        error!("input {input:?} should be a list");
        return illegal_input();
    };
    list.append(&mut values);
    Val::default()
}

pub fn pop() -> MutPrimFuncVal {
    DynPrimFn { id: "list.pop", f: mut_impl(fn_pop) }.mut_()
}

fn fn_pop(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Unit(_) = input else {
        error!("input {input:?} should be a unit");
        return illegal_input();
    };
    list.pop().unwrap_or_default()
}

pub fn pop_many() -> MutPrimFuncVal {
    DynPrimFn { id: "list.pop_many", f: mut_impl(fn_pop_many) }.mut_()
}

fn fn_pop_many(_cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
    };
    let Val::Int(i) = input else {
        error!("input {input:?} should be an int");
        return illegal_input();
    };
    let Some(i) = i.to_usize() else {
        error!("index {i:?} should <= list.len {}", list.len());
        return fail();
    };
    let list = &mut **list;
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return fail();
    }
    let start = list.len() - i;
    let list = list.split_off(start);
    let list: List<Val> = list.into();
    Val::List(list.into())
}

pub fn clear() -> MutPrimFuncVal {
    DynPrimFn { id: "list.clear", f: mut_impl(fn_clear) }.mut_()
}

fn fn_clear(_cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx();
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
