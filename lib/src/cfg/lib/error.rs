use log::error;

use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::const_impl;
use crate::cfg::lib::free_impl;
use crate::cfg::lib::mut_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Pair;
use crate::type_::Text;

#[derive(Clone)]
pub struct ErrorLib {
    pub abort: FreePrimFuncVal,
    pub assert: FreePrimFuncVal,
    pub is_aborted: ConstPrimFuncVal,
    pub recover: MutPrimFuncVal,
}

impl Default for ErrorLib {
    fn default() -> Self {
        ErrorLib { abort: abort(), assert: assert(), is_aborted: is_aborted(), recover: recover() }
    }
}

impl CfgMod for ErrorLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_error.abort", self.abort);
        extend_func(cfg, "_error.assert", self.assert);
        extend_func(cfg, "_error.is_aborted", self.is_aborted);
        extend_func(cfg, "_error.recover", self.recover);
    }
}

pub fn abort() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_abort) }.free()
}

fn fn_abort(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Unit(_) = input else {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    };
    cfg.abort();
    Val::default()
}

pub fn assert() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_assert) }.free()
}

fn fn_assert(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Bit(bit) = pair.first else {
        error!("input.first {:?} should be a bit", pair.first);
        return illegal_input(cfg);
    };
    let Val::Text(message) = pair.second else {
        error!("input.second {:?} should be a text", pair.second);
        return illegal_input(cfg);
    };
    let message = Text::from(message);
    if !*bit {
        error!("assertion failed: {message}");
        return abort_bug_with_msg(cfg, &message);
    }
    Val::default()
}

pub fn is_aborted() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_is_aborted) }.const_()
}

fn fn_is_aborted(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Cfg(target_cfg) = &*ctx else {
        error!("ctx {ctx:?} should be a cfg");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let aborted = target_cfg.is_aborted();
    Val::Bit(aborted.into())
}

pub fn recover() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_recover) }.mut_()
}

fn fn_recover(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let Val::Cfg(target_cfg) = ctx else {
        error!("ctx {ctx:?} should be a cfg");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    target_cfg.recover();
    Val::default()
}
