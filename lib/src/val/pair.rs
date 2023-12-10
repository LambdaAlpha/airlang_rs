use crate::{
    pair::Pair,
    syntax::repr::pair::PairRepr,
    ReprError,
    Val,
};

pub type PairVal = Pair<Val, Val>;

impl From<&PairRepr> for PairVal {
    fn from(value: &PairRepr) -> Self {
        PairVal::new(Val::from(&value.first), Val::from(&value.second))
    }
}

impl From<PairRepr> for PairVal {
    fn from(value: PairRepr) -> Self {
        PairVal::new(Val::from(value.first), Val::from(value.second))
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
        Ok(PairRepr::new(
            self.first.try_into()?,
            self.second.try_into()?,
        ))
    }
}
