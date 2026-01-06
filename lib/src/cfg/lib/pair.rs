use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::const_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct PairLib {
    pub get_first: ConstPrimFuncVal,
    pub set_first: MutPrimFuncVal,
    pub get_second: ConstPrimFuncVal,
    pub set_second: MutPrimFuncVal,
}

impl Default for PairLib {
    fn default() -> Self {
        PairLib {
            get_first: get_first(),
            set_first: set_first(),
            get_second: get_second(),
            set_second: set_second(),
        }
    }
}

impl CfgMod for PairLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_pair.get_first", self.get_first);
        extend_func(cfg, "_pair.set_first", self.set_first);
        extend_func(cfg, "_pair.get_second", self.get_second);
        extend_func(cfg, "_pair.set_second", self.set_second);
    }
}

pub fn get_first() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_first) }.const_()
}

fn fn_get_first(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    pair.first.clone()
}

pub fn set_first() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_first) }.mut_()
}

fn fn_set_first(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    swap(&mut pair.first, &mut input);
    input
}

pub fn get_second() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_second) }.const_()
}

fn fn_get_second(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    pair.second.clone()
}

pub fn set_second() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_second) }.mut_()
}

fn fn_set_second(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return illegal_ctx(cfg);
    };
    swap(&mut pair.second, &mut input);
    input
}
