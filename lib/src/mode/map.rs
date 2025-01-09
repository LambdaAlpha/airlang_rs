use crate::{
    Map,
    MapVal,
    Mode,
    Pair,
    PrimitiveMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::{
        id::Id,
        recursive::SelfMode,
    },
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapMode<M> {
    Id,
    Form {
        some: Map<Val, M>,
        else1: Pair<M, M>,
    },
}

impl Transformer<MapVal, Val> for MapMode<Mode> {
    fn transform<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            MapMode::Id => Id.transform_map(ctx, map),
            MapMode::Form { some, else1 } => {
                FormCore::transform_map_some_else(some, &else1.first, &else1.second, ctx, map)
            }
        }
    }
}

impl<M: Default> Default for MapMode<M> {
    fn default() -> Self {
        Self::Form {
            some: Map::default(),
            else1: Pair::default(),
        }
    }
}

impl From<PrimitiveMode> for MapMode<Mode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => MapMode::Id,
            PrimitiveMode::Form(mode) => MapMode::Form {
                some: Map::default(),
                else1: Pair::new(
                    Mode::Primitive(PrimitiveMode::Form(mode)),
                    Mode::Primitive(PrimitiveMode::Form(mode)),
                ),
            },
            PrimitiveMode::Eval(mode) => MapMode::Form {
                some: Map::default(),
                else1: Pair::new(
                    Mode::Primitive(PrimitiveMode::Eval(mode)),
                    Mode::Primitive(PrimitiveMode::Eval(mode)),
                ),
            },
        }
    }
}

impl From<PrimitiveMode> for MapMode<SelfMode> {
    fn from(mode: PrimitiveMode) -> Self {
        match mode {
            PrimitiveMode::Id => MapMode::Id,
            PrimitiveMode::Form(_) => MapMode::Form {
                some: Map::default(),
                else1: Pair::new(SelfMode::Self1, SelfMode::Self1),
            },
            PrimitiveMode::Eval(_) => MapMode::Form {
                some: Map::default(),
                else1: Pair::new(SelfMode::Self1, SelfMode::Self1),
            },
        }
    }
}
