use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    CtxError,
    Map,
    Symbol,
    Val,
};

#[allow(clippy::wrong_self_convention)]
pub(crate) trait CtxMapRef<'a>: Sized {
    fn is_reverse(self) -> bool;

    fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError>;

    fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError>;

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'a, Val>, CtxError>;

    fn remove(self, name: Symbol) -> Result<Val, CtxError>;

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError>;

    fn set_invariant(self, name: Symbol, invariant: Invariant) -> Result<(), CtxError>;

    fn get_invariant(self, name: Symbol) -> Option<Invariant>;

    fn is_static(self, name: Symbol) -> Option<bool>;

    fn is_assignable(self, name: Symbol) -> bool {
        let Some(invariant) = self.get_invariant(name) else {
            return true;
        };
        invariant == Invariant::None
    }
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub(crate) struct CtxMap {
    map: Map<Symbol, CtxValue>,
    // the invariants of normal map are hold in the future
    // the invariants of reverse map are hold in the past
    reverse: bool,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Invariant {
    // no limit
    #[default]
    None,
    // can't be assigned
    Final,
    // can't be modified
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxValue {
    pub(crate) invariant: Invariant,
    // the invariant of static binding is hold both in the past and in the future
    // corollaries
    // - static binding either always exists or never exists
    // - the invariant of static binding is constant
    pub(crate) static1: bool,
    pub(crate) val: Val,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct DynRef<'a, T> {
    pub(crate) ref1: &'a mut T,
    pub(crate) is_const: bool,
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

    pub(crate) fn set_reverse(&mut self, reverse: bool) {
        self.reverse = reverse;
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.map.remove(name)
    }
}

impl<'l> CtxMapRef<'l> for &'l mut CtxMap {
    fn is_reverse(self) -> bool {
        (&*self).is_reverse()
    }

    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        (&*self).get_ref(name)
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        if self.map.get(&name).is_none() {
            return Err(CtxError::NotFound);
        }
        let ctx_value = self.map.get_mut(&name).unwrap();
        let is_const = ctx_value.invariant == Invariant::Const;
        Ok(DynRef::new(&mut ctx_value.val, is_const))
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if value.static1 {
            return Err(CtxError::AccessDenied);
        }
        if !self.reverse && value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.map.remove(&name).unwrap().val)
    }

    // ignore static field of CtxValue
    fn put_value(self, name: Symbol, mut new: CtxValue) -> Result<Option<Val>, CtxError> {
        debug_assert!(!new.static1);
        let Some(old) = self.map.get(&name) else {
            if self.reverse && new.invariant != Invariant::None {
                return Err(CtxError::AccessDenied);
            }
            new.static1 = false;
            return Ok(self.put_unchecked(name, new));
        };
        if old.static1 {
            if old.invariant != Invariant::None || new.invariant != Invariant::None {
                return Err(CtxError::AccessDenied);
            }
            new.static1 = true;
            return Ok(self.put_unchecked(name, new));
        }
        #[allow(clippy::collapsible_else_if)]
        if self.reverse {
            if new.invariant != Invariant::None {
                return Err(CtxError::AccessDenied);
            }
        } else {
            if old.invariant != Invariant::None {
                return Err(CtxError::AccessDenied);
            }
        }
        new.static1 = false;
        Ok(self.put_unchecked(name, new))
    }

    fn set_invariant(self, name: Symbol, new: Invariant) -> Result<(), CtxError> {
        let Some(old) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if old.static1 {
            if old.invariant != new {
                return Err(CtxError::AccessDenied);
            }
            return Ok(());
        }
        #[allow(clippy::collapsible_else_if)]
        if self.reverse {
            if new != Invariant::None && old.invariant == Invariant::None
                || new == Invariant::Const && old.invariant != Invariant::Const
            {
                return Err(CtxError::AccessDenied);
            }
        } else {
            if new == Invariant::None && old.invariant != Invariant::None
                || new != Invariant::Const && old.invariant == Invariant::Const
            {
                return Err(CtxError::AccessDenied);
            }
        }
        old.invariant = new;
        Ok(())
    }

    fn get_invariant(self, name: Symbol) -> Option<Invariant> {
        (&*self).get_invariant(name)
    }

    fn is_static(self, name: Symbol) -> Option<bool> {
        (&*self).is_static(name)
    }
}

impl<'l> CtxMapRef<'l> for &'l CtxMap {
    fn is_reverse(self) -> bool {
        self.reverse
    }

    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        let Some(tagged_val) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(&tagged_val.val)
    }

    fn get_ref_mut(self, _name: Symbol) -> Result<&'l mut Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_ref_dyn(self, _name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn remove(self, _name: Symbol) -> Result<Val, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn put_value(self, _name: Symbol, _val: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_invariant(self, _name: Symbol, _invariant: Invariant) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_invariant(self, name: Symbol) -> Option<Invariant> {
        let value = self.map.get(&name)?;
        Some(value.invariant)
    }

    fn is_static(self, name: Symbol) -> Option<bool> {
        let value = self.map.get(&name)?;
        Some(value.static1)
    }
}

#[allow(unused)]
impl CtxValue {
    pub(crate) fn new(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::None,
            static1: false,
            val,
        }
    }

    pub(crate) fn new_final(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::Final,
            static1: false,
            val,
        }
    }

    pub(crate) fn new_const(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::Const,
            static1: false,
            val,
        }
    }
}

impl Debug for CtxValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.invariant)
            .field(&self.val)
            .finish()
    }
}

impl<'a, T> DynRef<'a, T> {
    pub(crate) fn new(ref1: &'a mut T, is_const: bool) -> Self {
        DynRef { ref1, is_const }
    }

    pub(crate) fn as_const(&'a self) -> &'a T {
        self.ref1
    }

    pub(crate) fn as_mut(&'a mut self) -> Option<&'a mut T> {
        if self.is_const {
            None
        } else {
            Some(&mut self.ref1)
        }
    }
}
