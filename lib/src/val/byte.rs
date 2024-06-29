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

use crate::Byte;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ByteVal(Box<Byte>);

impl ByteVal {
    #[allow(unused)]
    pub(crate) fn new(byte: Box<Byte>) -> Self {
        Self(byte)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Byte> {
        self.0
    }
}

impl From<Byte> for ByteVal {
    fn from(value: Byte) -> Self {
        Self(Box::new(value))
    }
}

impl From<ByteVal> for Byte {
    fn from(value: ByteVal) -> Self {
        *value.0
    }
}

impl From<&ByteVal> for Byte {
    fn from(value: &ByteVal) -> Self {
        Byte::clone(value)
    }
}

impl Debug for ByteVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Byte::fmt(self, f)
    }
}

impl Deref for ByteVal {
    type Target = Byte;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ByteVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
