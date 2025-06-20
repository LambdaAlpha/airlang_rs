use super::FreeFn;
use super::FuncMode;
use super::Prelude;
use super::PreludeCtx;
use super::free_impl;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Unit;

#[derive(Clone)]
pub struct UnitPrelude {
    pub unit: FreeStaticPrimFuncVal,
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

pub fn unit() -> FreeStaticPrimFuncVal {
    FreeFn { id: "unit", f: free_impl(fn_unit), mode: FuncMode::default() }.free_static()
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
