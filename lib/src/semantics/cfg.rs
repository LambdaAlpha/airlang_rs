use std::collections::hash_map::Entry;

use const_format::concatcp;
use derive_more::Deref;
use derive_more::DerefMut;

use crate::semantics::cfg::fact::Facts;
use crate::semantics::cfg::fact::ValId;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::DynFunc;
use crate::semantics::val::FuncVal;
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
    facts: Facts,
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

    pub(crate) fn facts(&self) -> &Facts {
        &self.facts
    }

    pub fn fact_put(&mut self, ctx: &mut Val, func: FuncVal, input: Val) {
        let output = func.call(self, ctx, input.clone());
        let input = ValId::from(input);
        let output = ValId::from(output);
        self.facts.put(func, input, output);
    }

    pub fn fact_call(&self, func: FuncVal, input: Val) -> Option<Val> {
        let outputs = self.facts.call(func, ValId::from(input))?;
        if outputs.is_empty() {
            return None;
        }
        Some(Val::clone(&outputs[0]))
    }

    pub fn fact_solve(&self, func: FuncVal, output: Val) -> Option<Val> {
        let inputs = self.facts.solve(func, ValId::from(output))?;
        if inputs.is_empty() {
            return None;
        }
        Some(Val::clone(&inputs[0]))
    }

    pub fn fact_exist(&self, func: FuncVal, input: Val, output: Val) -> bool {
        let input = ValId::from(input);
        let output = ValId::from(output);
        self.facts.exist(func, input, output)
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

pub(crate) mod fact;
