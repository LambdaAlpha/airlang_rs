use log::error;

use super::FreePrimFn;
use super::Library;
use super::free_impl;
use crate::cfg::lib::setup::default_free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Link;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct LinkLib {
    pub new: FreePrimFuncVal,
    pub get: FreePrimFuncVal,
    pub set: FreePrimFuncVal,
}

impl Default for LinkLib {
    fn default() -> Self {
        LinkLib { new: new(), get: get(), set: set() }
    }
}

impl Library for LinkLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.new.prelude(ctx);
        self.get.prelude(ctx);
        self.set.prelude(ctx);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "link", f: free_impl(fn_new), mode: default_free_mode() }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(Link::new(input))
}

pub fn get() -> FreePrimFuncVal {
    FreePrimFn { id: "link.get", f: free_impl(fn_get), mode: default_free_mode() }.free()
}

fn fn_get(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Link(link) = input else {
        error!("input {input:?} should be a link");
        return Val::default();
    };
    link.get_clone()
}

pub fn set() -> FreePrimFuncVal {
    FreePrimFn { id: "link.set", f: free_impl(fn_set), mode: default_free_mode() }.free()
}

fn fn_set(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Link(mut link) = pair.first else {
        error!("input.first {:?} should be a link", pair.first);
        return Val::default();
    };
    link.set(pair.second)
}
