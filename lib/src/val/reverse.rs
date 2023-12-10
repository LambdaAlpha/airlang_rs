use crate::{
    reverse::Reverse,
    syntax::ReverseRepr,
    ReprError,
    Val,
};

pub type ReverseVal = Reverse<Val, Val>;

impl From<&ReverseRepr> for ReverseVal {
    fn from(value: &ReverseRepr) -> Self {
        ReverseVal::new(Val::from(&value.func), Val::from(&value.output))
    }
}

impl From<ReverseRepr> for ReverseVal {
    fn from(value: ReverseRepr) -> Self {
        ReverseVal::new(Val::from(value.func), Val::from(value.output))
    }
}

impl TryInto<ReverseRepr> for &ReverseVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ReverseRepr, Self::Error> {
        Ok(ReverseRepr::new(
            (&self.func).try_into()?,
            (&self.output).try_into()?,
        ))
    }
}

impl TryInto<ReverseRepr> for ReverseVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ReverseRepr, Self::Error> {
        Ok(ReverseRepr::new(
            self.func.try_into()?,
            self.output.try_into()?,
        ))
    }
}
