use std::collections::hash_map::Entry;

use const_format::concatcp;
use derive_more::Deref;
use derive_more::DerefMut;

use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Text;

// todo design invariant
#[derive(Clone, PartialEq, Eq, Deref, DerefMut)]
pub struct Cfg {
    steps: u128,
    aborted: bool,
    #[deref]
    #[deref_mut]
    map: Map<Key, Val>,
}

impl Cfg {
    pub const ABORT_TYPE: &str = "_error.abort.type";
    pub const ABORT_MSG: &str = "_error.abort.message";

    pub const ABORT_TYPE_STEPS: &str = concatcp!(PREFIX_ID, "steps");
    pub const ABORT_TYPE_BUG: &str = concatcp!(PREFIX_ID, "bug");

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

impl From<Map<Key, Val>> for Cfg {
    fn from(map: Map<Key, Val>) -> Self {
        Self { steps: u128::MAX, aborted: false, map }
    }
}

impl From<Cfg> for Map<Key, Val> {
    fn from(cfg: Cfg) -> Self {
        cfg.map
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self { steps: u128::MAX, aborted: false, map: Map::default() }
    }
}
