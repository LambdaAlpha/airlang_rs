use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    Val,
    call::Call,
    syntax::{
        ReprError,
        repr::call::CallRepr,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CallVal(Box<Call<Val, Val>>);

impl CallVal {
    #[allow(unused)]
    pub(crate) fn new(call: Box<Call<Val, Val>>) -> Self {
        Self(call)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Call<Val, Val>> {
        self.0
    }
}

impl From<Call<Val, Val>> for CallVal {
    fn from(value: Call<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<CallVal> for Call<Val, Val> {
    fn from(value: CallVal) -> Self {
        *value.0
    }
}

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

impl Deref for CallVal {
    type Target = Call<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CallVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
