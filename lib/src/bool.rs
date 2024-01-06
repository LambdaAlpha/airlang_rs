use std::fmt::{
    Debug,
    Formatter,
};

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct Bool(bool);

impl Bool {
    pub fn new(b: bool) -> Self {
        Bool(b)
    }

    pub fn f() -> Bool {
        Bool(false)
    }

    pub fn t() -> Bool {
        Bool(true)
    }

    pub fn bool(&self) -> bool {
        self.0
    }

    pub fn and(self, b: Bool) -> Bool {
        Bool(self.0 && b.0)
    }

    pub fn or(self, b: Bool) -> Bool {
        Bool(self.0 || b.0)
    }

    pub fn xor(self, b: Bool) -> Bool {
        Bool(self.0 != b.0)
    }

    pub fn imply(self, b: Bool) -> Bool {
        Bool(!self.0 || b.0)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn not(self) -> Bool {
        Bool(!self.0)
    }
}

impl From<&bool> for Bool {
    fn from(value: &bool) -> Self {
        Bool(*value)
    }
}

impl From<bool> for Bool {
    fn from(value: bool) -> Self {
        Bool(value)
    }
}

impl From<Bool> for bool {
    fn from(value: Bool) -> Self {
        value.0
    }
}

impl From<&Bool> for bool {
    fn from(value: &Bool) -> Self {
        value.0
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}
