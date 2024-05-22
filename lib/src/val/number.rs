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

use crate::Number;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NumberVal(Box<Number>);

impl NumberVal {
    #[allow(unused)]
    pub(crate) fn new(number: Box<Number>) -> Self {
        Self(number)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Number> {
        self.0
    }
}

impl From<Number> for NumberVal {
    fn from(value: Number) -> Self {
        NumberVal(Box::new(value))
    }
}

impl From<NumberVal> for Number {
    fn from(value: NumberVal) -> Self {
        *value.0
    }
}

impl From<&NumberVal> for Number {
    fn from(value: &NumberVal) -> Self {
        value.0.deref().clone()
    }
}

impl Debug for NumberVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Number::fmt(self, f)
    }
}

impl Deref for NumberVal {
    type Target = Number;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NumberVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
