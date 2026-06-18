use std::collections::hash_map::Entry;

use derive_more::Deref;
use derive_more::DerefMut;

use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::utils::hint::cold_path;

#[derive(Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Cfg {
    aborted: bool,
    #[deref]
    #[deref_mut]
    map: Map<Key, Val>,
}

impl Cfg {
    pub fn import(&self, key: Key) -> Option<&Val> {
        self.map.get(&key)
    }

    pub fn export(&mut self, key: Key, val: Val) -> Option<()> {
        if self.map.contains_key(&key) {
            return None;
        }
        self.map.insert(key, val);
        Some(())
    }

    pub fn extend(&mut self, key: Key, val: Val) {
        match self.map.entry(key.clone()) {
            Entry::Occupied(_) => panic!("expect a unique key, but {key} is already used"),
            Entry::Vacant(entry) => {
                entry.insert(val);
            },
        }
    }

    #[cold]
    pub fn abort(&mut self) -> Val {
        self.aborted = true;
        Val::default()
    }

    pub fn recover(&mut self) {
        self.aborted = false;
    }

    #[inline(always)]
    pub fn is_aborted(&self) -> bool {
        if self.aborted {
            cold_path();
            true
        } else {
            false
        }
    }
}

impl From<Map<Key, Val>> for Cfg {
    fn from(map: Map<Key, Val>) -> Self {
        Self { map, ..Cfg::default() }
    }
}

impl From<Cfg> for Map<Key, Val> {
    fn from(cfg: Cfg) -> Self {
        cfg.map
    }
}
