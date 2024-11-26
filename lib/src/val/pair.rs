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
    Val,
    pair::Pair,
    syntax::{
        ReprError,
        repr::pair::PairRepr,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PairVal(Box<Pair<Val, Val>>);

impl PairVal {
    #[allow(unused)]
    pub(crate) fn new(pair: Box<Pair<Val, Val>>) -> Self {
        Self(pair)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Pair<Val, Val>> {
        self.0
    }
}

impl From<Pair<Val, Val>> for PairVal {
    fn from(value: Pair<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<PairVal> for Pair<Val, Val> {
    fn from(value: PairVal) -> Self {
        *value.0
    }
}

impl From<&PairRepr> for PairVal {
    fn from(value: &PairRepr) -> Self {
        let pair = Pair::new(Val::from(&value.first), Val::from(&value.second));
        Self(Box::new(pair))
    }
}

impl From<PairRepr> for PairVal {
    fn from(value: PairRepr) -> Self {
        let pair = Pair::new(Val::from(value.first), Val::from(value.second));
        Self(Box::new(pair))
    }
}

impl TryInto<PairRepr> for &PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        Ok(PairRepr::new(
            (&self.first).try_into()?,
            (&self.second).try_into()?,
        ))
    }
}

impl TryInto<PairRepr> for PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        let pair = PairRepr::new(self.0.first.try_into()?, self.0.second.try_into()?);
        Ok(pair)
    }
}

impl Debug for PairVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Pair::fmt(self, f)
    }
}

impl Deref for PairVal {
    type Target = Pair<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PairVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
