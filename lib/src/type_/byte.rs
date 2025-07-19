use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use derive_more::Into;

#[derive(Clone, Default, PartialEq, Eq, Hash, From, Into, Deref, DerefMut, derive_more::Debug)]
#[deref(forward)]
pub struct Byte(#[debug("{_0:02x?}")] Vec<u8>);

impl Byte {
    pub(crate) fn push(&mut self, byte: &[u8]) {
        self.0.extend_from_slice(byte);
    }
}
