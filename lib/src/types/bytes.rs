use std::fmt::{
    Debug,
    Formatter,
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Bytes(Vec<u8>);

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Bytes(value.to_owned())
    }
}

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes(value)
    }
}

impl From<Bytes> for Vec<u8> {
    fn from(value: Bytes) -> Self {
        value.0
    }
}

impl From<&Bytes> for Vec<u8> {
    fn from(value: &Bytes) -> Self {
        value.0.clone()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Bytes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bytes({:x?})", self.0)
    }
}
