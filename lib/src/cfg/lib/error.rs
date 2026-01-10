use log::error;

use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::const_impl;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct ErrorLib {
    pub abort: FreePrimFuncVal,
    pub is_aborted: ConstPrimFuncVal,
    pub get_abort_reason: ConstPrimFuncVal,
}

impl Default for ErrorLib {
    fn default() -> Self {
        ErrorLib { abort: abort(), is_aborted: is_aborted(), get_abort_reason: get_abort_reason() }
    }
}

impl CfgMod for ErrorLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_error.abort", self.abort);
        extend_func(cfg, "_error.is_aborted", self.is_aborted);
        extend_func(cfg, "_error.get_abort_reason", self.get_abort_reason);
    }
}

pub fn abort() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_abort) }.free()
}

fn fn_abort(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Key(reason) = input else {
        error!("input {input:?} should be a key");
        return illegal_input(cfg);
    };
    cfg.abort(reason);
    Val::default()
}

pub fn is_aborted() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: true, f: const_impl(fn_is_aborted) }.const_()
}

fn fn_is_aborted(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Cfg(target_cfg) = &*ctx else {
        error!("ctx {ctx:?} should be a cfg");
        return illegal_ctx(cfg);
    };
    let aborted = target_cfg.is_aborted();
    Val::Bit(aborted.into())
}

pub fn get_abort_reason() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: true, f: const_impl(fn_get_abort_reason) }.const_()
}

fn fn_get_abort_reason(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Cfg(target_cfg) = &*ctx else {
        error!("ctx {ctx:?} should be a cfg");
        return illegal_ctx(cfg);
    };
    let abort_reason = target_cfg.abort_reason();
    Val::Key(abort_reason)
}
