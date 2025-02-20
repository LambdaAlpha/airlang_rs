use crate::{
    Map,
    MapVal,
    Pair,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::Mode,
    transformer::Transformer,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MapMode {
    pub some: Map<Val, Option<Mode>>,
    pub else1: Pair<Option<Mode>, Option<Mode>>,
}

impl Transformer<MapVal, Val> for MapMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where Ctx: CtxMeta<'a> {
        let some = &self.some;
        let key = &self.else1.first;
        let value = &self.else1.second;
        FormCore::transform_map_some_else(some, key, value, ctx, map)
    }
}

impl From<UniMode> for MapMode {
    fn from(mode: UniMode) -> Self {
        let m = Some(Mode::Uni(mode));
        MapMode { some: Map::default(), else1: Pair::new(m.clone(), m) }
    }
}
