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
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxGuard {
    pub(crate) const1: bool,
    // `static` key-value binding is constant in the past and in the future
    // `static` `const` value is constant in the past and in the future
    pub(crate) static1: bool,
    // `forward` `const` value is constant in the future
    // `reverse` `const` value is constant in the past
    pub(crate) reverse: bool,
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct OptCtxGuard {
    pub(crate) const1: Option<bool>,
    pub(crate) static1: Option<bool>,
    pub(crate) reverse: Option<bool>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxValue {
    pub(crate) val: Val,
    pub(crate) guard: CtxGuard,
    // lock access to the value for a period of time in the future
    pub(crate) lock: bool,
}

impl CtxMap {
    pub(crate) fn new(map: Map<Symbol, CtxValue>) -> Self {
        Self { map }
    }

    pub(crate) fn unwrap(self) -> Map<Symbol, CtxValue> {
        self.map
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub(crate) fn is_null(&self, name: Symbol) -> bool {
        self.map.get(&name).is_none()
    }

    pub(crate) fn get_ref(&self, name: Symbol) -> Result<&Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        Ok(&value.val)
    }

    pub(crate) fn get_ref_mut(&mut self, name: Symbol) -> Result<&mut Val, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        if value.guard.const1 {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    pub(crate) fn get_ref_dyn(&mut self, name: Symbol) -> Result<DynRef<Val>, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        Ok(DynRef::new(&mut value.val, value.guard.const1))
    }

    pub(crate) fn remove(&mut self, name: Symbol) -> Result<Val, CtxError> {
        let Entry::Occupied(entry) = self.map.entry(name) else {
            return Err(CtxError::NotFound);
        };
        if entry.get().lock {
            return Err(CtxError::AccessDenied);
        }
        if !entry.get().guard.is_removable() {
            return Err(CtxError::AccessDenied);
        }
        Ok(entry.remove().val)
    }

    pub(crate) fn put(
        &mut self, name: Symbol, val: Val, guard: OptCtxGuard,
    ) -> Result<Option<Val>, CtxError> {
        match self.map.entry(name) {
            Entry::Occupied(mut entry) => {
                if entry.get().lock {
                    return Err(CtxError::AccessDenied);
                }
                let old_guard = entry.get().guard;
                let new_guard = guard.update(old_guard);
                if !old_guard.is_replaceable(new_guard) {
                    return Err(CtxError::AccessDenied);
                }
                Ok(Some(entry.insert(CtxValue::new(val, new_guard)).val))
            }
            Entry::Vacant(entry) => {
                let guard = guard.unwrap_or_default();
                if !guard.is_insertable() {
                    return Err(CtxError::AccessDenied);
                }
                entry.insert(CtxValue::new(val, guard));
                Ok(None)
            }
        }
    }

    pub(crate) fn is_assignable(&self, name: Symbol, guard: OptCtxGuard) -> bool {
        let Some(old) = self.map.get(&name) else {
            return guard.unwrap_or_default().is_insertable();
        };
        if old.lock {
            return false;
        }
        old.guard.is_replaceable(guard.update(old.guard))
    }

    pub(crate) fn is_const(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.guard.const1)
    }

    pub(crate) fn set_const(&mut self, name: Symbol, const1: bool) -> Result<(), CtxError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if !old.guard.is_const_updatable(const1) {
            return Err(CtxError::AccessDenied);
        }
        old.guard.const1 = const1;
        Ok(())
    }

    pub(crate) fn is_reverse(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.guard.reverse)
    }

    pub(crate) fn set_reverse(&mut self, name: Symbol, reverse: bool) -> Result<(), CtxError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if !old.guard.is_reverse_updatable(reverse) {
            return Err(CtxError::AccessDenied);
        }
        old.guard.reverse = reverse;
        Ok(())
    }

    pub(crate) fn is_locked(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.lock)
    }

    pub(crate) fn lock(&mut self, name: Symbol) -> Result<CtxValue, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.lock {
            return Err(CtxError::AccessDenied);
        }
        value.lock = true;
        Ok(CtxValue::new(take(&mut value.val), value.guard))
    }

    pub(crate) fn unlock(&mut self, name: Symbol, val: Val) -> Option<()> {
        let value = self.map.get_mut(&name)?;
        value.lock = false;
        value.val = val;
        Some(())
    }

    pub(crate) fn is_static(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.guard.static1)
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.map.remove(name)
    }
}

impl CtxGuard {
    #[expect(dead_code)]
    pub(crate) fn new_nonstatic(const1: bool, reverse: bool) -> Self {
        Self { const1, static1: false, reverse }
    }

    pub(crate) fn new_static(const1: bool) -> Self {
        Self { const1, static1: true, reverse: false }
    }

    fn is_removable(self) -> bool {
        if self.static1 {
            return false;
        }
        if !self.const1 {
            return true;
        }
        self.reverse
    }

    fn is_insertable(self) -> bool {
        if self.static1 {
            return false;
        }
        if !self.const1 {
            return true;
        }
        !self.reverse
    }

    fn is_replaceable(self, new: CtxGuard) -> bool {
        if self.static1 || new.static1 {
            return false;
        }
        match (self.reverse, new.reverse) {
            (false, false) => !self.const1,
            (true, true) => !new.const1,
            (false, true) => !self.const1 && !new.const1,
            (true, false) => true,
        }
    }

    fn is_const_updatable(self, const1: bool) -> bool {
        if self.const1 == const1 {
            return true;
        }
        if self.static1 {
            return false;
        }
        if self.reverse { !const1 } else { !self.const1 }
    }

    fn is_reverse_updatable(self, reverse: bool) -> bool {
        if self.reverse == reverse {
            return true;
        }
        if self.static1 {
            return true;
        }
        if !self.const1 {
            return true;
        }
        self.reverse
    }
}

impl OptCtxGuard {
    pub(crate) fn update(self, mut guard: CtxGuard) -> CtxGuard {
        if let Some(const1) = self.const1 {
            guard.const1 = const1;
        }
        if let Some(static1) = self.static1 {
            guard.static1 = static1;
        }
        if let Some(reverse) = self.reverse {
            guard.reverse = reverse;
        }
        guard
    }

    pub(crate) fn unwrap_or_default(self) -> CtxGuard {
        CtxGuard {
            const1: self.const1.unwrap_or_default(),
            static1: self.static1.unwrap_or_default(),
            reverse: self.reverse.unwrap_or_default(),
        }
    }
}

impl CtxValue {
    pub(crate) fn new(val: Val, guard: CtxGuard) -> Self {
        Self { val, guard, lock: false }
    }
}

impl Debug for CtxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tuple = f.debug_tuple("");
        tuple.field(&self.val);
        if self.guard.static1 {
            tuple.field(&"static");
        }
        if self.guard.reverse {
            tuple.field(&"reverse");
        }
        if self.guard.const1 {
            tuple.field(&"const");
        }
        if self.lock {
            tuple.field(&"lock");
        }
        tuple.finish()
    }
}
