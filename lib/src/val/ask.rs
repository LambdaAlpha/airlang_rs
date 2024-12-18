use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Val,
    ask::Ask,
    box_wrap,
    syntax::{
        ReprError,
        repr::ask::AskRepr,
    },
};

box_wrap!(pub AskVal(Ask<Val, Val>));

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
