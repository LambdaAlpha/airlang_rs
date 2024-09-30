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
    ask::Ask,
    syntax::repr::ask::AskRepr,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AskVal(Box<Ask<Val, Val>>);

impl AskVal {
    #[allow(unused)]
    pub(crate) fn new(ask: Box<Ask<Val, Val>>) -> Self {
        Self(ask)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Ask<Val, Val>> {
        self.0
    }
}

impl From<Ask<Val, Val>> for AskVal {
    fn from(value: Ask<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<AskVal> for Ask<Val, Val> {
    fn from(value: AskVal) -> Self {
        *value.0
    }
}

impl From<&AskRepr> for AskVal {
    fn from(value: &AskRepr) -> Self {
        let ask = Ask::new(Val::from(&value.func), Val::from(&value.output));
        Self(Box::new(ask))
    }
}

impl From<AskRepr> for AskVal {
    fn from(value: AskRepr) -> Self {
        let ask = Ask::new(Val::from(value.func), Val::from(value.output));
        Self(Box::new(ask))
    }
}

impl TryInto<AskRepr> for &AskVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AskRepr, Self::Error> {
        Ok(AskRepr::new(
            (&self.func).try_into()?,
            (&self.output).try_into()?,
        ))
    }
}

impl TryInto<AskRepr> for AskVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AskRepr, Self::Error> {
        let ask = AskRepr::new(self.0.func.try_into()?, self.0.output.try_into()?);
        Ok(ask)
    }
}

impl Debug for AskVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Ask::fmt(self, f)
    }
}

impl Deref for AskVal {
    type Target = Ask<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AskVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
