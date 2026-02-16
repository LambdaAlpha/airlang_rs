use const_format::concatcp;

use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxFreeInputFreeFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
use crate::type_::Unit;

#[derive(Clone)]
pub struct UnitLib {
    pub default: PrimFuncVal,
}

pub const DEFAULT: &str = concatcp!(PREFIX_ID, UNIT, ".default");

impl Default for UnitLib {
    fn default() -> Self {
        UnitLib { default: CtxFreeInputFreeFunc { fn_: default }.build() }
    }
}

impl CfgMod for UnitLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, DEFAULT, self.default);
    }
}

pub fn default(_cfg: &mut Cfg) -> Val {
    Val::Unit(Unit)
}
