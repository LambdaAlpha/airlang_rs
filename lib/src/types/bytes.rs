use crate::traits::TryClone;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
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

impl Into<Vec<u8>> for Bytes {
    fn into(self) -> Vec<u8> {
        self.0
    }
}

impl Into<Vec<u8>> for &Bytes {
    fn into(self) -> Vec<u8> {
        self.0.clone()
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl TryClone for Bytes {
    fn try_clone(&self) -> Option<Self>
    where
        Self: Sized,
    {
        Some(self.clone())
    }
}
