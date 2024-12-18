use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Val,
    box_wrap,
    call::Call,
    syntax::{
        ReprError,
        repr::call::CallRepr,
    },
};

box_wrap!(pub CallVal(Call<Val, Val>));

impl From<&CallRepr> for CallVal {
    fn from(value: &CallRepr) -> Self {
        let call = Call::new(Val::from(&value.func), Val::from(&value.input));
        Self(Box::new(call))
    }
}

impl From<CallRepr> for CallVal {
    fn from(value: CallRepr) -> Self {
        let call = Call::new(Val::from(value.func), Val::from(value.input));
        Self(Box::new(call))
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
        let call = CallRepr::new(self.0.func.try_into()?, self.0.input.try_into()?);
        Ok(call)
    }
}

impl Debug for CallVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Call::fmt(self, f)
    }
}
