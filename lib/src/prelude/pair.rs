use std::mem::swap;

use super::DynFn;
use super::FreeFn;
use super::Prelude;
use super::PreludeCtx;
use super::const_impl;
use super::ctx_default_mode;
use super::free_impl;
use super::mut_impl;
use crate::semantics::func::FuncMode;
use crate::semantics::val::ConstStaticPrimFuncVal;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::MutStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::PAIR;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct PairPrelude {
    pub new: FreeStaticPrimFuncVal,
    pub first: ConstStaticPrimFuncVal,
    pub set_first: MutStaticPrimFuncVal,
    pub second: ConstStaticPrimFuncVal,
    pub set_second: MutStaticPrimFuncVal,
}

impl Default for PairPrelude {
    fn default() -> Self {
        PairPrelude {
            new: new(),
            first: first(),
            set_first: set_first(),
            second: second(),
            set_second: set_second(),
        }
    }
}

impl Prelude for PairPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.new.put(ctx);
        self.first.put(ctx);
        self.set_first.put(ctx);
        self.second.put(ctx);
        self.set_second.put(ctx);
    }
}

pub fn new() -> FreeStaticPrimFuncVal {
    FreeFn { id: PAIR, f: free_impl(fn_new), mode: FuncMode::default() }.free_static()
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(_) = input else {
        return Val::default();
    };
    input
}

pub fn first() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "pair.first",
        f: const_impl(fn_first),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_first(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return Val::default();
    };
    pair.first.clone()
}

pub fn set_first() -> MutStaticPrimFuncVal {
    DynFn {
        id: "pair.set_first",
        f: mut_impl(fn_set_first),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_set_first(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return Val::default();
    };
    swap(&mut pair.first, &mut input);
    input
}

pub fn second() -> ConstStaticPrimFuncVal {
    DynFn {
        id: "pair.second",
        f: const_impl(fn_second),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .const_static()
}

fn fn_second(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return Val::default();
    };
    pair.second.clone()
}

pub fn set_second() -> MutStaticPrimFuncVal {
    DynFn {
        id: "pair.set_second",
        f: mut_impl(fn_set_second),
        mode: FuncMode { forward: ctx_default_mode(), reverse: FuncMode::default_mode() },
        ctx_explicit: true,
    }
    .mut_static()
}

fn fn_set_second(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return Val::default();
    };
    swap(&mut pair.second, &mut input);
    input
}
