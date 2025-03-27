use crate::{
    Val,
    solve::Solve,
    syntax::{
        ReprError,
        repr::solve::SolveRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub SolveVal(Solve<Val, Val>));

impl From<&SolveRepr> for SolveVal {
    fn from(value: &SolveRepr) -> Self {
        let solve = Solve::new(Val::from(&value.func), Val::from(&value.output));
        Self(Box::new(solve))
    }
}

impl From<SolveRepr> for SolveVal {
    fn from(value: SolveRepr) -> Self {
        let solve = Solve::new(Val::from(value.func), Val::from(value.output));
        Self(Box::new(solve))
    }
}

impl TryInto<SolveRepr> for &SolveVal {
    type Error = ReprError;
    fn try_into(self) -> Result<SolveRepr, Self::Error> {
        Ok(SolveRepr::new((&self.func).try_into()?, (&self.output).try_into()?))
    }
}

impl TryInto<SolveRepr> for SolveVal {
    type Error = ReprError;
    fn try_into(self) -> Result<SolveRepr, Self::Error> {
        let solve = SolveRepr::new(self.0.func.try_into()?, self.0.output.try_into()?);
        Ok(solve)
    }
}
