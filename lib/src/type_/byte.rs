use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Byte(Vec<u8>);

impl Byte {
    pub(crate) fn push(&mut self, byte: &[u8]) {
        self.0.extend_from_slice(byte);
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

impl Deref for Byte {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Byte {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for Byte {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Byte({:02x?})", self.0)
    }
}
