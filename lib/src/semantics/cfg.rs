use std::collections::hash_map::Entry;

use const_format::concatcp;
use derive_more::Deref;
use derive_more::DerefMut;

use crate::semantics::core::PREFIX_CELL;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::utils::hint::cold_path;

// todo design invariant
#[derive(Default, Clone, PartialEq, Eq, Hash, Deref, DerefMut)]
pub struct Cfg {
    aborted: bool,
    #[deref]
    #[deref_mut]
    map: Map<Key, Val>,
}

impl Cfg {
    pub const ABORT_TYPE: &str = concatcp!(PREFIX_CELL, "error.abort.type");
    pub const ABORT_MSG: &str = concatcp!(PREFIX_CELL, "error.abort.message");

    pub const ABORT_TYPE_BUG: &str = concatcp!(PREFIX_CELL, "bug");

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

    #[inline(always)]
    pub fn step(&mut self) -> bool {
        if self.aborted {
            cold_path();
            return false;
        }
        true
    }

    #[cold]
    pub fn abort(&mut self) {
        self.aborted = true;
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
        Self { aborted: false, map }
    }
}

impl From<Cfg> for Map<Key, Val> {
    fn from(cfg: Cfg) -> Self {
        cfg.map
    }
}
