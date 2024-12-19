use crate::{
    Val,
    box_wrap,
    pair::Pair,
    syntax::{
        ReprError,
        repr::pair::PairRepr,
    },
};

box_wrap!(pub PairVal(Pair<Val, Val>));

impl From<&PairRepr> for PairVal {
    fn from(value: &PairRepr) -> Self {
        let pair = Pair::new(Val::from(&value.first), Val::from(&value.second));
        Self(Box::new(pair))
    }
}

impl From<PairRepr> for PairVal {
    fn from(value: PairRepr) -> Self {
        let pair = Pair::new(Val::from(value.first), Val::from(value.second));
        Self(Box::new(pair))
    }
}

impl TryInto<PairRepr> for &PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        Ok(PairRepr::new(
            (&self.first).try_into()?,
            (&self.second).try_into()?,
        ))
    }
}

impl TryInto<PairRepr> for PairVal {
    type Error = ReprError;
    fn try_into(self) -> Result<PairRepr, Self::Error> {
        let pair = PairRepr::new(self.0.first.try_into()?, self.0.second.try_into()?);
        Ok(pair)
    }
}
