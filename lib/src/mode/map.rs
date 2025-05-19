use crate::ConstRef;
use crate::ConstStaticFn;
use crate::Ctx;
use crate::FreeStaticFn;
use crate::Map;
use crate::MapVal;
use crate::MutStaticFn;
use crate::Pair;
use crate::PrimMode;
use crate::Val;
use crate::core::MapForm;
use crate::mode::Mode;
use crate::mode::ModeFn;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapMode {
    pub some: Map<Val, Option<Mode>>,
    pub else1: Pair<Option<Mode>, Option<Mode>>,
}

impl ModeFn for MapMode {}

impl FreeStaticFn<MapVal, Val> for MapMode {
    fn free_static_call(&self, input: MapVal) -> Val {
        let some = &self.some;
        let key = &self.else1.first;
        let value = &self.else1.second;
        MapForm { some, key, value }.free_static_call(input)
    }
}

impl ConstStaticFn<Ctx, MapVal, Val> for MapMode {
    fn const_static_call(&self, ctx: ConstRef<Ctx>, input: MapVal) -> Val {
        let some = &self.some;
        let key = &self.else1.first;
        let value = &self.else1.second;
        MapForm { some, key, value }.const_static_call(ctx, input)
    }
}

impl MutStaticFn<Ctx, MapVal, Val> for MapMode {
    fn mut_static_call(&self, ctx: &mut Ctx, input: MapVal) -> Val {
        let some = &self.some;
        let key = &self.else1.first;
        let value = &self.else1.second;
        MapForm { some, key, value }.mut_static_call(ctx, input)
    }
}

impl From<PrimMode> for MapMode {
    fn from(mode: PrimMode) -> Self {
        Self { some: Map::default(), else1: Pair::new(Some(mode.into()), Some(mode.into())) }
    }
}
