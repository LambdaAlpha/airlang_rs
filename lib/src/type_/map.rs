use std::collections::hash_map::IntoKeys;
use std::collections::hash_map::IntoValues;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::Hasher;

use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::IntoIterator;
use rustc_hash::FxBuildHasher;
use rustc_hash::FxHashMap;

#[derive(Clone, IntoIterator, Deref, DerefMut, derive_more::Debug)]
#[into_iterator(owned, ref, ref_mut)]
#[debug("{_0:?}")]
pub struct Map<K, V>(FxHashMap<K, V>);

impl<K, V> Map<K, V> {
    pub(crate) fn with_capacity(len: usize) -> Self {
        Map(FxHashMap::with_capacity_and_hasher(len, Default::default()))
    }

    pub(crate) fn into_keys(self) -> IntoKeys<K, V> {
        self.0.into_keys()
    }

    pub(crate) fn into_values(self) -> IntoValues<K, V> {
        self.0.into_values()
    }
}

impl<K: Eq + Hash, V> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Map(FxHashMap::from_iter(iter))
    }
}

impl<K: Eq + Hash, V> Extend<(K, V)> for Map<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

impl<K: Eq + Hash, V: PartialEq> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<K: Eq + Hash, V: Eq> Eq for Map<K, V> {}

impl<K: Hash, V: Hash> Hash for Map<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let hash = self.iter().map(|kv| FxBuildHasher.hash_one(kv)).fold(0, u64::wrapping_add);
        state.write_u64(hash);
    }
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Map(Default::default())
    }
}
