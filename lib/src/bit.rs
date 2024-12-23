use std::fmt::{
    Debug,
    Formatter,
};

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Bit(bool);

impl Bit {
    pub fn new(b: bool) -> Self {
        Bit(b)
    }

    pub fn false1() -> Bit {
        Bit(false)
    }

    pub fn true1() -> Bit {
        Bit(true)
    }

    pub fn bool(self) -> bool {
        self.0
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

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Bit {
        Bit(!self.0)
    }
}

impl From<bool> for Bit {
    fn from(value: bool) -> Self {
        Bit(value)
    }
}

impl From<Bit> for bool {
    fn from(value: Bit) -> Self {
        value.0
    }
}

impl Debug for Bit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}
