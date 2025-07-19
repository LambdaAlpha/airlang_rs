use derive_more::Deref;
use derive_more::From;
use derive_more::Into;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Deref, From, Into, derive_more::Debug)]
#[debug("{_0}")]
pub struct Bit(bool);

impl Bit {
    pub fn false_() -> Bit {
        Bit(false)
    }

    pub fn true_() -> Bit {
        Bit(true)
    }

    pub fn and(self, b: Bit) -> Bit {
        Bit(self.0 && b.0)
    }

    pub fn or(self, b: Bit) -> Bit {
        Bit(self.0 || b.0)
    }

    pub fn xor(self, b: Bit) -> Bit {
        Bit(self.0 != b.0)
    }

    pub fn imply(self, b: Bit) -> Bit {
        Bit(!self.0 || b.0)
    }

    #[expect(clippy::should_implement_trait)]
    pub fn not(self) -> Bit {
        Bit(!self.0)
    }
}
