use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::val::BIT;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct BitLib {
    pub not: PrimFuncVal,
    pub and: PrimFuncVal,
    pub or: PrimFuncVal,
    pub xor: PrimFuncVal,
    pub imply: PrimFuncVal,
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

pub fn not() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_not }.build()
}

fn fn_not(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(b) = input else {
        return bug!(cfg, "{NOT}: expected input to be a bit, but got {input}");
    };
    Val::Bit(b.not())
}

pub fn and() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_and }.build()
}

fn fn_and(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{AND}: expected input to be a pair, but got {input}");
    };
    let Val::Bit(left) = pair.left else {
        return bug!(cfg, "{AND}: expected input.left to be a bit, but got {}", pair.left);
    };
    let Val::Bit(right) = pair.right else {
        return bug!(cfg, "{AND}: expected input.right to be a bit, but got {}", pair.right);
    };
    Val::Bit(left.and(right))
}

pub fn or() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_or }.build()
}

fn fn_or(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{OR}: expected input to be a pair, but got {input}");
    };
    let Val::Bit(left) = pair.left else {
        return bug!(cfg, "{OR}: expected input.left to be a bit, but got {}", pair.left);
    };
    let Val::Bit(right) = pair.right else {
        return bug!(cfg, "{OR}: expected input.right to be a bit, but got {}", pair.right);
    };
    Val::Bit(left.or(right))
}

pub fn xor() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_xor }.build()
}

fn fn_xor(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{XOR}: expected input to be a pair, but got {input}");
    };
    let Val::Bit(left) = pair.left else {
        return bug!(cfg, "{XOR}: expected input.left to be a bit, but got {}", pair.left);
    };
    let Val::Bit(right) = pair.right else {
        return bug!(cfg, "{XOR}: expected input.right to be a bit, but got {}", pair.right);
    };
    Val::Bit(left.xor(right))
}

pub fn imply() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_imply }.build()
}

fn fn_imply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{IMPLY}: expected input to be a pair, but got {input}");
    };
    let Val::Bit(left) = pair.left else {
        return bug!(cfg, "{IMPLY}: expected input.left to be a bit, but got {}", pair.left);
    };
    let Val::Bit(right) = pair.right else {
        return bug!(cfg, "{IMPLY}: expected input.right to be a bit, but got {}", pair.right);
    };
    Val::Bit(left.imply(right))
}
