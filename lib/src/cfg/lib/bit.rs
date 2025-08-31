use super::FreePrimFn;
use super::Library;
use super::ctx_put_func;
use super::free_impl;
use super::setup::default_free_mode;
use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
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

impl Library for BitLib {
    fn prelude(&self, ctx: &mut Ctx) {
        ctx_put_func(ctx, "not", &self.not);
        ctx_put_func(ctx, "and", &self.and);
        ctx_put_func(ctx, "or", &self.or);
        ctx_put_func(ctx, "xor", &self.xor);
        ctx_put_func(ctx, "imply", &self.imply);
    }
}

pub fn not() -> FreePrimFuncVal {
    FreePrimFn { id: "bit.not", f: free_impl(fn_not), mode: default_free_mode() }.free()
}

fn fn_not(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(b) = input else {
        return Val::default();
    };
    Val::Bit(b.not())
}

pub fn and() -> FreePrimFuncVal {
    FreePrimFn { id: "bit.and", f: free_impl(fn_and), mode: default_free_mode() }.free()
}

fn fn_and(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.and(right))
}

pub fn or() -> FreePrimFuncVal {
    FreePrimFn { id: "bit.or", f: free_impl(fn_or), mode: default_free_mode() }.free()
}

fn fn_or(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.or(right))
}

pub fn xor() -> FreePrimFuncVal {
    FreePrimFn { id: "bit.xor", f: free_impl(fn_xor), mode: default_free_mode() }.free()
}

fn fn_xor(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.xor(right))
}

pub fn imply() -> FreePrimFuncVal {
    FreePrimFn { id: "bit.imply", f: free_impl(fn_imply), mode: default_free_mode() }.free()
}

fn fn_imply(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Bit(left) = pair.first else {
        return Val::default();
    };
    let Val::Bit(right) = pair.second else {
        return Val::default();
    };
    Val::Bit(left.imply(right))
}
