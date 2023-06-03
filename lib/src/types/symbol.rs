use {
    smartstring::alias::CompactString,
    std::ops::Deref,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Symbol(CompactString);

impl Symbol {
    pub(crate) fn from_str(s: &str) -> Self {
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
