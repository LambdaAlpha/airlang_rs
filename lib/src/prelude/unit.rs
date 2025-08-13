use super::FreePrimFn;
use super::Prelude;
use super::PreludeCtx;
use super::free_impl;
use crate::prelude::setup::default_free_mode;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Unit;

#[derive(Clone)]
pub struct UnitPrelude {
    pub unit: FreePrimFuncVal,
}

impl Default for UnitPrelude {
    fn default() -> Self {
        UnitPrelude { unit: unit() }
    }
}

impl Prelude for UnitPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.unit.put(ctx);
    }
}

pub fn unit() -> FreePrimFuncVal {
    FreePrimFn { id: "unit", f: free_impl(fn_unit), mode: default_free_mode() }.free()
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
