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
};

use crate::Str;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StrVal(Box<Str>);

impl StrVal {
    #[allow(unused)]
    pub(crate) fn new(str: Box<Str>) -> Self {
        Self(str)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Str> {
        self.0
    }
}

impl From<Str> for StrVal {
    fn from(value: Str) -> Self {
        StrVal(Box::new(value))
    }
}

impl From<StrVal> for Str {
    fn from(value: StrVal) -> Self {
        *value.0
    }
}

impl From<&StrVal> for Str {
    fn from(value: &StrVal) -> Self {
        Str::clone(value)
    }
}

impl Debug for StrVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Str::fmt(self, f)
    }
}

impl Deref for StrVal {
    type Target = Str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StrVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
