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

use crate::Bytes;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BytesVal(Box<Bytes>);

impl BytesVal {
    #[allow(unused)]
    pub(crate) fn new(bytes: Box<Bytes>) -> Self {
        Self(bytes)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Box<Bytes> {
        self.0
    }
}

impl From<Bytes> for BytesVal {
    fn from(value: Bytes) -> Self {
        Self(Box::new(value))
    }
}

impl From<BytesVal> for Bytes {
    fn from(value: BytesVal) -> Self {
        *value.0
    }
}

impl From<&BytesVal> for Bytes {
    fn from(value: &BytesVal) -> Self {
        Bytes::clone(value)
    }
}

impl Debug for BytesVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Bytes::fmt(self, f)
    }
}

impl Deref for BytesVal {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BytesVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
