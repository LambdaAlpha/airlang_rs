use crate::{
    Val,
    box_wrap,
    map::Map,
    syntax::{
        ReprError,
        repr::map::MapRepr,
    },
};

box_wrap!(pub MapVal(Map<Val, Val>));

impl From<&MapRepr> for MapVal {
    fn from(value: &MapRepr) -> Self {
        let map = value
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self(Box::new(map))
    }
}

impl From<MapRepr> for MapVal {
    fn from(value: MapRepr) -> Self {
        let map = value
            .into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect();
        Self(Box::new(map))
    }
}

impl TryInto<MapRepr> for &MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.0
            .iter()
            .map(|(k, v)| Ok((k.try_into()?, v.try_into()?)))
            .collect()
    }
}

impl TryInto<MapRepr> for MapVal {
    type Error = ReprError;
    fn try_into(self) -> Result<MapRepr, Self::Error> {
        self.0
            .into_iter()
            .map(|(k, v)| Ok((k.try_into()?, v.try_into()?)))
            .collect()
    }
}
