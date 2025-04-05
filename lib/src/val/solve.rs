use crate::{
    Val,
    solve::Solve,
    syntax::{
        ReprError,
        repr::solve::SolveRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub SolveVal(Solve<Val>));

impl From<&SolveRepr> for SolveVal {
    fn from(value: &SolveRepr) -> Self {
        Self(Box::new(Solve::new(Val::from(&value.func))))
    }
}

impl From<SolveRepr> for SolveVal {
    fn from(value: SolveRepr) -> Self {
        Self(Box::new(Solve::new(Val::from(value.func))))
    }
}

impl TryInto<SolveRepr> for &SolveVal {
    type Error = ReprError;
    fn try_into(self) -> Result<SolveRepr, Self::Error> {
        Ok(SolveRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<SolveRepr> for SolveVal {
    type Error = ReprError;
    fn try_into(self) -> Result<SolveRepr, Self::Error> {
        Ok(SolveRepr::new(self.0.func.try_into()?))
    }
}
