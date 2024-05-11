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

use crate::Float;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FloatVal(Box<Float>);

impl FloatVal {
    #[allow(unused)]
    pub(crate) fn new(float: Box<Float>) -> Self {
        Self(float)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Float> {
        self.0
    }
}

impl From<Float> for FloatVal {
    fn from(value: Float) -> Self {
        FloatVal(Box::new(value))
    }
}

impl From<FloatVal> for Float {
    fn from(value: FloatVal) -> Self {
        *value.0
    }
}

impl From<&FloatVal> for Float {
    fn from(value: &FloatVal) -> Self {
        value.0.deref().clone()
    }
}

impl Debug for FloatVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Float::fmt(self, f)
    }
}

impl Deref for FloatVal {
    type Target = Float;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for FloatVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
