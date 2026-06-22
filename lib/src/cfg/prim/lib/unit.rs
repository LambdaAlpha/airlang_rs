use const_format::concatcp;

use crate::cfg::CfgMod;
use crate::cfg::export_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::FreeFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Unit;

#[derive(Copy, Clone)]
pub struct UnitLib {
    pub default: PrimFuncVal,
}

pub const DEFAULT: &str = concatcp!(PREFIX_CELL, UNIT, ".default");

impl Default for UnitLib {
    fn default() -> Self {
        Self { default: FreeFunc { fn_: default }.build() }
    }
}

impl CfgMod for UnitLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, DEFAULT, self.default);
    }
}

pub fn default(_cfg: &mut Cfg) -> Val {
    Val::Unit(Unit)
}
