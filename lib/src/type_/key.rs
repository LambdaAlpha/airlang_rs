use derive_more::Deref;
use string_cache::Atom;
use string_cache::EmptyStaticAtomSet;

/// Note: Key and &str have different hash
#[derive(Clone, Default, PartialEq, Eq, Hash, Deref, derive_more::Debug)]
#[deref(forward)]
#[debug("'{_0}'")]
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

#[expect(clippy::to_string_trait_impl)]
impl ToString for Key {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<Key> for String {
    fn from(value: Key) -> Self {
        value.0.to_string()
    }
}
