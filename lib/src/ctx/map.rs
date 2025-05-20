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
    // the invariants of normal map are hold in the future
    // the invariants of reverse map are hold in the past
    reverse: bool,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum VarAccess {
    #[default]
    Assign,
    Mut,
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxValue {
    pub(crate) access: VarAccess,
    // the invariant of static binding is hold both in the past and in the future
    // corollaries
    // - static binding either always exists or never exists
    // - the invariant of static binding is constant
    pub(crate) static1: bool,
    // make val inaccessible
    pub(crate) free: bool,
    pub(crate) val: Val,
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
        let Some(ctx_value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if ctx_value.free {
            return Err(CtxError::AccessDenied);
        }
        Ok(&ctx_value.val)
    }

    pub(crate) fn get_ref_mut(&mut self, name: Symbol) -> Result<&mut Val, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.free || value.access == VarAccess::Const {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    pub(crate) fn get_ref_dyn(&mut self, name: Symbol) -> Result<DynRef<Val>, CtxError> {
        if self.map.get(&name).is_none() {
            return Err(CtxError::NotFound);
        }
        let ctx_value = self.map.get_mut(&name).unwrap();
        if ctx_value.free {
            return Err(CtxError::AccessDenied);
        }
        let is_const = ctx_value.access == VarAccess::Const;
        Ok(DynRef::new(&mut ctx_value.val, is_const))
    }

    pub(crate) fn remove(&mut self, name: Symbol) -> Result<Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.static1 {
            return Err(CtxError::AccessDenied);
        }
        if !self.reverse && value.access != VarAccess::Assign {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.map.remove(&name).unwrap().val)
    }

    pub(crate) fn put_value(
        &mut self, name: Symbol, access: VarAccess, val: Val,
    ) -> Result<Option<Val>, CtxError> {
        let Some(old) = self.map.get(&name) else {
            if self.reverse && access != VarAccess::Assign {
                return Err(CtxError::AccessDenied);
            }
            let ctx_value = CtxValue { access, val, static1: false, free: false };
            return Ok(self.put_unchecked(name, ctx_value));
        };
        if old.free {
            return Err(CtxError::AccessDenied);
        }
        if old.static1 {
            if old.access != VarAccess::Assign || access != VarAccess::Assign {
                return Err(CtxError::AccessDenied);
            }
            let ctx_value = CtxValue { access, val, static1: true, free: false };
            return Ok(self.put_unchecked(name, ctx_value));
        }
        if self.reverse {
            if access != VarAccess::Assign {
                return Err(CtxError::AccessDenied);
            }
        } else {
            if old.access != VarAccess::Assign {
                return Err(CtxError::AccessDenied);
            }
        }
        let ctx_value = CtxValue { access, val, static1: false, free: false };
        Ok(self.put_unchecked(name, ctx_value))
    }

    pub(crate) fn set_access(&mut self, name: Symbol, new: VarAccess) -> Result<(), CtxError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if old.static1 {
            return if old.access == new { Ok(()) } else { Err(CtxError::AccessDenied) };
        }
        if self.reverse {
            if new != VarAccess::Assign && old.access == VarAccess::Assign
                || new == VarAccess::Const && old.access != VarAccess::Const
            {
                return Err(CtxError::AccessDenied);
            }
        } else {
            if new == VarAccess::Assign && old.access != VarAccess::Assign
                || new != VarAccess::Const && old.access == VarAccess::Const
            {
                return Err(CtxError::AccessDenied);
            }
        }
        old.access = new;
        Ok(())
    }

    pub(crate) fn get_access(&self, name: Symbol) -> Option<VarAccess> {
        let value = self.map.get(&name)?;
        Some(value.access)
    }

    pub(crate) fn is_accessible(&self, name: Symbol) -> Option<bool> {
        let ctx_value = self.map.get(&name)?;
        Some(!ctx_value.free)
    }

    pub(crate) fn set_inaccessible(&mut self, name: Symbol) -> Option<CtxValue> {
        let ctx_value = self.map.get_mut(&name)?;
        if ctx_value.free {
            return None;
        }
        ctx_value.free = true;
        let val = take(&mut ctx_value.val);
        Some(CtxValue { val, ..*ctx_value })
    }

    pub(crate) fn set_accessible(&mut self, name: Symbol, val: Val) -> Option<()> {
        let ctx_value = self.map.get_mut(&name)?;
        ctx_value.free = false;
        ctx_value.val = val;
        Some(())
    }

    pub(crate) fn is_static(&self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.static1)
    }

    pub(crate) fn is_assignable(&self, name: Symbol, new_access: VarAccess) -> bool {
        let Some(old) = self.map.get(&name) else {
            return true;
        };
        if old.free {
            return false;
        }
        if self.reverse { new_access == VarAccess::Assign } else { old.access == VarAccess::Assign }
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.map.remove(name)
    }
}

#[expect(dead_code)]
impl CtxValue {
    pub(crate) fn new_assign(val: Val) -> CtxValue {
        CtxValue { val, access: VarAccess::Assign, static1: false, free: false }
    }

    pub(crate) fn new_mut(val: Val) -> CtxValue {
        CtxValue { val, access: VarAccess::Mut, static1: false, free: false }
    }

    pub(crate) fn new_const(val: Val) -> CtxValue {
        CtxValue { val, access: VarAccess::Const, static1: false, free: false }
    }
}

impl Debug for CtxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("").field(&self.access).field(&self.val).finish()
    }
}
