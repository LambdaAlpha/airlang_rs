use super::FreePrimFn;
use super::Library;
use super::free_impl;
use crate::cfg::lib::setup::default_free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
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

impl Library for UnitLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.unit.prelude(ctx);
    }
}

pub fn unit() -> FreePrimFuncVal {
    FreePrimFn { id: "unit", f: free_impl(fn_unit), mode: default_free_mode() }.free()
}

fn fn_unit(_cfg: &mut Cfg, _input: Val) -> Val {
    Val::Unit(Unit)
}
