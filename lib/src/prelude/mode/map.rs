use super::Mode;
use super::ModeFn;
use super::PrimMode;
use crate::semantics::core::MapForm;
use crate::semantics::func::ConstStaticFn;
use crate::semantics::func::FreeStaticFn;
use crate::semantics::func::MutStaticFn;
use crate::semantics::val::MapVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Map;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapMode {
    pub some: Map<Val, Option<Mode>>,
    pub else_: Option<Mode>,
}

impl ModeFn for MapMode {}

impl FreeStaticFn<MapVal, Val> for MapMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        MapForm { some: &self.some, else_: &self.else_ }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, MapVal, Val> for MapMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        MapForm { some: &self.some, else_: &self.else_ }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, MapVal, Val> for MapMode {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        MapForm { some: &self.some, else_: &self.else_ }.mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for MapMode {
    fn from(mode: PrimMode) -> Self {
        Self { some: Map::default(), else_: Some(mode.into()) }
    }
}
