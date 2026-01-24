use std::mem::swap;

use const_format::concatcp;
use log::error;

use super::DynPrimFn;
use super::const_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct PairLib {
    pub get_left: ConstPrimFuncVal,
    pub set_left: MutPrimFuncVal,
    pub get_right: ConstPrimFuncVal,
    pub set_right: MutPrimFuncVal,
}

pub const GET_LEFT: &str = concatcp!(PREFIX_ID, PAIR, ".get_left");
pub const SET_LEFT: &str = concatcp!(PREFIX_ID, PAIR, ".set_left");
pub const GET_RIGHT: &str = concatcp!(PREFIX_ID, PAIR, ".get_right");
pub const SET_RIGHT: &str = concatcp!(PREFIX_ID, PAIR, ".set_right");

impl Default for PairLib {
    fn default() -> Self {
        PairLib {
            get_left: get_left(),
            set_left: set_left(),
            get_right: get_right(),
            set_right: set_right(),
        }
    }
}

impl CfgMod for PairLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, GET_LEFT, self.get_left);
        extend_func(cfg, SET_LEFT, self.set_left);
        extend_func(cfg, GET_RIGHT, self.get_right);
        extend_func(cfg, SET_RIGHT, self.set_right);
    }
}

pub fn get_left() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_left) }.const_()
}

fn fn_get_left(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    pair.left.clone()
}

pub fn set_left() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_left) }.mut_()
}

fn fn_set_left(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    swap(&mut pair.left, &mut input);
    input
}

pub fn get_right() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_right) }.const_()
}

fn fn_get_right(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    pair.right.clone()
}

pub fn set_right() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_right) }.mut_()
}

fn fn_set_right(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    swap(&mut pair.right, &mut input);
    input
}
