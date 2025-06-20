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
use crate::type_::Pair;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapMode {
    pub some: Map<Val, Option<Mode>>,
    pub else_: Pair<Option<Mode>, Option<Mode>>,
}

impl ModeFn for MapMode {}

impl FreeStaticFn<MapVal, Val> for MapMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        let some = &self.some;
        let key = &self.else_.first;
        let value = &self.else_.second;
        MapForm { some, key, value }.free_static_call(input)
    }
}

impl ConstStaticFn<Val, MapVal, Val> for MapMode {
    fn const_static_call(&self, ctx: ConstRef<Val>, input: MapVal) -> Val {
        let some = &self.some;
        let key = &self.else_.first;
        let value = &self.else_.second;
        MapForm { some, key, value }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Val, MapVal, Val> for MapMode {
    fn mut_static_call(&self, ctx: &mut Val, input: MapVal) -> Val {
        let some = &self.some;
        let key = &self.else_.first;
        let value = &self.else_.second;
        MapForm { some, key, value }.mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for MapMode {
    fn from(mode: PrimMode) -> Self {
        Self { some: Map::default(), else_: Pair::new(Some(mode.into()), Some(mode.into())) }
    }
}
