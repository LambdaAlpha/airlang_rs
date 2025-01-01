use crate::{
    CtxError,
    Invariant,
    Map,
    Symbol,
    Val,
    ctx::{
        CtxValue,
        DynRef,
    },
};

pub(crate) trait CtxMapRef<'a>: Sized {
    fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError>;

    fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError>;

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'a, Val>, CtxError>;

    fn remove(self, name: Symbol) -> Result<Val, CtxError>;

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError>;

    fn set_invariant(self, name: Symbol, invariant: Invariant) -> Result<(), CtxError>;

    fn get_invariant(self, name: Symbol) -> Option<Invariant>;

    fn fallback(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
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
    fallback: bool,
}

impl CtxMap {
    pub(crate) fn new(map: Map<Symbol, CtxValue>, fallback: bool) -> Self {
        Self { map, fallback }
    }

    pub(crate) fn unwrap(self) -> Map<Symbol, CtxValue> {
        self.map
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub(crate) fn set_fallback(&mut self, fallback: bool) {
        self.fallback = fallback;
    }

    pub(crate) fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.map.remove(name)
    }
}

impl<'l> CtxMapRef<'l> for &'l mut CtxMap {
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
        if value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.map.remove(&name).unwrap().val)
    }

    fn put_value(self, name: Symbol, val: CtxValue) -> Result<Option<Val>, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Ok(self.put_unchecked(name, val));
        };
        if value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.put_unchecked(name, val))
    }

    fn set_invariant(self, name: Symbol, invariant: Invariant) -> Result<(), CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if !self.fallback
            && (invariant == Invariant::None && value.invariant != Invariant::None
                || invariant != Invariant::Const && value.invariant == Invariant::Const)
        {
            return Err(CtxError::AccessDenied);
        }
        value.invariant = invariant;
        Ok(())
    }

    fn get_invariant(self, name: Symbol) -> Option<Invariant> {
        (&*self).get_invariant(name)
    }

    fn fallback(self) -> bool {
        (&*self).fallback()
    }
}

impl<'l> CtxMapRef<'l> for &'l CtxMap {
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

    fn fallback(self) -> bool {
        self.fallback
    }
}
