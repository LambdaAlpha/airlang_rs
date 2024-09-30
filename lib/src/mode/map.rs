use crate::{
    Map,
    MapVal,
    Mode,
    PairMode,
    Val,
    ctx::ref1::CtxMeta,
    transformer::Transformer,
};

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct MapMode {
    pub some: Map<Val, Mode>,
    pub else1: PairMode,
}

impl Transformer<MapVal, Val> for MapMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let val_map = Map::from(val_map);
        let map: Map<Val, Val> = val_map
            .into_iter()
            .map(|(k, v)| {
                if let Some(mode) = self.some.get(&k) {
                    let v = mode.transform(ctx.reborrow(), v);
                    (k, v)
                } else {
                    let k = self.else1.first.transform(ctx.reborrow(), k);
                    let v = self.else1.second.transform(ctx.reborrow(), v);
                    (k, v)
                }
            })
            .collect();
        Val::Map(map.into())
    }
}
