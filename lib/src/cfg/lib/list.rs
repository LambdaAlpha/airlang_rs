use std::mem::swap;

use const_format::concatcp;
use num_traits::ToPrimitive;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputEvalFunc;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxMutInputEvalFunc;
use crate::semantics::func::CtxMutInputFreeFunc;
use crate::semantics::val::LIST;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct ListLib {
    pub get_length: PrimFuncVal,
    pub set: PrimFuncVal,
    pub set_many: PrimFuncVal,
    pub get: PrimFuncVal,
    pub get_many: PrimFuncVal,
    pub insert: PrimFuncVal,
    pub insert_many: PrimFuncVal,
    pub remove: PrimFuncVal,
    pub remove_many: PrimFuncVal,
    pub push: PrimFuncVal,
    pub push_many: PrimFuncVal,
    pub pop: PrimFuncVal,
    pub pop_many: PrimFuncVal,
    pub clear: PrimFuncVal,
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

pub fn get_length() -> PrimFuncVal {
    CtxConstInputFreeFunc { fn_: fn_get_length }.build()
}

fn fn_get_length(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{GET_LENGTH}: expected context to be a list, but got {ctx}");
    };
    let len: Int = list.len().into();
    Val::Int(len.into())
}

pub fn set() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_set }.build()
}

fn fn_set(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{SET}: expected context to be a list, but got {ctx}");
    };
    let Val::Pair(index_value) = input else {
        return bug!(cfg, "{SET}: expected input to be a pair, but got {input}");
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(cfg, SET, index) else {
        return Val::default();
    };
    let mut value = index_value.right;
    let Some(current) = list.get_mut(i) else {
        return bug!(cfg, "{SET}: index {i} should < list.len {}", list.len());
    };
    swap(current, &mut value);
    value
}

pub fn set_many() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_set_many }.build()
}

fn fn_set_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{SET_MANY}: expected context to be a list, but got {ctx}");
    };
    let Val::Pair(index_value) = input else {
        return bug!(cfg, "{SET_MANY}: expected input to be a pair, but got {input}");
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(cfg, SET_MANY, index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.right else {
        return bug!(
            cfg,
            "{SET_MANY}: expected input.right to be a list, but got {}",
            index_value.right
        );
    };
    let values = List::from(values);
    let end = i + values.len();
    if end > list.len() {
        return bug!(cfg, "{SET_MANY}: end {end} should <= list.len {}", list.len());
    }
    let ret: List<Val> = list.splice(i .. end, values).collect();
    Val::List(ret.into())
}

pub fn get() -> PrimFuncVal {
    CtxConstInputEvalFunc { fn_: fn_get }.build()
}

fn fn_get(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{GET}: expected context to be a list, but got {ctx}");
    };
    let Some(i) = to_index(cfg, GET, input) else {
        return Val::default();
    };
    let Some(val) = list.get(i) else {
        return bug!(cfg, "{GET}: index {i} should < list.len {}", list.len());
    };
    val.clone()
}

pub fn get_many() -> PrimFuncVal {
    CtxConstInputEvalFunc { fn_: fn_get_many }.build()
}

fn fn_get_many(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{GET_MANY}: expected context to be a list, but got {ctx}");
    };
    let Val::Pair(range) = input else {
        return bug!(cfg, "{GET_MANY}: expected input to be a pair, but got {input}");
    };
    let range = Pair::from(range);
    let Some((from, to)) = to_range(cfg, GET_MANY, range) else {
        return Val::default();
    };
    let from = from.unwrap_or_default();
    let to = to.unwrap_or(list.len());
    let Some(slice) = list.get(from .. to) else {
        return bug!(cfg, "{GET_MANY}: range {from} : {to} should be in 0 : {}", list.len());
    };
    Val::List(List::from(slice.to_owned()).into())
}

pub fn insert() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_insert }.build()
}

fn fn_insert(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{INSERT}: expected context to be a list, but got {ctx}");
    };
    let Val::Pair(index_value) = input else {
        return bug!(cfg, "{INSERT}: expected input to be a pair, but got {input}");
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(cfg, INSERT, index) else {
        return Val::default();
    };
    let value = index_value.right;
    if i > list.len() {
        return bug!(cfg, "{INSERT}: index {i} should <= list.len {}", list.len());
    }
    list.insert(i, value);
    Val::default()
}

pub fn insert_many() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_insert_many }.build()
}

fn fn_insert_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{INSERT_MANY}: expected context to be a list, but got {ctx}");
    };
    let Val::Pair(index_value) = input else {
        return bug!(cfg, "{INSERT_MANY}: expected input to be a pair, but got {input}");
    };
    let index_value = Pair::from(index_value);
    let index = index_value.left;
    let Some(i) = to_index(cfg, INSERT_MANY, index) else {
        return Val::default();
    };
    let Val::List(values) = index_value.right else {
        return bug!(
            cfg,
            "{INSERT_MANY}: expected input.right to be a list, but got {}",
            index_value.right
        );
    };
    let values = List::from(values);
    if i > list.len() {
        return bug!(cfg, "{INSERT_MANY}: index {i} should <= list.len {}", list.len());
    }
    list.splice(i .. i, values);
    Val::default()
}

pub fn remove() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_remove }.build()
}

fn fn_remove(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{REMOVE}: expected context to be a list, but got {ctx}");
    };
    let Some(i) = to_index(cfg, REMOVE, input) else {
        return Val::default();
    };
    if i >= list.len() {
        return bug!(cfg, "{REMOVE}: index {i} should < list.len {}", list.len());
    }
    list.remove(i)
}

pub fn remove_many() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_remove_many }.build()
}

fn fn_remove_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{REMOVE_MANY}: expected context to be a list, but got {ctx}");
    };
    let Val::Pair(range) = input else {
        return bug!(cfg, "{REMOVE_MANY}: expected input to be a pair, but got {input}");
    };
    let range = Pair::from(range);
    let Some((from, to)) = to_range(cfg, REMOVE_MANY, range) else {
        return Val::default();
    };
    let from = from.unwrap_or_default();
    let to = to.unwrap_or(list.len());
    if from > to || to > list.len() {
        return bug!(cfg, "{REMOVE_MANY}: range {from} : {to} should be in 0 : {}", list.len());
    }
    let ret: List<Val> = list.splice(from .. to, Vec::new()).collect();
    Val::List(ret.into())
}

pub fn push() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_push }.build()
}

fn fn_push(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{PUSH}: expected context to be a list, but got {ctx}");
    };
    list.push(input);
    Val::default()
}

pub fn push_many() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_push_many }.build()
}

fn fn_push_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{PUSH_MANY}: expected context to be a list, but got {ctx}");
    };
    let Val::List(mut values) = input else {
        return bug!(cfg, "{PUSH_MANY}: expected input to be a list, but got {input}");
    };
    list.append(&mut values);
    Val::default()
}

pub fn pop() -> PrimFuncVal {
    CtxMutInputFreeFunc { fn_: fn_pop }.build()
}

fn fn_pop(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{POP}: expected context to be a list, but got {ctx}");
    };
    let Some(val) = list.pop() else {
        return bug!(cfg, "{POP}: expected list to be non-empty");
    };
    val
}

pub fn pop_many() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_pop_many }.build()
}

fn fn_pop_many(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{POP_MANY}: expected context to be a list, but got {ctx}");
    };
    let Val::Int(i) = input else {
        return bug!(cfg, "{POP_MANY}: expected input to be an integer, but got {input}");
    };
    let Some(i) = i.to_usize() else {
        return bug!(cfg, "{POP_MANY}: index {i} should <= list.len {}", list.len());
    };
    let list = &mut **list;
    if i > list.len() {
        return bug!(cfg, "{POP_MANY}: index {i} should <= list.len {}", list.len());
    }
    let start = list.len() - i;
    let list = list.split_off(start);
    let list: List<Val> = list.into();
    Val::List(list.into())
}

pub fn clear() -> PrimFuncVal {
    CtxMutInputFreeFunc { fn_: fn_clear }.build()
}

fn fn_clear(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::List(list) = ctx else {
        return bug!(cfg, "{CLEAR}: expected context to be a list, but got {ctx}");
    };
    list.clear();
    Val::default()
}

fn to_index(cfg: &mut Cfg, key: &str, val: Val) -> Option<usize> {
    let Val::Int(i) = val else {
        bug!(cfg, "{key}: expected index to be an integer, but got {val}");
        return None;
    };
    i.to_usize()
}

fn to_range(
    cfg: &mut Cfg, key: &str, pair: Pair<Val, Val>,
) -> Option<(Option<usize>, Option<usize>)> {
    let from = match pair.left {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        v => {
            bug!(cfg, "{key}: expected range.from to be an integer or a unit, but got {v}");
            return None;
        },
    };
    let to = match pair.right {
        Val::Int(i) => Some(i.to_usize()?),
        Val::Unit(_) => None,
        v => {
            bug!(cfg, "{key}: expected range.to to be an integer or a unit, but got {v}");
            return None;
        },
    };
    Some((from, to))
}
