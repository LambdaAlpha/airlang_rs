use std::collections::hash_map::IntoIter;
use std::collections::hash_map::IntoKeys;
use std::collections::hash_map::IntoValues;
use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;

use rustc_hash::FxHashMap;
use rustc_hash::FxHasher;

#[derive(Clone)]
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

impl<K, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    #[expect(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut Map<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    #[expect(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<K, V> Deref for Map<K, V> {
    type Target = FxHashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for Map<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        state.write_u64(
            self.0
                .iter()
                .map(|kv| {
                    let mut h = FxHasher::default();
                    kv.hash(&mut h);
                    h.finish()
                })
                .fold(0, u64::wrapping_add),
        );
    }
}

impl<K, V> Default for Map<K, V> {
    fn default() -> Self {
        Map(Default::default())
    }
}

impl<K: Debug, V: Debug> Debug for Map<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}
