use crate::{
    Val,
    either::Either,
    syntax::{
        ReprError,
        repr::either::EitherRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub EitherVal(Either<Val, Val>));

impl From<&EitherRepr> for EitherVal {
    fn from(value: &EitherRepr) -> Self {
        let either = match value {
            EitherRepr::This(this) => Either::This(Val::from(this)),
            EitherRepr::That(that) => Either::That(Val::from(that)),
        };
        Self(Box::new(either))
    }
}

impl From<EitherRepr> for EitherVal {
    fn from(value: EitherRepr) -> Self {
        let either = match value {
            EitherRepr::This(this) => Either::This(Val::from(this)),
            EitherRepr::That(that) => Either::That(Val::from(that)),
        };
        Self(Box::new(either))
    }
}

impl TryInto<EitherRepr> for &EitherVal {
    type Error = ReprError;
    fn try_into(self) -> Result<EitherRepr, Self::Error> {
        let either = match &**self {
            Either::This(this) => Either::This(this.try_into()?),
            Either::That(that) => Either::That(that.try_into()?),
        };
        Ok(either)
    }
}

impl TryInto<EitherRepr> for EitherVal {
    type Error = ReprError;
    fn try_into(self) -> Result<EitherRepr, Self::Error> {
        let either = match *self.0 {
            Either::This(this) => Either::This(this.try_into()?),
            Either::That(that) => Either::That(that.try_into()?),
        };
        Ok(either)
    }
}
