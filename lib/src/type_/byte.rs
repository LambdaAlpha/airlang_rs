use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use derive_more::Into;

#[derive(Clone, Default, PartialEq, Eq, Hash, From, Into, Deref, DerefMut)]
#[deref(forward)]
pub struct Byte(Vec<u8>);

impl Byte {
    pub(crate) fn push(&mut self, byte: &[u8]) {
        self.0.extend_from_slice(byte);
    }
}
