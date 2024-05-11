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

use crate::Int;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct IntVal(Box<Int>);

impl IntVal {
    #[allow(unused)]
    pub(crate) fn new(int: Box<Int>) -> Self {
        Self(int)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Int> {
        self.0
    }
}

impl From<Int> for IntVal {
    fn from(value: Int) -> Self {
        IntVal(Box::new(value))
    }
}

impl From<IntVal> for Int {
    fn from(value: IntVal) -> Self {
        *value.0
    }
}

impl From<&IntVal> for Int {
    fn from(value: &IntVal) -> Self {
        Int::clone(value)
    }
}

impl Debug for IntVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Int::fmt(self, f)
    }
}

impl Deref for IntVal {
    type Target = Int;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IntVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
