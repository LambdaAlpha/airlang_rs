use std::ops::Deref;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Symbol(String);

impl Symbol {
    pub(crate) fn new(s: String) -> Self {
        Symbol(s)
    }
}

impl Deref for Symbol {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
