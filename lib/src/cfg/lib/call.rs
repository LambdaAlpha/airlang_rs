use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
use crate::cfg::exception::illegal_input;
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
    pub func: ConstPrimFuncVal,
    pub set_func: MutPrimFuncVal,
    pub input: ConstPrimFuncVal,
    pub set_input: MutPrimFuncVal,
}

impl Default for CallLib {
    fn default() -> Self {
        CallLib {
            new: new(),
            func: func(),
            set_func: set_func(),
            input: input(),
            set_input: set_input(),
        }
    }
}

impl CfgMod for CallLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.func.extend(cfg);
        self.set_func.extend(cfg);
        self.input.extend(cfg);
        self.set_input.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "call.new", f: free_impl(fn_new) }.free()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.first, pair.second).into())
}

pub fn func() -> ConstPrimFuncVal {
    DynPrimFn { id: "call.function", f: const_impl(fn_func) }.const_()
}

fn fn_func(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return illegal_ctx(cfg);
    };
    call.func.clone()
}

pub fn set_func() -> MutPrimFuncVal {
    DynPrimFn { id: "call.set_function", f: mut_impl(fn_set_func) }.mut_()
}

fn fn_set_func(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return illegal_ctx(cfg);
    };
    swap(&mut call.func, &mut input);
    input
}

pub fn input() -> ConstPrimFuncVal {
    DynPrimFn { id: "call.input", f: const_impl(fn_input) }.const_()
}

fn fn_input(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        return illegal_ctx(cfg);
    };
    call.input.clone()
}

pub fn set_input() -> MutPrimFuncVal {
    DynPrimFn { id: "call.set_input", f: mut_impl(fn_set_input) }.mut_()
}

fn fn_set_input(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return illegal_ctx(cfg);
    };
    swap(&mut call.input, &mut input);
    input
}
