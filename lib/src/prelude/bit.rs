use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::free_impl;
use crate::val::Val;

#[derive(Clone)]
pub(crate) struct BitPrelude {
    pub(crate) not: FreeStaticPrimFuncVal,
    pub(crate) and: FreeStaticPrimFuncVal,
    pub(crate) or: FreeStaticPrimFuncVal,
    pub(crate) xor: FreeStaticPrimFuncVal,
    pub(crate) imply: FreeStaticPrimFuncVal,
}

impl Default for BitPrelude {
    fn default() -> Self {
        BitPrelude { not: not(), and: and(), or: or(), xor: xor(), imply: imply() }
    }
}

impl Prelude for BitPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.not.put(ctx);
        self.and.put(ctx);
        self.or.put(ctx);
        self.xor.put(ctx);
        self.imply.put(ctx);
    }
}

fn not() -> FreeStaticPrimFuncVal {
    FreeFn { id: "not", f: free_impl(fn_not), mode: FuncMode::default() }.free_static()
}

fn fn_not(input: Val) -> Val {
    let Val::Bit(b) = input else {
        return Val::default();
    };
    Val::Bit(b.not())
}

fn and() -> FreeStaticPrimFuncVal {
    FreeFn { id: "and", f: free_impl(fn_and), mode: FuncMode::default() }.free_static()
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

fn or() -> FreeStaticPrimFuncVal {
    FreeFn { id: "or", f: free_impl(fn_or), mode: FuncMode::default() }.free_static()
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

fn xor() -> FreeStaticPrimFuncVal {
    FreeFn { id: "xor", f: free_impl(fn_xor), mode: FuncMode::default() }.free_static()
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

fn imply() -> FreeStaticPrimFuncVal {
    FreeFn { id: "imply", f: free_impl(fn_imply), mode: FuncMode::default() }.free_static()
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
