use crate::{
    call::Call,
    syntax::repr::call::CallRepr,
    ReprError,
    Val,
};

pub type CallVal = Call<Val, Val>;

impl From<&CallRepr> for CallVal {
    fn from(value: &CallRepr) -> Self {
        CallVal::new(Val::from(&value.func), Val::from(&value.input))
    }
}

impl From<CallRepr> for CallVal {
    fn from(value: CallRepr) -> Self {
        CallVal::new(Val::from(value.func), Val::from(value.input))
    }
}

impl TryInto<CallRepr> for &CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(
            (&self.func).try_into()?,
            (&self.input).try_into()?,
        ))
    }
}

impl TryInto<CallRepr> for CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(CallRepr::new(self.func.try_into()?, self.input.try_into()?))
    }
}
