use crate::{
    Val,
    abstract1::Abstract,
    syntax::{
        ReprError,
        repr::abstract1::AbstractRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub AbstractVal(Abstract<Val, Val>));

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
