use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_input;
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
        self.not.extend(cfg);
        self.and.extend(cfg);
        self.or.extend(cfg);
        self.xor.extend(cfg);
        self.imply.extend(cfg);
    }
}

pub fn not() -> FreePrimFuncVal {
    FreePrimFn { id: "_bit.not", raw_input: false, f: free_impl(fn_not) }.free()
}

fn fn_not(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(b) = input else {
        return illegal_input(cfg);
    };
    Val::Bit(b.not())
}

pub fn and() -> FreePrimFuncVal {
    FreePrimFn { id: "_bit.and", raw_input: false, f: free_impl(fn_and) }.free()
}

fn fn_and(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.first else {
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.second else {
        return illegal_input(cfg);
    };
    Val::Bit(left.and(right))
}

pub fn or() -> FreePrimFuncVal {
    FreePrimFn { id: "_bit.or", raw_input: false, f: free_impl(fn_or) }.free()
}

fn fn_or(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.first else {
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.second else {
        return illegal_input(cfg);
    };
    Val::Bit(left.or(right))
}

pub fn xor() -> FreePrimFuncVal {
    FreePrimFn { id: "_bit.xor", raw_input: false, f: free_impl(fn_xor) }.free()
}

fn fn_xor(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.first else {
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.second else {
        return illegal_input(cfg);
    };
    Val::Bit(left.xor(right))
}

pub fn imply() -> FreePrimFuncVal {
    FreePrimFn { id: "_bit.imply", raw_input: false, f: free_impl(fn_imply) }.free()
}

fn fn_imply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return illegal_input(cfg);
    };
    let Val::Bit(left) = pair.first else {
        return illegal_input(cfg);
    };
    let Val::Bit(right) = pair.second else {
        return illegal_input(cfg);
    };
    Val::Bit(left.imply(right))
}
