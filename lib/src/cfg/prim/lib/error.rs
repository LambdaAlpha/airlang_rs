use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::core::abort_by_bug_with_msg;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::func::CtxFreeInputFreeFunc;
use crate::semantics::func::CtxMutInputFreeFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Pair;
use crate::type_::Text;

#[derive(Clone)]
pub struct ErrorLib {
    pub abort: PrimFuncVal,
    pub assert: PrimFuncVal,
    pub is_aborted: PrimFuncVal,
    pub recover: PrimFuncVal,
}

const ERROR: &str = "error";

pub const ABORT: &str = concatcp!(PREFIX_ID, ERROR, ".abort");
pub const ASSERT: &str = concatcp!(PREFIX_ID, ERROR, ".assert");
pub const IS_ABORTED: &str = concatcp!(PREFIX_ID, ERROR, ".is_aborted");
pub const RECOVER: &str = concatcp!(PREFIX_ID, ERROR, ".recover");

impl Default for ErrorLib {
    fn default() -> Self {
        ErrorLib {
            abort: CtxFreeInputFreeFunc { fn_: abort }.build(),
            assert: CtxFreeInputEvalFunc { fn_: assert }.build(),
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
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{ASSERT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Bit(bit) = pair.left else {
        return bug!(cfg, "{ASSERT}: expected input.left to be a bit, but got {}", pair.left);
    };
    let Val::Text(message) = pair.right else {
        return bug!(cfg, "{ASSERT}: expected input.right to be a text, but got {}", pair.right);
    };
    let message = Text::from(message);
    if !*bit {
        return abort_by_bug_with_msg(cfg, message);
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
