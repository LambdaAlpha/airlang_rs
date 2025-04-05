use crate::{
    Val,
    inverse::Inverse,
    syntax::{
        ReprError,
        repr::inverse::InverseRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub InverseVal(Inverse<Val>));

impl From<&InverseRepr> for InverseVal {
    fn from(value: &InverseRepr) -> Self {
        Self(Box::new(Inverse::new(Val::from(&value.func))))
    }
}

impl From<InverseRepr> for InverseVal {
    fn from(value: InverseRepr) -> Self {
        Self(Box::new(Inverse::new(Val::from(value.func))))
    }
}

impl TryInto<InverseRepr> for &InverseVal {
    type Error = ReprError;
    fn try_into(self) -> Result<InverseRepr, Self::Error> {
        Ok(InverseRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<InverseRepr> for InverseVal {
    type Error = ReprError;
    fn try_into(self) -> Result<InverseRepr, Self::Error> {
        Ok(InverseRepr::new(self.0.func.try_into()?))
    }
}
