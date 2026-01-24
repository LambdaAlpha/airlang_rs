use std::mem::swap;

use const_format::concatcp;
use log::error;
use num_traits::ToPrimitive;

use super::ConstImpl;
use super::MutImpl;
use super::abort_const;
use super::abort_free;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::LIST;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct ListLib {
    pub get_length: ConstPrimFuncVal,
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

pub const GET_LENGTH: &str = concatcp!(PREFIX_ID, LIST, ".get_length");
pub const SET: &str = concatcp!(PREFIX_ID, LIST, ".set");
pub const SET_MANY: &str = concatcp!(PREFIX_ID, LIST, ".set_many");
pub const GET: &str = concatcp!(PREFIX_ID, LIST, ".get");
pub const GET_MANY: &str = concatcp!(PREFIX_ID, LIST, ".get_many");
pub const INSERT: &str = concatcp!(PREFIX_ID, LIST, ".insert");
pub const INSERT_MANY: &str = concatcp!(PREFIX_ID, LIST, ".insert_many");
pub const REMOVE: &str = concatcp!(PREFIX_ID, LIST, ".remove");
pub const REMOVE_MANY: &str = concatcp!(PREFIX_ID, LIST, ".remove_many");
pub const PUSH: &str = concatcp!(PREFIX_ID, LIST, ".push");
pub const PUSH_MANY: &str = concatcp!(PREFIX_ID, LIST, ".push_many");
pub const POP: &str = concatcp!(PREFIX_ID, LIST, ".pop");
pub const POP_MANY: &str = concatcp!(PREFIX_ID, LIST, ".pop_many");
pub const CLEAR: &str = concatcp!(PREFIX_ID, LIST, ".clear");

impl Default for ListLib {
    fn default() -> Self {
        ListLib {
            get_length: get_length(),
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
        extend_func(cfg, GET_LENGTH, self.get_length);
        extend_func(cfg, SET, self.set);
        extend_func(cfg, SET_MANY, self.set_many);
        extend_func(cfg, GET, self.get);
        extend_func(cfg, GET_MANY, self.get_many);
        extend_func(cfg, INSERT, self.insert);
        extend_func(cfg, INSERT_MANY, self.insert_many);
        extend_func(cfg, REMOVE, self.remove);
        extend_func(cfg, REMOVE_MANY, self.remove_many);
        extend_func(cfg, PUSH, self.push);
        extend_func(cfg, PUSH_MANY, self.push_many);
        extend_func(cfg, POP, self.pop);
        extend_func(cfg, POP_MANY, self.pop_many);
        extend_func(cfg, CLEAR, self.clear);
    }
}

pub fn get_length() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_LENGTH), const_: fn_get_length }.build()
}

fn fn_get_length(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let len: Int = list.len().into();
    Val::Int(len.into())
}

pub fn set() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET), const_: abort_const(SET), mut_: fn_set }.build()
}

fn fn_set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(index) else {
        error!("input.left should be a valid index");
        return illegal_input(cfg);
    };
    let mut value = index_value.right;
    let Some(current) = list.get_mut(i) else {
        error!("index {i:?} should < list.len {}", list.len());
        return illegal_input(cfg);
    };
    swap(current, &mut value);
    value
}

pub fn set_many() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET_MANY), const_: abort_const(SET_MANY), mut_: fn_set_many }.build()
}

fn fn_set_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(index) else {
        error!("input.left should be a valid index");
        return illegal_input(cfg);
    };
    let Val::List(values) = index_value.right else {
        error!("input.right {:?} should be a list", index_value.right);
        return illegal_input(cfg);
    };
    let values = List::from(values);
    let end = i + values.len();
    if end > list.len() {
        error!("end {end} should <= list.len {}", list.len());
        return illegal_input(cfg);
    }
    let ret: List<Val> = list.splice(i .. end, values).collect();
    Val::List(ret.into())
}

pub fn get() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET), const_: fn_get }.build()
}

fn fn_get(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Some(i) = to_index(input) else {
        error!("input should be a valid index");
        return illegal_input(cfg);
    };
    let Some(val) = list.get(i) else {
        error!("index {i} should < list.len {}", list.len());
        return illegal_input(cfg);
    };
    val.clone()
}

pub fn get_many() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_MANY), const_: fn_get_many }.build()
}

fn fn_get_many(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::List(list) = &*ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Pair(range) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let range = Pair::from(range);
    let Some((from, to)) = to_range(range) else {
        error!("input should be a valid range");
        return illegal_input(cfg);
    };
    let from = from.unwrap_or_default();
    let to = to.unwrap_or(list.len());
    let Some(slice) = list.get(from .. to) else {
        error!("range {from} : {to} should be in 0 : {}", list.len());
        return illegal_input(cfg);
    };
    Val::List(List::from(slice.to_owned()).into())
}

pub fn insert() -> MutPrimFuncVal {
    MutImpl { free: abort_free(INSERT), const_: abort_const(INSERT), mut_: fn_insert }.build()
}

fn fn_insert(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(index) else {
        error!("input.left should be a valid index");
        return illegal_input(cfg);
    };
    let value = index_value.right;
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return illegal_input(cfg);
    }
    list.insert(i, value);
    Val::default()
}

pub fn insert_many() -> MutPrimFuncVal {
    MutImpl {
        free: abort_free(INSERT_MANY),
        const_: abort_const(INSERT_MANY),
        mut_: fn_insert_many,
    }
    .build()
}

fn fn_insert_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Pair(index_value) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(index) else {
        error!("input.left should be a valid index");
        return illegal_input(cfg);
    };
    let Val::List(values) = index_value.right else {
        error!("input.right {:?} should be a list", index_value.right);
        return illegal_input(cfg);
    };
    let values = List::from(values);
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return illegal_input(cfg);
    }
    list.splice(i .. i, values);
    Val::default()
}

pub fn remove() -> MutPrimFuncVal {
    MutImpl { free: abort_free(REMOVE), const_: abort_const(REMOVE), mut_: fn_remove }.build()
}

fn fn_remove(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Some(i) = to_index(input) else {
        error!("input should be a valid index");
        return illegal_input(cfg);
    };
    if i >= list.len() {
        error!("index {i} should < list.len {}", list.len());
        return illegal_input(cfg);
    }
    list.remove(i)
}

pub fn remove_many() -> MutPrimFuncVal {
    MutImpl {
        free: abort_free(REMOVE_MANY),
        const_: abort_const(REMOVE_MANY),
        mut_: fn_remove_many,
    }
    .build()
}

fn fn_remove_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Pair(range) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let range = Pair::from(range);
    let Some((from, to)) = to_range(range) else {
        error!("input should be a valid range");
        return illegal_input(cfg);
    };
    let from = from.unwrap_or_default();
    let to = to.unwrap_or(list.len());
    if from > to || to > list.len() {
        error!("range {from} : {to} should be in 0 : {}", list.len());
        return illegal_input(cfg);
    }
    let ret: List<Val> = list.splice(from .. to, Vec::new()).collect();
    Val::List(ret.into())
}

pub fn push() -> MutPrimFuncVal {
    MutImpl { free: abort_free(PUSH), const_: abort_const(PUSH), mut_: fn_push }.build()
}

fn fn_push(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    list.push(input);
    Val::default()
}

pub fn push_many() -> MutPrimFuncVal {
    MutImpl { free: abort_free(PUSH_MANY), const_: abort_const(PUSH_MANY), mut_: fn_push_many }
        .build()
}

fn fn_push_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::List(mut values) = input else {
        error!("input {input:?} should be a list");
        return illegal_input(cfg);
    };
    list.append(&mut values);
    Val::default()
}

pub fn pop() -> MutPrimFuncVal {
    MutImpl { free: abort_free(POP), const_: abort_const(POP), mut_: fn_pop }.build()
}

fn fn_pop(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Unit(_) = input else {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    };
    let Some(val) = list.pop() else {
        error!("list should be non-empty");
        return abort_bug_with_msg(cfg, "_list.pop list should be non-empty");
    };
    val
}

pub fn pop_many() -> MutPrimFuncVal {
    MutImpl { free: abort_free(POP_MANY), const_: abort_const(POP_MANY), mut_: fn_pop_many }.build()
}

fn fn_pop_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    let Val::Int(i) = input else {
        error!("input {input:?} should be an int");
        return illegal_input(cfg);
    };
    let Some(i) = i.to_usize() else {
        error!("index {i:?} should <= list.len {}", list.len());
        return illegal_input(cfg);
    };
    let list = &mut **list;
    if i > list.len() {
        error!("index {i} should <= list.len {}", list.len());
        return illegal_input(cfg);
    }
    let start = list.len() - i;
    let list = list.split_off(start);
    let list: List<Val> = list.into();
    Val::List(list.into())
}

pub fn clear() -> MutPrimFuncVal {
    MutImpl { free: abort_free(CLEAR), const_: abort_const(CLEAR), mut_: fn_clear }.build()
}

fn fn_clear(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        error!("ctx {ctx:?} should be a list");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
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
    let from = match pair.left {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        v => {
            error!("from {v:?} should be an int or a unit");
            return None;
        }
    };
    let to = match pair.right {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        v => {
            error!("to {v:?} should be an int or a unit");
            return None;
        }
    };
    Some((from, to))
}
