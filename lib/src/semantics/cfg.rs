use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

use const_format::concatcp;
use derive_more::Deref;
use derive_more::DerefMut;
use rustc_hash::FxBuildHasher;
use stable_deref_trait::StableDeref;

use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::Val;
use crate::type_::Key;

// todo design invariant
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cfg {
    steps: u128,
    aborted: bool,
    abort_reason: Key,
    max_scope: usize,
    // box is required for StableDeref, which is required for insert
    map: OnceMap<Key, Box<OnceMap<usize /*scope*/, Box<Val>>>>,
}

#[derive(Debug, Default, Deref, DerefMut)]
struct OnceMap<K, V>(once_map::unsync::OnceMap<K, V, FxBuildHasher>);

impl Cfg {
    pub const ABORT_REASON_STEPS_EXCEED: &str = concatcp!(PREFIX_ID, "steps_exceed");

    pub fn begin_scope(&mut self) {
        self.max_scope += 1;
    }

    pub fn end_scope(&mut self) {
        let mut to_remove = Vec::new();
        for (k, v) in self.map.iter_mut() {
            v.remove(&self.max_scope);
            if v.is_empty() {
                to_remove.push(k.clone());
            }
        }
        for k in to_remove {
            self.map.remove(&k);
        }
        self.max_scope -= 1;
    }

    pub fn extend_scope(&self, name: Key, val: Val) -> Option<()> {
        let scopes = self.map.insert(name, |_| Box::default());
        if scopes.contains_key(&self.max_scope) {
            return None;
        }
        scopes.insert(self.max_scope, |_| Box::new(val));
        Some(())
    }

    pub fn exist(&self, name: Key) -> bool {
        self.map.get(&name).is_some()
    }

    pub fn import(&self, name: Key) -> Option<Val> {
        let scopes = self.map.get(&name)?;
        let max_scope = *scopes.read_only_view().keys().max().expect("scopes should not be empty");
        let val = scopes.get(&max_scope).unwrap().clone();
        Some(val)
    }

    pub fn export(&self, name: Key, val: Val) -> Option<()> {
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

    pub fn snapshot(&self) -> Self {
        let map = self
            .map
            .read_only_view()
            .iter()
            .map(|(k, scopes)| {
                let max_scope =
                    *scopes.read_only_view().keys().max().expect("scopes should not be empty");
                let val = scopes.get(&max_scope).unwrap().clone();
                let new_scopes = OnceMap(Default::default());
                new_scopes.insert(0usize, |_| Box::new(val));
                (k.clone(), Box::new(new_scopes))
            })
            .collect();
        Self { steps: u128::MAX, aborted: false, abort_reason: Key::default(), max_scope: 0, map }
    }

    #[inline(always)]
    pub fn step(&mut self) -> bool {
        if self.aborted {
            return false;
        }
        if self.steps == 0 {
            self.aborted = true;
            self.abort_reason = Key::from_str_unchecked(Self::ABORT_REASON_STEPS_EXCEED);
            return false;
        }
        self.steps -= 1;
        true
    }

    pub(crate) fn set_steps_unchecked(&mut self, n: u128) {
        self.steps = n;
    }

    pub fn steps(&self) -> u128 {
        self.steps
    }

    pub fn abort(&mut self, reason: Key) {
        self.aborted = true;
        self.abort_reason = reason;
    }

    pub(crate) fn resume(&mut self) {
        self.aborted = false;
        self.abort_reason = Key::default();
    }

    pub fn is_aborted(&self) -> bool {
        self.aborted
    }

    pub fn abort_reason(&self) -> Key {
        self.abort_reason.clone()
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            steps: u128::MAX,
            aborted: false,
            abort_reason: Key::default(),
            max_scope: 0,
            map: OnceMap::default(),
        }
    }
}

impl IntoIterator for Cfg {
    type Item = (Key, Box<dyn Iterator<Item = (usize, Val)>>);
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

impl<K, V> FromIterator<(K, V)> for OnceMap<K, V>
where K: Eq + Hash
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(FromIterator::from_iter(iter))
    }
}
