use crate::{
    Val,
    equiv::Equiv,
    syntax::{
        ReprError,
        repr::equiv::EquivRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub EquivVal(Equiv<Val>));

impl From<&EquivRepr> for EquivVal {
    fn from(value: &EquivRepr) -> Self {
        Self(Box::new(Equiv::new(Val::from(&value.func))))
    }
}

impl From<EquivRepr> for EquivVal {
    fn from(value: EquivRepr) -> Self {
        Self(Box::new(Equiv::new(Val::from(value.func))))
    }
}

impl TryInto<EquivRepr> for &EquivVal {
    type Error = ReprError;
    fn try_into(self) -> Result<EquivRepr, Self::Error> {
        Ok(EquivRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<EquivRepr> for EquivVal {
    type Error = ReprError;
    fn try_into(self) -> Result<EquivRepr, Self::Error> {
        Ok(EquivRepr::new(self.0.func.try_into()?))
    }
}
