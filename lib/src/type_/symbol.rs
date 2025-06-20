use std::borrow::Borrow;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Deref;

use string_cache::Atom;
use string_cache::EmptyStaticAtomSet;

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct Symbol(Atom<EmptyStaticAtomSet>);

impl Symbol {
    pub const MIN: char = ' ';
    pub const MAX: char = '~';

    pub fn from_str_unchecked(s: &str) -> Self {
        Symbol(Atom::from(s))
    }

    pub fn from_string_unchecked(s: String) -> Self {
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

#[expect(clippy::to_string_trait_impl)]
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
