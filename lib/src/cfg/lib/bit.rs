use log::error;

use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct BitLib {
    pub not: FreePrimFuncVal,
    pub and: FreePrimFuncVal,
    pub or: FreePrimFuncVal,
    pub xor: FreePrimFuncVal,
    pub imply: FreePrimFuncVal,
}

impl Default for BitLib {
    fn default() -> Self {
        BitLib { not: not(), and: and(), or: or(), xor: xor(), imply: imply() }
    }
}

impl CfgMod for BitLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_bit.not", self.not);
        extend_func(cfg, "_bit.and", self.and);
        extend_func(cfg, "_bit.or", self.or);
        extend_func(cfg, "_bit.xor", self.xor);
        extend_func(cfg, "_bit.imply", self.imply);
    }
}

pub fn not() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_not) }.free()
}

fn fn_not(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(b) = input else {
        error!("input {input:?} should be a bit");
        return illegal_input(cfg);
    };
    Val::Bit(b.not())
}

pub fn and() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_and) }.free()
}

fn fn_and(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.left else {
        error!("input.left {:?} should be a bit", pair.left);
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.right else {
        error!("input.right {:?} should be a bit", pair.right);
        return illegal_input(cfg);
    };
    Val::Bit(left.and(right))
}

pub fn or() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_or) }.free()
}

fn fn_or(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.left else {
        error!("input.left {:?} should be a bit", pair.left);
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.right else {
        error!("input.right {:?} should be a bit", pair.right);
        return illegal_input(cfg);
    };
    Val::Bit(left.or(right))
}

pub fn xor() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_xor) }.free()
}

fn fn_xor(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.left else {
        error!("input.left {:?} should be a bit", pair.left);
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.right else {
        error!("input.right {:?} should be a bit", pair.right);
        return illegal_input(cfg);
    };
    Val::Bit(left.xor(right))
}

pub fn imply() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_imply) }.free()
}

fn fn_imply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.left else {
        error!("input.left {:?} should be a bit", pair.left);
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.right else {
        error!("input.right {:?} should be a bit", pair.right);
        return illegal_input(cfg);
    };
    Val::Bit(left.imply(right))
}
