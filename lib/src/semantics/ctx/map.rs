use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem::take;

use super::CtxError;
use crate::semantics::val::Val;
use crate::type_::Map;
use crate::type_::Symbol;
use crate::type_::ref_::DynRef;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct CtxMap {
    map: Map<Symbol, CtxValue>,
}

// still -> (none <-> null) -> final
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Contract {
    #[default]
    None,
    // the reference is `static` in the past and in the future
    // but the value may change in the past or in the future
    Static,
    // the reference and the value are `still` unchanged in the past
    // but the reference and the value may change in the future
    Still,
    // the reference and the value are `final` in the future
    // but the reference and the value may change in the past
    Final,
    // the reference and the value are `constant` in the past and in the future
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct CtxValue {
    pub(crate) val: Val,
    pub(crate) contract: Contract,
    // lock access to the value for a period of time in the future
    pub(in crate::semantics) lock: bool,
}

impl CtxMap {
    pub(crate) fn new(map: Map<Symbol, CtxValue>) -> Self {
        Self { map }
    }

    pub(crate) fn unwrap(self) -> Map<Symbol, CtxValue> {
        self.map
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn is_null(&self, name: Symbol) -> bool {
        self.map.get(&name).is_none()
    }

    pub fn get_ref(&self, name: Symbol) -> Result<&Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        Ok(&value.val)
    }

    pub fn get_ref_mut(&mut self, name: Symbol) -> Result<&mut Val, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        if !value.contract.is_mutable() {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    pub fn get_ref_dyn(&mut self, name: Symbol) -> Result<DynRef<'_, Val>, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        Ok(DynRef::new(&mut value.val, !value.contract.is_mutable()))
    }

    pub fn remove(&mut self, name: Symbol) -> Result<Val, CtxError> {
        let Entry::Occupied(entry) = self.map.entry(name) else {
            return Err(CtxError::NotFound);
        };
        if entry.get().lock {
            return Err(CtxError::AccessDenied);
        }
        if !entry.get().contract.is_removable() {
            return Err(CtxError::AccessDenied);
        }
        Ok(entry.remove().val)
    }

    pub fn put(
        &mut self, name: Symbol, val: Val, contract: Contract,
    ) -> Result<Option<Val>, CtxError> {
        match self.map.entry(name) {
            Entry::Occupied(mut entry) => {
                if entry.get().lock {
                    return Err(CtxError::AccessDenied);
                }
                let old = entry.get().contract;
                if !old.is_replaceable(contract) {
                    return Err(CtxError::AccessDenied);
                }
                Ok(Some(entry.insert(CtxValue::new(val, contract)).val))
            }
            Entry::Vacant(entry) => {
                if !contract.is_insertable() {
                    return Err(CtxError::AccessDenied);
                }
                entry.insert(CtxValue::new(val, contract));
                Ok(None)
            }
        }
    }

    pub fn is_assignable(&self, name: Symbol, contract: Contract) -> bool {
        let Some(old) = self.map.get(&name) else {
            return contract.is_insertable();
        };
        if old.lock {
            return false;
        }
        old.contract.is_replaceable(contract)
    }

    pub fn get_contract(&self, name: Symbol) -> Option<Contract> {
        let value = self.map.get(&name)?;
        Some(value.contract)
    }

    pub fn set_contract(&mut self, name: Symbol, contract: Contract) -> Result<(), CtxError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if !old.contract.is_valid_transition(contract) {
            return Err(CtxError::AccessDenied);
        }
        old.contract = contract;
        Ok(())
    }

    pub fn is_locked(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.lock)
    }

    pub(in crate::semantics) fn lock(&mut self, name: Symbol) -> Result<CtxValue, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        value.lock = true;
        Ok(CtxValue::new(take(&mut value.val), value.contract))
    }

    pub(in crate::semantics) fn unlock(&mut self, name: Symbol, val: Val) -> Option<()> {
        let value = self.map.get_mut(&name)?;
        value.lock = false;
        value.val = val;
        Some(())
    }

    pub(in crate::semantics) fn put_unchecked(
        &mut self, name: Symbol, val: CtxValue,
    ) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(in crate::semantics) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
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
}

impl CtxValue {
    pub(crate) fn new(val: Val, contract: Contract) -> Self {
        Self { val, contract, lock: false }
    }
}

impl Debug for CtxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("");
        tuple.field(&self.val);
        if self.contract != Contract::None {
            tuple.field(&self.contract);
        }
        if self.lock {
            tuple.field(&"lock");
        }
        tuple.finish()
    }
}
