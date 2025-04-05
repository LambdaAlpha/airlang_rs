use crate::{
    Val,
    optimize::Optimize,
    syntax::{
        ReprError,
        repr::optimize::OptimizeRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub OptimizeVal(Optimize<Val>));

impl From<&OptimizeRepr> for OptimizeVal {
    fn from(value: &OptimizeRepr) -> Self {
        Self(Box::new(Optimize::new(Val::from(&value.func))))
    }
}

impl From<OptimizeRepr> for OptimizeVal {
    fn from(value: OptimizeRepr) -> Self {
        Self(Box::new(Optimize::new(Val::from(value.func))))
    }
}

impl TryInto<OptimizeRepr> for &OptimizeVal {
    type Error = ReprError;
    fn try_into(self) -> Result<OptimizeRepr, Self::Error> {
        Ok(OptimizeRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<OptimizeRepr> for OptimizeVal {
    type Error = ReprError;
    fn try_into(self) -> Result<OptimizeRepr, Self::Error> {
        Ok(OptimizeRepr::new(self.0.func.try_into()?))
    }
}
