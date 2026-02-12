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
    pub from_any: PrimFuncVal,
}

pub const FROM_ANY: &str = concatcp!(PREFIX_ID, UNIT, ".from_any");

impl Default for UnitLib {
    fn default() -> Self {
        UnitLib { from_any: from_any() }
    }
}

impl CfgMod for UnitLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, FROM_ANY, self.from_any);
    }
}

pub fn from_any() -> PrimFuncVal {
    CtxFreeInputFreeFunc { fn_: fn_from_any }.build()
}

fn fn_from_any(_cfg: &mut Cfg) -> Val {
    Val::Unit(Unit)
}
