use crate::{
    ctx::ref1::CtxMeta,
    mode::{
        eval::Eval,
        id::Id,
    },
    transformer::Transformer,
    Map,
    MapVal,
    Mode,
    PairMode,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapMode {
    All(PairMode),
    Some(Map<Val, Mode>),
}

impl Default for MapMode {
    fn default() -> Self {
        MapMode::All(Default::default())
    }
}

impl Transformer<MapVal, Val> for MapMode {
    fn transform<'a, Ctx>(&self, mut ctx: Ctx, val_map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        let val_map = Map::from(val_map);
        match self {
            MapMode::All(mode) => {
                let map: Map<Val, Val> = val_map
                    .into_iter()
                    .map(|(k, v)| {
                        let k = mode.first.transform(ctx.reborrow(), k);
                        let v = mode.second.transform(ctx.reborrow(), v);
                        (k, v)
                    })
                    .collect();
                Val::Map(map.into())
            }
            MapMode::Some(mode_map) => {
                let map: Map<Val, Val> = val_map
                    .into_iter()
                    .map(|(k, v)| {
                        let v = if let Some(mode) = mode_map.get(&k) {
                            mode.transform(ctx.reborrow(), v)
                        } else {
                            Eval.transform(ctx.reborrow(), v)
                        };
                        let k = Id.transform(ctx.reborrow(), k);
                        (k, v)
                    })
                    .collect();
                Val::Map(map.into())
            }
        }
    }
}
