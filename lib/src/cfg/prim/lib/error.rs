use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputAwareFunc;
use crate::semantics::func::CtxFreeInputFreeFunc;
use crate::semantics::func::CtxMutInputFreeFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;

#[derive(Copy, Clone)]
pub struct ErrorLib {
    pub abort: PrimFuncVal,
    pub assert: PrimFuncVal,
    pub is_aborted: PrimFuncVal,
    pub recover: PrimFuncVal,
}

const ERROR: &str = "error";

pub const ABORT: &str = concatcp!(PREFIX_CELL, ERROR, ".abort");
pub const ASSERT: &str = concatcp!(PREFIX_CELL, ERROR, ".assert");
pub const IS_ABORTED: &str = concatcp!(PREFIX_CELL, ERROR, ".is_aborted");
pub const RECOVER: &str = concatcp!(PREFIX_CELL, ERROR, ".recover");

impl Default for ErrorLib {
    fn default() -> Self {
        Self {
            abort: CtxFreeInputFreeFunc { fn_: abort }.build(),
            assert: CtxFreeInputAwareFunc { fn_: assert }.build(),
            is_aborted: CtxConstInputFreeFunc { fn_: is_aborted }.build(),
            recover: CtxMutInputFreeFunc { fn_: recover }.build(),
        }
    }
}

impl CfgMod for ErrorLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, ABORT, self.abort);
        extend_func(cfg, ASSERT, self.assert);
        extend_func(cfg, IS_ABORTED, self.is_aborted);
        extend_func(cfg, RECOVER, self.recover);
    }
}

pub fn abort(cfg: &mut Cfg) -> Val {
    cfg.abort();
    Val::default()
}

pub fn assert(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(bit) = input else {
        return bug!(cfg, "{ASSERT}: expected input.left to be a bit, but got {input}");
    };
    if !*bit {
        cfg.abort();
    }
    Val::default()
}

pub fn is_aborted(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Cfg(target_cfg) = ctx else {
        return bug!(cfg, "{IS_ABORTED}: expected context to be a config, but got {ctx}");
    };
    let aborted = target_cfg.is_aborted();
    Val::Bit(aborted.into())
}

pub fn recover(cfg: &mut Cfg, ctx: &mut Val) -> Val {
    let Val::Cfg(target_cfg) = ctx else {
        return bug!(cfg, "{RECOVER}: expected context to be a config, but got {ctx}");
    };
    target_cfg.recover();
    Val::default()
}
