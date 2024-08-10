use crate::{
    ctx::{
        CtxValue,
        DynRef,
    },
    CtxError,
    Invariant,
    Map,
    Symbol,
    Val,
};

pub(crate) trait CtxMapRef<'a> {
    fn get_ref(self, name: Symbol) -> Result<&'a Val, CtxError>;

    fn get_ref_mut(self, name: Symbol) -> Result<&'a mut Val, CtxError>;

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'a, Val>, CtxError>;

    fn remove(self, name: Symbol) -> Result<Val, CtxError>;

    #[allow(clippy::wrong_self_convention)]
    fn is_unchecked(self) -> bool;

    #[allow(clippy::wrong_self_convention)]
    fn is_assignable(self, name: Symbol) -> bool;

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError>;

    fn set_final(self, name: Symbol) -> Result<(), CtxError>;

    #[allow(clippy::wrong_self_convention)]
    fn is_final(self, name: Symbol) -> Result<bool, CtxError>;

    fn set_const(self, name: Symbol) -> Result<(), CtxError>;

    #[allow(clippy::wrong_self_convention)]
    fn is_const(self, name: Symbol) -> Result<bool, CtxError>;
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub(crate) struct CtxMap {
    map: Map<Symbol, CtxValue>,
    unchecked: bool,
}

impl CtxMap {
    pub(crate) fn new(map: Map<Symbol, CtxValue>, unchecked: bool) -> Self {
        Self { map, unchecked }
    }

    pub(crate) fn unwrap(self) -> Map<Symbol, CtxValue> {
        self.map
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[allow(unused)]
    pub(crate) fn set_checked(&mut self) {
        self.unchecked = false;
    }

    #[allow(unused)]
    pub(crate) fn into_unchecked(mut self) -> Self {
        self.unchecked = true;
        self
    }

    fn put_unchecked(&mut self, name: Symbol, val: CtxValue) -> Option<Val> {
        self.map.insert(name, val).map(|ctx_value| ctx_value.val)
    }

    pub(crate) fn into_val(mut self, name: Symbol) -> Result<Val, CtxError> {
        let Some(ctx_value) = self.map.remove(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(ctx_value.val)
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
        if !self.unchecked && value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        Ok(&mut value.val)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        if self.map.get(&name).is_none() {
            return Err(CtxError::NotFound);
        }
        let ctx_value = self.map.get_mut(&name).unwrap();
        let is_const = !self.unchecked && ctx_value.invariant == Invariant::Const;
        Ok(DynRef::new(&mut ctx_value.val, is_const))
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        if !self.unchecked && value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.map.remove(&name).unwrap().val)
    }

    fn is_unchecked(self) -> bool {
        self.unchecked
    }

    fn is_assignable(self, name: Symbol) -> bool {
        (&*self).is_assignable(name)
    }

    fn put_value(self, name: Symbol, val: CtxValue) -> Result<Option<Val>, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Ok(self.put_unchecked(name, val));
        };
        if !self.unchecked && value.invariant != Invariant::None {
            return Err(CtxError::AccessDenied);
        }
        Ok(self.put_unchecked(name, val))
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        if !self.unchecked && value.invariant == Invariant::Const {
            return Err(CtxError::AccessDenied);
        }
        value.invariant = Invariant::Final;
        Ok(())
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        (&*self).is_final(name)
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        let Some(value) = self.map.get_mut(&name) else {
            return Err(CtxError::NotFound);
        };
        value.invariant = Invariant::Const;
        Ok(())
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        (&*self).is_const(name)
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

    fn is_unchecked(self) -> bool {
        self.unchecked
    }

    fn is_assignable(self, name: Symbol) -> bool {
        let Some(value) = self.map.get(&name) else {
            return true;
        };
        value.invariant == Invariant::None
    }

    fn put_value(self, _name: Symbol, _val: CtxValue) -> Result<Option<Val>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_final(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(value.invariant != Invariant::None)
    }

    fn set_const(self, _name: Symbol) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        let Some(value) = self.map.get(&name) else {
            return Err(CtxError::NotFound);
        };
        Ok(value.invariant == Invariant::Const)
    }
}
