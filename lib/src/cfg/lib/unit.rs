use const_format::concatcp;

use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
use crate::type_::Unit;

#[derive(Clone)]
pub struct UnitLib {
    pub from_any: FreePrimFuncVal,
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

pub fn from_any() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_from_any) }.free()
}

fn fn_from_any(_cfg: &mut Cfg, _input: Val) -> Val {
    Val::Unit(Unit)
}
