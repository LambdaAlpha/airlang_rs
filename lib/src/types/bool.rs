#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
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

    pub fn and(&self, b: &Bool) -> Bool {
        Bool(self.0 && b.0)
    }

    pub fn or(&self, b: &Bool) -> Bool {
        Bool(self.0 || b.0)
    }

    pub fn not(&self) -> Bool {
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

impl Into<bool> for Bool {
    fn into(self) -> bool {
        self.0
    }
}

impl Into<bool> for &Bool {
    fn into(self) -> bool {
        self.0
    }
}
