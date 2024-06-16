use std::{
    fmt::{
        Debug,
        Formatter,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use crate::{
    annotate::Annotate,
    syntax::repr::annotate::AnnotateRepr,
    ReprError,
    Val,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AnnotateVal(Box<Annotate<Val, Val>>);

impl AnnotateVal {
    #[allow(unused)]
    pub(crate) fn new(annotate: Box<Annotate<Val, Val>>) -> Self {
        Self(annotate)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Annotate<Val, Val>> {
        self.0
    }
}

impl From<Annotate<Val, Val>> for AnnotateVal {
    fn from(value: Annotate<Val, Val>) -> Self {
        Self(Box::new(value))
    }
}

impl From<AnnotateVal> for Annotate<Val, Val> {
    fn from(value: AnnotateVal) -> Self {
        *value.0
    }
}

impl From<&AnnotateRepr> for AnnotateVal {
    fn from(value: &AnnotateRepr) -> Self {
        let annotate = Annotate::new(Val::from(&value.note), Val::from(&value.value));
        Self(Box::new(annotate))
    }
}

impl From<AnnotateRepr> for AnnotateVal {
    fn from(value: AnnotateRepr) -> Self {
        let annotate = Annotate::new(Val::from(value.note), Val::from(value.value));
        Self(Box::new(annotate))
    }
}

impl TryInto<AnnotateRepr> for &AnnotateVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AnnotateRepr, Self::Error> {
        Ok(AnnotateRepr::new(
            (&self.note).try_into()?,
            (&self.value).try_into()?,
        ))
    }
}

impl TryInto<AnnotateRepr> for AnnotateVal {
    type Error = ReprError;
    fn try_into(self) -> Result<AnnotateRepr, Self::Error> {
        let annotate = AnnotateRepr::new(self.0.note.try_into()?, self.0.value.try_into()?);
        Ok(annotate)
    }
}

impl Debug for AnnotateVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Annotate::fmt(self, f)
    }
}

impl Deref for AnnotateVal {
    type Target = Annotate<Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AnnotateVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
