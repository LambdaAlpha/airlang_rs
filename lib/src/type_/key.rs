use derive_more::Deref;
use string_cache::Atom;
use string_cache::EmptyStaticAtomSet;

/// Note: Key and &str have different hash
#[derive(Clone, Default, PartialEq, Eq, Hash, Deref)]
#[deref(forward)]
pub struct Key(Atom<EmptyStaticAtomSet>);

impl Key {
    pub const MIN: char = ' ';
    pub const MAX: char = '~';

    pub fn from_str_unchecked(s: &str) -> Self {
        Key(Atom::from(s))
    }

    pub fn from_string_unchecked(s: String) -> Self {
        Key(Atom::from(s))
    }

    pub(crate) fn is_key(c: char) -> bool {
        Self::MIN <= c && c <= Self::MAX
    }
}
