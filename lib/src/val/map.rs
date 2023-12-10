use crate::{
    map::Map,
    syntax::repr::map::MapRepr,
    ReprError,
    Val,
};

pub type MapVal = Map<Val, Val>;

impl From<&MapRepr> for MapVal {
    fn from(value: &MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}

impl From<MapRepr> for MapVal {
    fn from(value: MapRepr) -> Self {
        value
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}

impl TryInto<MapRepr> for &MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v.try_into()?)))
            .try_collect()
    }
}

impl TryInto<MapRepr> for MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v.try_into()?)))
            .try_collect()
    }
}
