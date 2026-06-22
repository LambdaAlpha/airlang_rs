use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::export_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::FreeFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

#[derive(Copy, Clone)]
pub struct ErrorLib {
    pub abort: PrimFuncVal,
    pub assert: PrimFuncVal,
}

const ERROR: &str = "error";

pub const ABORT: &str = concatcp!(PREFIX_CELL, ERROR, ".abort");
pub const ASSERT: &str = concatcp!(PREFIX_CELL, ERROR, ".assert");

impl Default for ErrorLib {
    fn default() -> Self {
        Self { abort: FreeFunc { fn_: abort }.build(), assert: CtxFreeFunc { fn_: assert }.build() }
    }
}

impl CfgMod for ErrorLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, ABORT, self.abort);
        export_func(cfg, ASSERT, self.assert);
    }
}

pub fn abort(cfg: &mut Cfg) -> Val {
    cfg.abort()
}

pub fn assert(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Bit(bit) = input else {
        return bug!(cfg, "{ASSERT}: expected input.left to be a bit, but got {input}");
    };
    if !*bit {
        return cfg.abort();
    }
    Val::default()
}
