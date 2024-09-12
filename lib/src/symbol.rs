use std::{
    borrow::Borrow,
    fmt::{
        Debug,
        Formatter,
    },
    ops::Deref,
};

use string_cache::{
    Atom,
    EmptyStaticAtomSet,
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Symbol(Atom<EmptyStaticAtomSet>);

impl Symbol {
    pub(crate) const MIN: char = ' ';
    pub(crate) const MAX: char = '~';
    pub(crate) const ID_PREFIX: char = '.';

    /// # Safety
    ///
    /// the input must be a valid symbol
    pub unsafe fn from_str_unchecked(s: &str) -> Self {
        Symbol(Atom::from(s))
    }

    pub(crate) fn from_str(s: &str) -> Self {
        Symbol(Atom::from(s))
    }

    pub(crate) fn from_string(s: String) -> Self {
        Symbol(Atom::from(s))
    }

    pub(crate) fn is_symbol(c: char) -> bool {
        Self::MIN <= c && c <= Self::MAX
    }
}

impl Deref for Symbol {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Symbol {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Symbol> for String {
    fn from(value: Symbol) -> Self {
        value.0.to_string()
    }
}

impl Borrow<str> for Symbol {
    fn borrow(&self) -> &str {
        self
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", &self.0)
    }
}
