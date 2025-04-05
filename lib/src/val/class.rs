use crate::{
    Val,
    class::Class,
    syntax::{
        ReprError,
        repr::class::ClassRepr,
    },
    types::wrap::box_wrap,
};

box_wrap!(pub ClassVal(Class<Val>));

impl From<&ClassRepr> for ClassVal {
    fn from(value: &ClassRepr) -> Self {
        Self(Box::new(Class::new(Val::from(&value.func))))
    }
}

impl From<ClassRepr> for ClassVal {
    fn from(value: ClassRepr) -> Self {
        Self(Box::new(Class::new(Val::from(value.func))))
    }
}

impl TryInto<ClassRepr> for &ClassVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ClassRepr, Self::Error> {
        Ok(ClassRepr::new((&self.func).try_into()?))
    }
}

impl TryInto<ClassRepr> for ClassVal {
    type Error = ReprError;
    fn try_into(self) -> Result<ClassRepr, Self::Error> {
        Ok(ClassRepr::new(self.0.func.try_into()?))
    }
}
