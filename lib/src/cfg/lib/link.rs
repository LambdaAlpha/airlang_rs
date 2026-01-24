use std::ops::DerefMut;

use const_format::concatcp;
use log::error;

use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::LINK;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::type_::DynRef;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct LinkLib {
    pub new: FreePrimFuncVal,
    pub new_constant: FreePrimFuncVal,
    pub which: FreePrimFuncVal,
}

pub const NEW: &str = concatcp!(PREFIX_ID, LINK, ".new");
pub const NEW_CONSTANT: &str = concatcp!(PREFIX_ID, LINK, ".new_constant");
pub const WHICH: &str = concatcp!(PREFIX_ID, LINK, ".which");

impl Default for LinkLib {
    fn default() -> Self {
        LinkLib { new: new(), new_constant: new_constant(), which: which() }
    }
}

impl CfgMod for LinkLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, NEW, self.new);
        extend_func(cfg, NEW_CONSTANT, self.new_constant);
        extend_func(cfg, WHICH, self.which);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(LinkVal::new(input, false))
}

pub fn new_constant() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_new_constant) }.free()
}

fn fn_new_constant(_cfg: &mut Cfg, input: Val) -> Val {
    Val::Link(LinkVal::new(input, true))
}

pub fn which() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_which) }.free()
}

fn fn_which(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Link(link) = pair.left else {
        error!("input.left {:?} should be a link", pair.left);
        return illegal_input(cfg);
    };
    let Val::Pair(func_input) = pair.right else {
        error!("input.right {:?} should be a pair", pair.right);
        return illegal_input(cfg);
    };
    let func_input = Pair::from(func_input);
    let Val::Func(func) = func_input.left else {
        error!("input.right.left should be a func");
        return illegal_input(cfg);
    };
    let Ok(mut ctx) = link.try_borrow_mut() else {
        error!("link is already borrowed");
        return abort_bug_with_msg(cfg, "link is in use");
    };
    let const_ = link.is_const();
    func.dyn_call(cfg, DynRef::new(ctx.deref_mut(), const_), func_input.right)
}
