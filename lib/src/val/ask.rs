use crate::{
    ask::Ask,
    syntax::repr::ask::AskRepr,
    ReprError,
    Val,
};

pub type AskVal = Ask<Val, Val>;

impl From<&AskRepr> for AskVal {
    fn from(value: &AskRepr) -> Self {
        AskVal::new(Val::from(&value.func), Val::from(&value.output))
    }
}

impl From<AskRepr> for AskVal {
    fn from(value: AskRepr) -> Self {
        AskVal::new(Val::from(value.func), Val::from(value.output))
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
        Ok(AskRepr::new(self.func.try_into()?, self.output.try_into()?))
    }
}
