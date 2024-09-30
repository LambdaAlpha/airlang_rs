use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    ReprError,
    Val,
    map::Map,
    syntax::repr::map::MapRepr,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MapVal(Box<Map<Val, Val>>);

impl MapVal {
    #[allow(unused)]
    pub(crate) fn new(map: Box<Map<Val, Val>>) -> Self {
        Self(map)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Map<Val, Val>> {
        self.0
    }
}

impl From<Map<Val, Val>> for MapVal {
    fn from(value: Map<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<MapVal> for Map<Val, Val> {
    fn from(value: MapVal) -> Self {
        *value.0
    }
}

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

impl Debug for MapVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Map::fmt(self, f)
    }
}

impl Deref for MapVal {
    type Target = Map<Val, Val>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MapVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
