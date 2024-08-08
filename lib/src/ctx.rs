use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    hash::Hash,
};

use ref1::CtxRef;

use crate::{
    ctx::map::{
        CtxMap,
        CtxMapRef,
    },
    symbol::Symbol,
    val::Val,
    FuncVal,
};

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
    Unexpected,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash)]
pub struct Ctx {
    pub(crate) variables: CtxMap,
    pub(crate) solver: Option<FuncVal>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Invariant {
    // no limit
    None,
    // can't be assigned
    Final,
    // can't be modified
    Const,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub(crate) struct CtxValue {
    pub(crate) invariant: Invariant,
    pub(crate) val: Val,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub(crate) struct DynRef<'a, T> {
    pub(crate) ref1: &'a mut T,
    pub(crate) is_const: bool,
}

impl<'l> CtxMapRef<'l> for &'l mut Ctx {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        (&mut self.variables).get_ref(name)
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        (&mut self.variables).get_ref_mut(name)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        (&mut self.variables).get_ref_dyn(name)
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        (&mut self.variables).remove(name)
    }

    fn is_assignable(self, name: Symbol) -> bool {
        (&mut self.variables).is_assignable(name)
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        (&mut self.variables).put_value(name, value)
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        (&mut self.variables).set_final(name)
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        (&mut self.variables).is_final(name)
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        (&mut self.variables).set_const(name)
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        (&mut self.variables).is_const(name)
    }
}

impl<'l> CtxRef<'l> for &'l mut Ctx {
    fn get_solver(self) -> Result<&'l FuncVal, CtxError> {
        let Some(solver) = &self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(solver)
    }

    fn get_solver_mut(self) -> Result<&'l mut FuncVal, CtxError> {
        let Some(solver) = &mut self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(solver)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, FuncVal>, CtxError> {
        let Some(solver) = &mut self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(DynRef::new(solver, false))
    }

    fn set_solver(self, solver: Option<FuncVal>) -> Result<(), CtxError> {
        self.solver = solver;
        Ok(())
    }
}

impl<'l> CtxMapRef<'l> for &'l Ctx {
    fn get_ref(self, name: Symbol) -> Result<&'l Val, CtxError> {
        self.variables.get_ref(name)
    }

    fn get_ref_mut(self, name: Symbol) -> Result<&'l mut Val, CtxError> {
        self.variables.get_ref_mut(name)
    }

    fn get_ref_dyn(self, name: Symbol) -> Result<DynRef<'l, Val>, CtxError> {
        self.variables.get_ref_dyn(name)
    }

    fn remove(self, name: Symbol) -> Result<Val, CtxError> {
        self.variables.remove(name)
    }

    fn is_assignable(self, name: Symbol) -> bool {
        self.variables.is_assignable(name)
    }

    fn put_value(self, name: Symbol, value: CtxValue) -> Result<Option<Val>, CtxError> {
        self.variables.put_value(name, value)
    }

    fn set_final(self, name: Symbol) -> Result<(), CtxError> {
        self.variables.set_final(name)
    }

    fn is_final(self, name: Symbol) -> Result<bool, CtxError> {
        self.variables.is_final(name)
    }

    fn set_const(self, name: Symbol) -> Result<(), CtxError> {
        self.variables.set_const(name)
    }

    fn is_const(self, name: Symbol) -> Result<bool, CtxError> {
        self.variables.is_const(name)
    }
}

impl<'l> CtxRef<'l> for &'l Ctx {
    fn get_solver(self) -> Result<&'l FuncVal, CtxError> {
        let Some(solver) = &self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(solver)
    }

    fn get_solver_mut(self) -> Result<&'l mut FuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, FuncVal>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_solver(self, _solver: Option<FuncVal>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl Ctx {
    pub(crate) fn new(variables: CtxMap, solver: Option<FuncVal>) -> Self {
        Self { variables, solver }
    }

    pub(crate) fn into_val(self, name: Symbol) -> Result<Val, CtxError> {
        self.variables.into_val(name)
    }
}

#[allow(unused)]
impl CtxValue {
    pub(crate) fn new(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::None,
            val,
        }
    }

    pub(crate) fn new_final(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::Final,
            val,
        }
    }

    pub(crate) fn new_const(val: Val) -> CtxValue {
        CtxValue {
            invariant: Invariant::Const,
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

impl Display for CtxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CtxError::NotFound => {
                write!(f, "not found")
            }
            CtxError::AccessDenied => {
                write!(f, "access denied")
            }
            CtxError::Unexpected => {
                write!(f, "unexpected")
            }
        }
    }
}

impl Error for CtxError {}

pub(crate) mod map;

pub(crate) mod ref1;

pub(crate) mod free;

pub(crate) mod const1;

pub(crate) mod mut1;

pub(crate) mod default;
