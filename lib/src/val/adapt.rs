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
    adapt::Adapt,
    syntax::repr::adapt::AdaptRepr,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AdaptVal(Box<Adapt<Val, Val>>);

impl AdaptVal {
    #[allow(unused)]
    pub(crate) fn new(adapt: Box<Adapt<Val, Val>>) -> Self {
        Self(adapt)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Adapt<Val, Val>> {
        self.0
    }
}

impl From<Adapt<Val, Val>> for AdaptVal {
    fn from(value: Adapt<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<AdaptVal> for Adapt<Val, Val> {
    fn from(value: AdaptVal) -> Self {
        *value.0
    }
}

impl From<&AdaptRepr> for AdaptVal {
    fn from(value: &AdaptRepr) -> Self {
        let adapt = Adapt::new(Val::from(&value.spec), Val::from(&value.value));
        Self(Box::new(adapt))
    }
}

impl From<AdaptRepr> for AdaptVal {
    fn from(value: AdaptRepr) -> Self {
        let adapt = Adapt::new(Val::from(value.spec), Val::from(value.value));
        Self(Box::new(adapt))
    }
}

impl TryInto<AdaptRepr> for &AdaptVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AdaptRepr, Self::Error> {
        Ok(AdaptRepr::new(
            (&self.spec).try_into()?,
            (&self.value).try_into()?,
        ))
    }
}

impl TryInto<AdaptRepr> for AdaptVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AdaptRepr, Self::Error> {
        let adapt = AdaptRepr::new(self.0.spec.try_into()?, self.0.value.try_into()?);
        Ok(adapt)
    }
}

impl Debug for AdaptVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Adapt::fmt(self, f)
    }
}

impl Deref for AdaptVal {
    type Target = Adapt<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AdaptVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
