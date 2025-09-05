use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::mode::default_dyn_mode;
use crate::cfg::mode::default_free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct PairLib {
    pub new: FreePrimFuncVal,
    pub first: ConstPrimFuncVal,
    pub set_first: MutPrimFuncVal,
    pub second: ConstPrimFuncVal,
    pub set_second: MutPrimFuncVal,
}

impl Default for PairLib {
    fn default() -> Self {
        PairLib {
            new: new(),
            first: first(),
            set_first: set_first(),
            second: second(),
            set_second: set_second(),
        }
    }
}

impl CfgMod for PairLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.first.extend(cfg);
        self.set_first.extend(cfg);
        self.second.extend(cfg);
        self.set_second.extend(cfg);
    }
}

impl Library for PairLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "pair.new", f: free_impl(fn_new), mode: default_free_mode() }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(_) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    input
}

pub fn first() -> ConstPrimFuncVal {
    DynPrimFn { id: "pair.first", f: const_impl(fn_first), mode: default_dyn_mode() }.const_()
}

fn fn_first(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    pair.first.clone()
}

pub fn set_first() -> MutPrimFuncVal {
    DynPrimFn { id: "pair.set_first", f: mut_impl(fn_set_first), mode: default_dyn_mode() }.mut_()
}

fn fn_set_first(_cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    swap(&mut pair.first, &mut input);
    input
}

pub fn second() -> ConstPrimFuncVal {
    DynPrimFn { id: "pair.second", f: const_impl(fn_second), mode: default_dyn_mode() }.const_()
}

fn fn_second(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    pair.second.clone()
}

pub fn set_second() -> MutPrimFuncVal {
    DynPrimFn { id: "pair.set_second", f: mut_impl(fn_set_second), mode: default_dyn_mode() }.mut_()
}

fn fn_set_second(_cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    swap(&mut pair.second, &mut input);
    input
}
