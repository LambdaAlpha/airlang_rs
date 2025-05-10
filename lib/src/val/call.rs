use crate::Val;
use crate::call::Call;
use crate::syntax::ReprError;
use crate::syntax::repr::call::CallRepr;
use crate::types::wrap::box_wrap;

box_wrap!(pub CallVal(Call<Val, Val>));

impl From<&CallRepr> for CallVal {
    fn from(value: &CallRepr) -> Self {
        Self(Box::new(Call {
            reverse: value.reverse,
            func: Val::from(&value.func),
            input: Val::from(&value.input),
        }))
    }
}

impl From<CallRepr> for CallVal {
    fn from(value: CallRepr) -> Self {
        Self(Box::new(Call {
            reverse: value.reverse,
            func: Val::from(value.func),
            input: Val::from(value.input),
        }))
    }
}

impl TryInto<CallRepr> for &CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(Call {
            reverse: self.reverse,
            func: (&self.func).try_into()?,
            input: (&self.input).try_into()?,
        })
    }
}

impl TryInto<CallRepr> for CallVal {
    type Error = ReprError;
    fn try_into(self) -> Result<CallRepr, Self::Error> {
        Ok(Call {
            reverse: self.0.reverse,
            func: self.0.func.try_into()?,
            input: self.0.input.try_into()?,
        })
    }
}
