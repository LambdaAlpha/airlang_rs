use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Unit;

#[derive(Clone)]
pub struct UnitLib {
    pub unit: FreePrimFuncVal,
}

impl Default for UnitLib {
    fn default() -> Self {
        UnitLib { unit: unit() }
    }
}

impl CfgMod for UnitLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_unit.unit", self.unit);
    }
}

pub fn unit() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_unit) }.free()
}

fn fn_unit(_cfg: &mut Cfg, _input: Val) -> Val {
    Val::Unit(Unit)
}
