use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::From;
use derive_more::Into;
use derive_more::IntoIterator;

#[derive(
    Clone, PartialEq, Eq, Hash, IntoIterator, Deref, DerefMut, From, Into, derive_more::Debug,
)]
#[into_iterator(owned, ref, ref_mut)]
#[debug("{_0:?}")]
pub struct List<T>(Vec<T>);

impl<T> Default for List<T> {
    fn default() -> Self {
        Self(Vec::default())
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        List(Vec::from_iter(iter))
    }
}
