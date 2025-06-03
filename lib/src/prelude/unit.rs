use crate::FreeStaticPrimFuncVal;
use crate::FuncMode;
use crate::Val;
use crate::prelude::FreeFn;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::free_impl;
use crate::unit::Unit;

#[derive(Clone)]
pub(crate) struct UnitPrelude {
    pub(crate) unit: FreeStaticPrimFuncVal,
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

fn unit() -> FreeStaticPrimFuncVal {
    FreeFn { id: "unit", f: free_impl(fn_unit), mode: FuncMode::default() }.free_static()
}

fn fn_unit(_input: Val) -> Val {
    Val::Unit(Unit)
}
