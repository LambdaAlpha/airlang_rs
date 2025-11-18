use log::error;

use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
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

impl CfgMod for LinkLib {
    fn extend(self, cfg: &Cfg) {
        self.new.extend(cfg);
        self.get.extend(cfg);
        self.set.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "_link.new", raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(Link::new(input))
}

pub fn get() -> FreePrimFuncVal {
    FreePrimFn { id: "_link.get", raw_input: false, f: free_impl(fn_get) }.free()
}

fn fn_get(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Link(link) = input else {
        error!("input {input:?} should be a link");
        return illegal_input(cfg);
    };
    link.get_clone()
}

pub fn set() -> FreePrimFuncVal {
    FreePrimFn { id: "_link.set", raw_input: false, f: free_impl(fn_set) }.free()
}

fn fn_set(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Link(mut link) = pair.first else {
        error!("input.first {:?} should be a link", pair.first);
        return illegal_input(cfg);
    };
    link.set(pair.second)
}
