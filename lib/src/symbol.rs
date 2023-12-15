use {
    smartstring::alias::CompactString,
    std::{
        borrow::Borrow,
        fmt::{
            Debug,
            Formatter,
        },
        ops::Deref,
    },
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Symbol(CompactString);

impl Symbol {
    /// # Safety
    ///
    /// the input must be a valid symbol
    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        Symbol(CompactString::from(s))
    }

    pub(crate) fn from_str(s: &str) -> Self {
        Symbol(CompactString::from(s))
    }

    #[allow(unused)]
    pub(crate) fn from_string(s: String) -> Self {
        Symbol(CompactString::from(s))
    }
}

impl Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Symbol> for String {
    fn from(value: Symbol) -> Self {
        value.0.into()
    }
}

impl Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        self
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}", &self.0)
    }
}
