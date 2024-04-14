use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    ops::Deref,
    rc::Rc,
};

use crate::logic::Assert;

#[derive(Clone, Eq)]
pub struct AssertVal(pub(crate) Rc<Assert>);

impl From<Rc<Assert>> for AssertVal {
    fn from(value: Rc<Assert>) -> Self {
        AssertVal(value)
    }
}

impl PartialEq for AssertVal {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        *self.0 == *other.0
    }
}

impl Hash for AssertVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

impl Debug for AssertVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}

impl Deref for AssertVal {
    type Target = Assert;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
