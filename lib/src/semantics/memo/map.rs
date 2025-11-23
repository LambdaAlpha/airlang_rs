use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::fmt::Formatter;

use derive_more::IsVariant;

use super::MemoError;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::ref_::DynRef;

// todo impl arena
#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct MemoMap {
    map: Map<Key, MemoValue>,
}

// still -> (none <-> null) -> final
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, IsVariant)]
pub enum Contract {
    #[default]
    None,
    // the reference and the value are `still` unchanged in the past
    // but the reference and the value may change in the future
    Still,
    // the reference and the value are `final` in the future
    // but the reference and the value may change in the past
    Final,
    // the reference is `static` in the past and in the future
    // but the value may change in the past or in the future
    Static,
    // the reference and the value are `constant` in the past and in the future
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MemoValue {
    pub(crate) val: Val,
    pub(crate) contract: Contract,
}

impl MemoMap {
    pub(crate) fn new(map: Map<Key, MemoValue>) -> Self {
        Self { map }
    }

    pub(crate) fn unwrap(self) -> Map<Key, MemoValue> {
        self.map
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn exist(&self, name: Key) -> bool {
        self.map.get(&name).is_some()
    }

    pub fn get_ref(&self, name: Key) -> Result<&Val, MemoError> {
        let Some(value) = self.map.get(&name) else {
            return Err(MemoError::NotFound);
        };
        Ok(&value.val)
    }

    pub fn get_ref_mut(&mut self, name: Key) -> Result<&mut Val, MemoError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(MemoError::NotFound);
        };
        if !value.contract.is_mutable() {
            return Err(MemoError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    pub fn get_ref_dyn(&mut self, name: Key) -> Result<DynRef<'_, Val>, MemoError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(MemoError::NotFound);
        };
        Ok(DynRef::new(&mut value.val, !value.contract.is_mutable()))
    }

    pub fn remove(&mut self, name: Key) -> Result<Val, MemoError> {
        let Entry::Occupied(entry) = self.map.entry(name) else {
            return Err(MemoError::NotFound);
        };
        if !entry.get().contract.is_removable() {
            return Err(MemoError::AccessDenied);
        }
        Ok(entry.remove().val)
    }

    pub fn put(
        &mut self, name: Key, val: Val, contract: Contract,
    ) -> Result<Option<Val>, MemoError> {
        match self.map.entry(name) {
            Entry::Occupied(mut entry) => {
                let old = entry.get().contract;
                if !old.is_replaceable(contract) {
                    return Err(MemoError::AccessDenied);
                }
                Ok(Some(entry.insert(MemoValue::new(val, contract)).val))
            }
            Entry::Vacant(entry) => {
                if !contract.is_insertable() {
                    return Err(MemoError::AccessDenied);
                }
                entry.insert(MemoValue::new(val, contract));
                Ok(None)
            }
        }
    }

    pub fn is_assignable(&self, name: Key, contract: Contract) -> bool {
        let Some(old) = self.map.get(&name) else {
            return contract.is_insertable();
        };
        old.contract.is_replaceable(contract)
    }

    pub fn get_contract(&self, name: Key) -> Option<Contract> {
        let value = self.map.get(&name)?;
        Some(value.contract)
    }

    pub fn set_contract(&mut self, name: Key, contract: Contract) -> Result<(), MemoError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(MemoError::NotFound);
        };
        if !old.contract.is_valid_transition(contract) {
            return Err(MemoError::AccessDenied);
        }
        old.contract = contract;
        Ok(())
    }

    pub fn reverse(mut self) -> Self {
        for v in self.map.values_mut() {
            v.contract = v.contract.reverse();
        }
        self
    }

    pub(in crate::semantics) fn put_unchecked(&mut self, name: Key, val: MemoValue) -> Option<Val> {
        self.map.insert(name, val).map(|memo_value| memo_value.val)
    }

    pub(in crate::semantics) fn remove_unchecked(&mut self, name: &Key) -> Option<MemoValue> {
        self.map.remove(name)
    }
}

impl Contract {
    pub(in crate::semantics) fn is_mutable(self) -> bool {
        matches!(self, Self::None | Self::Static)
    }

    pub(in crate::semantics) fn is_removable(self) -> bool {
        matches!(self, Self::None | Self::Still)
    }

    pub(in crate::semantics) fn is_insertable(self) -> bool {
        matches!(self, Self::None | Self::Final)
    }

    pub(in crate::semantics) fn is_replaceable(self, new: Self) -> bool {
        self.is_removable() && new.is_insertable()
    }

    pub(in crate::semantics) fn is_valid_transition(self, new: Self) -> bool {
        if self == new {
            return true;
        }
        match self {
            Self::None => matches!(new, Self::Final),
            Self::Static => false,
            Self::Still => matches!(new, Self::None | Self::Final),
            Self::Final => false,
            Self::Const => false,
        }
    }

    pub(in crate::semantics) fn reverse(self) -> Self {
        match self {
            Contract::None => Contract::None,
            Contract::Still => Contract::Final,
            Contract::Final => Contract::Still,
            Contract::Static => Contract::Static,
            Contract::Const => Contract::Const,
        }
    }
}

impl MemoValue {
    pub(crate) fn new(val: Val, contract: Contract) -> Self {
        Self { val, contract }
    }
}

impl Debug for MemoValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("");
        tuple.field(&self.val);
        if self.contract != Contract::None {
            tuple.field(&self.contract);
        }
        tuple.finish()
    }
}
