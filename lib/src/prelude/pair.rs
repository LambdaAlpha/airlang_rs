use std::mem::swap;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::Prelude;
use super::const_impl;
use super::free_impl;
use super::mut_impl;
use super::setup::default_dyn_mode;
use super::setup::default_free_mode;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::syntax::PAIR;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct PairPrelude {
    pub new: FreePrimFuncVal,
    pub first: ConstPrimFuncVal,
    pub set_first: MutPrimFuncVal,
    pub second: ConstPrimFuncVal,
    pub set_second: MutPrimFuncVal,
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
    fn put(self, ctx: &mut Ctx) {
        self.new.put(ctx);
        self.first.put(ctx);
        self.set_first.put(ctx);
        self.second.put(ctx);
        self.set_second.put(ctx);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: PAIR, f: free_impl(fn_new), mode: default_free_mode() }.free()
}

fn fn_new(input: Val) -> Val {
    let Val::Pair(_) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    input
}

pub fn first() -> ConstPrimFuncVal {
    DynPrimFn { id: "pair.first", f: const_impl(fn_first), mode: default_dyn_mode() }.const_()
}

fn fn_first(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    pair.first.clone()
}

pub fn set_first() -> MutPrimFuncVal {
    DynPrimFn { id: "pair.set_first", f: mut_impl(fn_set_first), mode: default_dyn_mode() }.mut_()
}

fn fn_set_first(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    swap(&mut pair.first, &mut input);
    input
}

pub fn second() -> ConstPrimFuncVal {
    DynPrimFn { id: "pair.second", f: const_impl(fn_second), mode: default_dyn_mode() }.const_()
}

fn fn_second(ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    pair.second.clone()
}

pub fn set_second() -> MutPrimFuncVal {
    DynPrimFn { id: "pair.set_second", f: mut_impl(fn_set_second), mode: default_dyn_mode() }.mut_()
}

fn fn_set_second(ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        error!("ctx {ctx:?} should be a pair");
        return Val::default();
    };
    swap(&mut pair.second, &mut input);
    input
}
