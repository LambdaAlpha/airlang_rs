use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
    rc::Rc,
};

use crate::logic::Assert;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AssertVal(Rc<Assert>);

impl AssertVal {
    #[allow(unused)]
    pub(crate) fn new(assert: Rc<Assert>) -> Self {
        Self(assert)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<Assert> {
        self.0
    }
}

impl From<Assert> for AssertVal {
    fn from(value: Assert) -> Self {
        Self(Rc::new(value))
    }
}

impl From<AssertVal> for Assert {
    fn from(value: AssertVal) -> Self {
        Rc::unwrap_or_clone(value.0)
    }
}

impl Debug for AssertVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Assert::fmt(self, f)
    }
}

impl Deref for AssertVal {
    type Target = Assert;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AssertVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}
