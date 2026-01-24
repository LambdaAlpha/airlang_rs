use const_format::concatcp;
use log::error;

use super::FreeImpl;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::BIT;
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

pub const NOT: &str = concatcp!(PREFIX_ID, BIT, ".not");
pub const AND: &str = concatcp!(PREFIX_ID, BIT, ".and");
pub const OR: &str = concatcp!(PREFIX_ID, BIT, ".or");
pub const XOR: &str = concatcp!(PREFIX_ID, BIT, ".xor");
pub const IMPLY: &str = concatcp!(PREFIX_ID, BIT, ".imply");

impl Default for BitLib {
    fn default() -> Self {
        BitLib { not: not(), and: and(), or: or(), xor: xor(), imply: imply() }
    }
}

impl CfgMod for BitLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, NOT, self.not);
        extend_func(cfg, AND, self.and);
        extend_func(cfg, OR, self.or);
        extend_func(cfg, XOR, self.xor);
        extend_func(cfg, IMPLY, self.imply);
    }
}

pub fn not() -> FreePrimFuncVal {
    FreeImpl { free: fn_not }.build()
}

fn fn_not(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(b) = input else {
        error!("input {input:?} should be a bit");
        return illegal_input(cfg);
    };
    Val::Bit(b.not())
}

pub fn and() -> FreePrimFuncVal {
    FreeImpl { free: fn_and }.build()
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
    FreeImpl { free: fn_or }.build()
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
    FreeImpl { free: fn_xor }.build()
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
    FreeImpl { free: fn_imply }.build()
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
