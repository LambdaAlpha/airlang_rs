use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CallLib {
    pub new: FreePrimFuncVal,
    pub get_function: ConstPrimFuncVal,
    pub set_function: MutPrimFuncVal,
    pub get_input: ConstPrimFuncVal,
    pub set_input: MutPrimFuncVal,
}

impl Default for CallLib {
    fn default() -> Self {
        CallLib {
            new: new(),
            get_function: get_function(),
            set_function: set_function(),
            get_input: get_input(),
            set_input: set_input(),
        }
    }
}

impl CfgMod for CallLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_call.new", self.new);
        extend_func(cfg, "_call.get_function", self.get_function);
        extend_func(cfg, "_call.set_function", self.set_function);
        extend_func(cfg, "_call.get_input", self.get_input);
        extend_func(cfg, "_call.set_input", self.set_input);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.left, pair.right).into())
}

pub fn get_function() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_function) }.const_()
}

fn fn_get_function(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    call.func.clone()
}

pub fn set_function() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_function) }.mut_()
}

fn fn_set_function(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    swap(&mut call.func, &mut input);
    input
}

pub fn get_input() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_input) }.const_()
}

fn fn_get_input(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    call.input.clone()
}

pub fn set_input() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_input) }.mut_()
}

fn fn_set_input(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    swap(&mut call.input, &mut input);
    input
}
