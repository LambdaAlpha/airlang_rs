use std::ops::DerefMut;

use log::error;

use super::DynPrimFn;
use super::FreePrimFn;
use super::dyn_impl;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::error::fail;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::LinkVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::DynRef;
use crate::type_::Pair;

// todo design
#[derive(Clone)]
pub struct LinkLib {
    pub new: FreePrimFuncVal,
    pub new_constant: FreePrimFuncVal,
    pub which: MutPrimFuncVal,
}

impl Default for LinkLib {
    fn default() -> Self {
        LinkLib { new: new(), new_constant: new_constant(), which: which() }
    }
}

impl CfgMod for LinkLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_link.new", self.new);
        extend_func(cfg, "_link.new_constant", self.new_constant);
        extend_func(cfg, "_link.which", self.which);
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

pub fn which() -> MutPrimFuncVal {
    DynPrimFn { raw_input: true, f: dyn_impl(fn_which) }.mut_()
}

fn fn_which(cfg: &mut Cfg, mut ctx: DynRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let link = Eval.dyn_call(cfg, ctx.reborrow(), pair.first);
    let Val::Link(link) = link else {
        error!("input.first {link:?} should be a link");
        return illegal_input(cfg);
    };
    let Val::Call(call) = pair.second else {
        error!("input.second {:?} should be a call", pair.second);
        return illegal_input(cfg);
    };
    let call = Call::from(call);
    let Val::Func(func) = Eval.dyn_call(cfg, ctx.reborrow(), call.func) else {
        error!("input.second.func should be a func");
        return fail(cfg);
    };
    let input =
        if func.raw_input() { call.input } else { Eval.dyn_call(cfg, ctx.reborrow(), call.input) };
    let Ok(mut ctx) = link.try_borrow_mut() else {
        error!("link is already borrowed");
        return Val::default();
    };
    if link.is_const() {
        func.const_call(cfg, ConstRef::new(ctx.deref_mut()), input)
    } else {
        func.mut_call(cfg, ctx.deref_mut(), input)
    }
}
