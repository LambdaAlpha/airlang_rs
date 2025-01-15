use crate::{
    Map,
    MapVal,
    Mode,
    Pair,
    UniMode,
    Val,
    core::FormCore,
    ctx::ref1::CtxMeta,
    mode::id::Id,
    transformer::{
        ByVal,
        Transformer,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MapMode {
    Id(Id),
    Form {
        some: Map<Val, Mode>,
        else1: Pair<Mode, Mode>,
    },
}

impl Transformer<MapVal, Val> for MapMode {
    fn transform<'a, Ctx>(&self, ctx: Ctx, map: MapVal) -> Val
    where
        Ctx: CtxMeta<'a>,
    {
        match self {
            MapMode::Id(mode) => mode.transform_map(ctx, map),
            MapMode::Form { some, else1 } => {
                FormCore::transform_map_some_else(some, &else1.first, &else1.second, ctx, map)
            }
        }
    }
}

impl Default for MapMode {
    fn default() -> Self {
        Self::Form {
            some: Map::default(),
            else1: Pair::default(),
        }
    }
}

impl From<UniMode> for MapMode {
    fn from(mode: UniMode) -> Self {
        match mode {
            UniMode::Id(mode) => MapMode::Id(mode),
            UniMode::Form(mode) => MapMode::Form {
                some: Map::default(),
                else1: Pair::new(
                    Mode::Uni(UniMode::Form(mode)),
                    Mode::Uni(UniMode::Form(mode)),
                ),
            },
            UniMode::Eval(mode) => MapMode::Form {
                some: Map::default(),
                else1: Pair::new(
                    Mode::Uni(UniMode::Eval(mode)),
                    Mode::Uni(UniMode::Eval(mode)),
                ),
            },
        }
    }
}
