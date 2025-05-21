use std::collections::hash_map::Entry;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::mem::take;

use crate::CtxError;
use crate::Map;
use crate::Symbol;
use crate::Val;
use crate::types::ref1::DynRef;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub(crate) struct CtxMap {
    map: Map<Symbol, CtxValue>,
    // `const` values in `forward` map are constant in the future
    // `const` values in `reverse` map are constant in the past
    reverse: bool,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxGuard {
    pub(crate) const1: bool,
    // `static` key-value binding is constant in the past and in the future
    // `static` `const` value is constant in the past and in the future
    pub(crate) static1: bool,
    // lock access to the value for a period of time in the future
    pub(crate) lock: bool,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxValue {
    pub(crate) val: Val,
    pub(crate) guard: CtxGuard,
}

impl CtxMap {
    pub(crate) fn new(map: Map<Symbol, CtxValue>, reverse: bool) -> Self {
        Self { map, reverse }
    }

    pub(crate) fn unwrap(self) -> Map<Symbol, CtxValue> {
        self.map
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub(crate) fn is_reverse(&self) -> bool {
        self.reverse
    }

    pub(crate) fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }

    pub(crate) fn get_ref(&self, name: Symbol) -> Result<&Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.guard.lock {
            return Err(CtxError::AccessDenied);
        }
        Ok(&value.val)
    }

    pub(crate) fn get_ref_mut(&mut self, name: Symbol) -> Result<&mut Val, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.guard.lock || value.guard.const1 {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    pub(crate) fn get_ref_dyn(&mut self, name: Symbol) -> Result<DynRef<Val>, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.guard.lock {
            return Err(CtxError::AccessDenied);
        }
        Ok(DynRef::new(&mut value.val, value.guard.const1))
    }

    pub(crate) fn remove(&mut self, name: Symbol) -> Result<Val, CtxError> {
        let Entry::Occupied(entry) = self.map.entry(name) else {
            return Err(CtxError::NotFound);
        };
        if entry.get().guard.lock || entry.get().guard.static1 {
            return Err(CtxError::AccessDenied);
        }
        if !self.reverse && entry.get().guard.const1 {
            return Err(CtxError::AccessDenied);
        }
        Ok(entry.remove().val)
    }

    pub(crate) fn put_value(
        &mut self, name: Symbol, val: Val, const1: bool,
    ) -> Result<Option<Val>, CtxError> {
        match self.map.entry(name) {
            Entry::Occupied(mut entry) => {
                if entry.get().guard.lock || entry.get().guard.static1 {
                    return Err(CtxError::AccessDenied);
                }
                if self.reverse {
                    if const1 {
                        return Err(CtxError::AccessDenied);
                    }
                } else {
                    if entry.get().guard.const1 {
                        return Err(CtxError::AccessDenied);
                    }
                }
                let guard = CtxGuard { const1, static1: false, lock: false };
                Ok(Some(entry.insert(CtxValue::new(val, guard)).val))
            }
            Entry::Vacant(entry) => {
                if self.reverse && const1 {
                    return Err(CtxError::AccessDenied);
                }
                let guard = CtxGuard { const1, static1: false, lock: false };
                entry.insert(CtxValue::new(val, guard));
                Ok(None)
            }
        }
    }

    pub(crate) fn set_const(&mut self, name: Symbol, const1: bool) -> Result<(), CtxError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if old.guard.static1 {
            return if old.guard.const1 == const1 { Ok(()) } else { Err(CtxError::AccessDenied) };
        }
        if self.reverse {
            if const1 && !old.guard.const1 {
                return Err(CtxError::AccessDenied);
            }
        } else {
            if !const1 && old.guard.const1 {
                return Err(CtxError::AccessDenied);
            }
        }
        old.guard.const1 = const1;
        Ok(())
    }

    pub(crate) fn is_const(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.guard.const1)
    }

    pub(crate) fn is_locked(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.guard.lock)
    }

    pub(crate) fn lock(&mut self, name: Symbol) -> Option<CtxValue> {
        let value = self.map.get_mut(&name)?;
        if value.guard.lock {
            return None;
        }
        value.guard.lock = true;
        Some(CtxValue::new(take(&mut value.val), value.guard))
    }

    pub(crate) fn unlock(&mut self, name: Symbol, val: Val) -> Option<()> {
        let value = self.map.get_mut(&name)?;
        value.guard.lock = false;
        value.val = val;
        Some(())
    }

    pub(crate) fn is_static(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.guard.static1)
    }

    pub(crate) fn is_assignable(&self, name: Symbol, new_const: bool) -> bool {
        let Some(old) = self.map.get(&name) else {
            return true;
        };
        if old.guard.lock || old.guard.static1 {
            return false;
        }
        if self.reverse { !new_const } else { !old.guard.const1 }
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.map.remove(name)
    }
}

impl CtxValue {
    pub(crate) fn new(val: Val, guard: CtxGuard) -> Self {
        Self { val, guard }
    }
}

impl Debug for CtxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("");
        tuple.field(&self.val);
        if self.guard.static1 {
            tuple.field(&"static");
        }
        if self.guard.const1 {
            tuple.field(&"const");
        }
        if self.guard.lock {
            tuple.field(&"lock");
        }
        tuple.finish()
    }
}
