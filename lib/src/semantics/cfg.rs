use std::fmt::Debug;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;

use derive_more::Deref;
use derive_more::DerefMut;
use rustc_hash::FxBuildHasher;
use stable_deref_trait::StableDeref;

use crate::semantics::val::Val;
use crate::type_::Symbol;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Cfg {
    max_scope: usize,
    // box is required for StableDeref, which is required for insert
    map: OnceMap<Symbol, Box<OnceMap<usize /*scope*/, Box<Val>>>>,
}

#[derive(Debug, Default, Deref, DerefMut)]
struct OnceMap<K, V>(once_map::unsync::OnceMap<K, V, FxBuildHasher>);

impl Cfg {
    pub fn begin_scope(&mut self) {
        self.max_scope += 1;
    }

    pub fn end_scope(&mut self) {
        for v in self.map.values_mut() {
            v.remove(&self.max_scope);
        }
        self.max_scope -= 1;
    }

    pub fn extend_scope(&self, name: Symbol, val: Val) -> Option<()> {
        let scopes = self.map.insert(name, |_| Box::default());
        if scopes.contains_key(&self.max_scope) {
            return None;
        }
        scopes.insert(self.max_scope, |_| Box::new(val));
        Some(())
    }

    pub fn import(&self, name: Symbol) -> Option<Val> {
        let scopes = self.map.get(&name)?;
        let max_scope = *scopes.read_only_view().keys().max()?;
        scopes.get(&max_scope).cloned()
    }

    pub fn export(&self, name: Symbol, val: Val) -> Option<()> {
        let scopes = self.map.insert(name, |_| Box::default());
        let min_scope = (0 ..= self.max_scope).into_iter().find(|i| !scopes.contains_key(i))?;
        scopes.insert(min_scope, |_| Box::new(val));
        Some(())
    }

    pub fn scope_level(&self) -> usize {
        self.max_scope
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }
}

impl IntoIterator for Cfg {
    type Item = (Symbol, Box<dyn Iterator<Item = (usize, Val)>>);
    type IntoIter = Box<dyn Iterator<Item = Self::Item>>;
    fn into_iter(self) -> Self::IntoIter {
        let iter = self.map.0.into_iter().map(|(k, v)| {
            let iter = (*v).0.into_iter().map(|(k, v)| (k, *v));
            let v: Box<dyn Iterator<Item = (usize, Val)>> = Box::new(iter);
            (k, v)
        });
        Box::new(iter)
    }
}

impl<K, V> Clone for OnceMap<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        let view = self.read_only_view();
        let iter = view.iter().map(|(k, v)| (K::clone(k), V::clone(v)));
        let map = once_map::unsync::OnceMap::from_iter(iter);
        Self(map)
    }
}

impl<K, V> PartialEq for OnceMap<K, V>
where
    K: Eq + Hash,
    V: StableDeref,
    <V as Deref>::Target: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.read_only_view()
            .iter()
            .all(|(key, value)| other.get(key).is_some_and(|v| **value == *v))
    }
}

impl<K, V> Eq for OnceMap<K, V>
where
    K: Eq + Hash,
    V: StableDeref,
    <V as Deref>::Target: PartialEq,
{
}

impl<K: Hash, V: Hash> Hash for OnceMap<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let view = self.read_only_view();
        let hash = view.iter().map(|kv| view.hasher().hash_one(kv)).fold(0, u64::wrapping_add);
        state.write_u64(hash);
    }
}
