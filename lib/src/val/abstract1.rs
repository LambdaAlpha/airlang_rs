use crate::{
    Val,
    abstract1::Abstract,
    syntax::{
        ReprError,
        repr::abstract1::AbstractRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub AbstractVal(Abstract<Val>));

impl From<&AbstractRepr> for AbstractVal {
    fn from(value: &AbstractRepr) -> Self {
        Self(Box::new(Abstract::new(Val::from(&value.value))))
    }
}

impl From<AbstractRepr> for AbstractVal {
    fn from(value: AbstractRepr) -> Self {
        Self(Box::new(Abstract::new(Val::from(value.value))))
    }
}

impl TryInto<AbstractRepr> for &AbstractVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AbstractRepr, Self::Error> {
        Ok(AbstractRepr::new((&self.value).try_into()?))
    }
}

impl TryInto<AbstractRepr> for AbstractVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AbstractRepr, Self::Error> {
        Ok(AbstractRepr::new(self.0.value.try_into()?))
    }
}
