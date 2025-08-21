use super::FreePrimFn;
use super::Prelude;
use super::free_impl;
use crate::prelude::setup::default_free_mode;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct BitPrelude {
    pub not: FreePrimFuncVal,
    pub and: FreePrimFuncVal,
    pub or: FreePrimFuncVal,
    pub xor: FreePrimFuncVal,
    pub imply: FreePrimFuncVal,
}

impl Default for BitPrelude {
    fn default() -> Self {
        BitPrelude { not: not(), and: and(), or: or(), xor: xor(), imply: imply() }
    }
}

impl Prelude for BitPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.not.put(ctx);
        self.and.put(ctx);
        self.or.put(ctx);
        self.xor.put(ctx);
        self.imply.put(ctx);
    }
}

pub fn not() -> FreePrimFuncVal {
    FreePrimFn { id: "not", f: free_impl(fn_not), mode: default_free_mode() }.free()
}

fn fn_not(input: Val) -> Val {
    let Val::Bit(b) = input else {
        return Val::default();
    };
    Val::Bit(b.not())
}

pub fn and() -> FreePrimFuncVal {
    FreePrimFn { id: "and", f: free_impl(fn_and), mode: default_free_mode() }.free()
}

fn fn_and(input: Val) -> Val {
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
    FreePrimFn { id: "or", f: free_impl(fn_or), mode: default_free_mode() }.free()
}

fn fn_or(input: Val) -> Val {
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
    FreePrimFn { id: "xor", f: free_impl(fn_xor), mode: default_free_mode() }.free()
}

fn fn_xor(input: Val) -> Val {
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
    FreePrimFn { id: "imply", f: free_impl(fn_imply), mode: default_free_mode() }.free()
}

fn fn_imply(input: Val) -> Val {
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
