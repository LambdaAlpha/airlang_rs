use const_format::concatcp;
use log::error;

use super::ConstImpl;
use super::FreeImpl;
use super::MutImpl;
use super::abort_const;
use super::abort_free;
use crate::cfg::CfgMod;
use crate::cfg::error::abort_bug_with_msg;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
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

const ERROR: &str = "error";

pub const ABORT: &str = concatcp!(PREFIX_ID, ERROR, ".abort");
pub const ASSERT: &str = concatcp!(PREFIX_ID, ERROR, ".assert");
pub const IS_ABORTED: &str = concatcp!(PREFIX_ID, ERROR, ".is_aborted");
pub const RECOVER: &str = concatcp!(PREFIX_ID, ERROR, ".recover");

impl Default for ErrorLib {
    fn default() -> Self {
        ErrorLib { abort: abort(), assert: assert(), is_aborted: is_aborted(), recover: recover() }
    }
}

impl CfgMod for ErrorLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, ABORT, self.abort);
        extend_func(cfg, ASSERT, self.assert);
        extend_func(cfg, IS_ABORTED, self.is_aborted);
        extend_func(cfg, RECOVER, self.recover);
    }
}

pub fn abort() -> FreePrimFuncVal {
    FreeImpl { free: fn_abort }.build()
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
    FreeImpl { free: fn_assert }.build()
}

fn fn_assert(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Bit(bit) = pair.left else {
        error!("input.left {:?} should be a bit", pair.left);
        return illegal_input(cfg);
    };
    let Val::Text(message) = pair.right else {
        error!("input.right {:?} should be a text", pair.right);
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
    ConstImpl { free: abort_free(IS_ABORTED), const_: fn_is_aborted }.build()
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
    MutImpl { free: abort_free(RECOVER), const_: abort_const(RECOVER), mut_: fn_recover }.build()
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
