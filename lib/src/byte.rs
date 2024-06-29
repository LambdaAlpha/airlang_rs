use std::fmt::{
    Debug,
    Formatter,
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Byte(Vec<u8>);

impl From<&[u8]> for Byte {
    fn from(value: &[u8]) -> Self {
        Byte(value.to_owned())
    }
}

impl From<Vec<u8>> for Byte {
    fn from(value: Vec<u8>) -> Self {
        Byte(value)
    }
}

impl From<Byte> for Vec<u8> {
    fn from(value: Byte) -> Self {
        value.0
    }
}

impl From<&Byte> for Vec<u8> {
    fn from(value: &Byte) -> Self {
        value.0.clone()
    }
}

impl AsRef<[u8]> for Byte {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Byte({:02x?})", self.0)
    }
}
