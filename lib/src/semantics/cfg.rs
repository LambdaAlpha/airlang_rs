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
use crate::type_::Map;
use crate::type_::Text;

// todo design invariant
#[derive(Clone, PartialEq, Eq)]
pub struct Cfg {
    steps: u128,
    aborted: bool,
    max_scope: usize,
    // box is required for StableDeref, which is required for insert
    map: OnceMap<Key, Box<OnceMap<usize /*scope*/, Box<Val>>>>,
}

#[derive(Default, Deref, DerefMut)]
struct OnceMap<K, V>(once_map::unsync::OnceMap<K, V, FxBuildHasher>);

impl Cfg {
    pub const ABORT_TYPE: &str = "_error.abort.type";
    pub const ABORT_MSG: &str = "_error.abort.message";

    pub const ABORT_TYPE_STEPS: &str = concatcp!(PREFIX_ID, "steps");
    pub const ABORT_TYPE_BUG: &str = concatcp!(PREFIX_ID, "bug");

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

    pub fn snapshot(&self) -> Map<Key, Val> {
        self.map
            .read_only_view()
            .iter()
            .map(|(k, scopes)| {
                let max_scope =
                    *scopes.read_only_view().keys().max().expect("scopes should not be empty");
                let val = scopes.get(&max_scope).unwrap().clone();
                (k.clone(), val)
            })
            .collect()
    }

    #[inline(always)]
    pub fn step(&mut self) -> bool {
        if self.aborted {
            return false;
        }
        if self.steps == 0 {
            self.export(
                Key::from_str_unchecked(Self::ABORT_TYPE),
                Val::Key(Key::from_str_unchecked(Self::ABORT_TYPE_STEPS)),
            );
            self.export(
                Key::from_str_unchecked(Self::ABORT_MSG),
                Val::Text(Text::from("out of steps").into()),
            );
            self.aborted = true;
            return false;
        }
        self.steps -= 1;
        true
    }

    pub fn set_steps(&mut self, n: u128) -> bool {
        if n > self.steps {
            return false;
        }
        self.steps = n;
        true
    }

    pub fn steps(&self) -> u128 {
        self.steps
    }

    pub fn abort(&mut self) {
        self.aborted = true;
    }

    pub fn recover(&mut self) {
        self.steps = u128::MAX;
        self.aborted = false;
    }

    pub fn is_aborted(&self) -> bool {
        self.aborted
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self { steps: u128::MAX, aborted: false, max_scope: 0, map: OnceMap::default() }
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
