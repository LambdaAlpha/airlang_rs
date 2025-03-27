use crate::{
    Val,
    optimize::Optimize,
    syntax::{
        ReprError,
        repr::optimize::OptimizeRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub OptimizeVal(Optimize<Val, Val>));

impl From<&OptimizeRepr> for OptimizeVal {
    fn from(value: &OptimizeRepr) -> Self {
        let optimize = Optimize::new(Val::from(&value.func), Val::from(&value.input));
        Self(Box::new(optimize))
    }
}

impl From<OptimizeRepr> for OptimizeVal {
    fn from(value: OptimizeRepr) -> Self {
        let optimize = Optimize::new(Val::from(value.func), Val::from(value.input));
        Self(Box::new(optimize))
    }
}

impl TryInto<OptimizeRepr> for &OptimizeVal {
    type Error = ReprError;
    fn try_into(self) -> Result<OptimizeRepr, Self::Error> {
        Ok(OptimizeRepr::new((&self.func).try_into()?, (&self.input).try_into()?))
    }
}

impl TryInto<OptimizeRepr> for OptimizeVal {
    type Error = ReprError;
    fn try_into(self) -> Result<OptimizeRepr, Self::Error> {
        let optimize = OptimizeRepr::new(self.0.func.try_into()?, self.0.input.try_into()?);
        Ok(optimize)
    }
}
