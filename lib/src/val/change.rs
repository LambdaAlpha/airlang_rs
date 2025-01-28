use crate::{
    Val,
    change::Change,
    syntax::{
        ReprError,
        repr::change::ChangeRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub ChangeVal(Change<Val, Val>));

impl From<&ChangeRepr> for ChangeVal {
    fn from(value: &ChangeRepr) -> Self {
        let change = Change::new(Val::from(&value.from), Val::from(&value.to));
        Self(Box::new(change))
    }
}

impl From<ChangeRepr> for ChangeVal {
    fn from(value: ChangeRepr) -> Self {
        let change = Change::new(Val::from(value.from), Val::from(value.to));
        Self(Box::new(change))
    }
}

impl TryInto<ChangeRepr> for &ChangeVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ChangeRepr, Self::Error> {
        Ok(ChangeRepr::new(
            (&self.from).try_into()?,
            (&self.to).try_into()?,
        ))
    }
}

impl TryInto<ChangeRepr> for ChangeVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ChangeRepr, Self::Error> {
        let change = ChangeRepr::new(self.0.from.try_into()?, self.0.to.try_into()?);
        Ok(change)
    }
}
