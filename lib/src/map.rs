use std::{
    collections::hash_map::{
        DefaultHasher,
        IntoIter,
        IntoKeys,
        IntoValues,
        Iter,
        IterMut,
    },
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    ops::{
        Deref,
        DerefMut,
    },
};

use rustc_hash::FxHashMap;

#[derive(Clone, PartialEq, Eq)]
pub struct Map<K: Eq + Hash, V>(FxHashMap<K, V>);

#[allow(dead_code)]
pub type Set<K> = Map<K, ()>;

impl<K: Eq + Hash, V> Map<K, V> {
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

impl<K: Eq + Hash, V> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a Map<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;
    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).into_iter()
    }
}

impl<'a, K: Eq + Hash, V> IntoIterator for &'a mut Map<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;
    #[allow(clippy::into_iter_on_ref)]
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<K: Eq + Hash, V> Deref for Map<K, V> {
    type Target = FxHashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K: Eq + Hash, V> DerefMut for Map<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K: Eq + Hash, V: Hash> Hash for Map<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(
            self.0
                .iter()
                .map(|kv| {
                    let mut h = DefaultHasher::new();
                    kv.hash(&mut h);
                    h.finish()
                })
                .fold(0, u64::wrapping_add),
        );
    }
}

impl<K: Eq + Hash, V> Default for Map<K, V> {
    fn default() -> Self {
        Map(Default::default())
    }
}

impl<K: Eq + Hash + Debug, V: Debug> Debug for Map<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(&self.0, f)
    }
}
