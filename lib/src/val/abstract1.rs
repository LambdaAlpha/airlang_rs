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
    abstract1::Abstract,
    syntax::repr::abstract1::AbstractRepr,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AbstractVal(Box<Abstract<Val, Val>>);

impl AbstractVal {
    #[allow(unused)]
    pub(crate) fn new(abstract1: Box<Abstract<Val, Val>>) -> Self {
        Self(abstract1)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Abstract<Val, Val>> {
        self.0
    }
}

impl From<Abstract<Val, Val>> for AbstractVal {
    fn from(value: Abstract<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<AbstractVal> for Abstract<Val, Val> {
    fn from(value: AbstractVal) -> Self {
        *value.0
    }
}

impl From<&AbstractRepr> for AbstractVal {
    fn from(value: &AbstractRepr) -> Self {
        let abstract1 = Abstract::new(Val::from(&value.func), Val::from(&value.input));
        Self(Box::new(abstract1))
    }
}

impl From<AbstractRepr> for AbstractVal {
    fn from(value: AbstractRepr) -> Self {
        let abstract1 = Abstract::new(Val::from(value.func), Val::from(value.input));
        Self(Box::new(abstract1))
    }
}

impl TryInto<AbstractRepr> for &AbstractVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AbstractRepr, Self::Error> {
        Ok(AbstractRepr::new(
            (&self.func).try_into()?,
            (&self.input).try_into()?,
        ))
    }
}

impl TryInto<AbstractRepr> for AbstractVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AbstractRepr, Self::Error> {
        let abstract1 = AbstractRepr::new(self.0.func.try_into()?, self.0.input.try_into()?);
        Ok(abstract1)
    }
}

impl Debug for AbstractVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Abstract::fmt(self, f)
    }
}

impl Deref for AbstractVal {
    type Target = Abstract<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AbstractVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
