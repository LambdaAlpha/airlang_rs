use crate::{
    Val,
    generate::Generate,
    syntax::{
        ReprError,
        repr::generate::GenerateRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub GenerateVal(Generate<Val>));

impl From<&GenerateRepr> for GenerateVal {
    fn from(value: &GenerateRepr) -> Self {
        Self(Box::new(Generate::new(Val::from(&value.func))))
    }
}

impl From<GenerateRepr> for GenerateVal {
    fn from(value: GenerateRepr) -> Self {
        Self(Box::new(Generate::new(Val::from(value.func))))
    }
}

impl TryInto<GenerateRepr> for &GenerateVal {
    type Error = ReprError;
    fn try_into(self) -> Result<GenerateRepr, Self::Error> {
        Ok(GenerateRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<GenerateRepr> for GenerateVal {
    type Error = ReprError;
    fn try_into(self) -> Result<GenerateRepr, Self::Error> {
        Ok(GenerateRepr::new(self.0.func.try_into()?))
    }
}
