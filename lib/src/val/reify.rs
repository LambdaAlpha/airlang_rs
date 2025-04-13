use crate::{
    Val,
    reify::Reify,
    syntax::{
        ReprError,
        repr::reify::ReifyRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub ReifyVal(Reify<Val>));

impl From<&ReifyRepr> for ReifyVal {
    fn from(value: &ReifyRepr) -> Self {
        Self(Box::new(Reify::new(Val::from(&value.func))))
    }
}

impl From<ReifyRepr> for ReifyVal {
    fn from(value: ReifyRepr) -> Self {
        Self(Box::new(Reify::new(Val::from(value.func))))
    }
}

impl TryInto<ReifyRepr> for &ReifyVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ReifyRepr, Self::Error> {
        Ok(ReifyRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<ReifyRepr> for ReifyVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ReifyRepr, Self::Error> {
        Ok(ReifyRepr::new(self.0.func.try_into()?))
    }
}
