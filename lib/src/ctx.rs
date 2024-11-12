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
    Map,
    ctx::map::CtxMap,
    symbol::Symbol,
    val::{
        Val,
        func::CellFuncVal,
    },
};

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ctx {
    variables: CtxMap,
    solver: Option<CellFuncVal>,
}

pub(crate) struct PubCtx {
    pub(crate) variables: CtxMap,
    pub(crate) solver: Option<CellFuncVal>,
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

impl<'l> CtxRef<'l> for &'l mut Ctx {
    fn get_variables(self) -> Result<&'l CtxMap, CtxError> {
        Ok(&self.variables)
    }

    fn get_variables_mut(self) -> Result<&'l mut CtxMap, CtxError> {
        Ok(&mut self.variables)
    }

    fn get_variables_dyn(self) -> Result<DynRef<'l, CtxMap>, CtxError> {
        Ok(DynRef::new(&mut self.variables, false))
    }

    fn get_solver(self) -> Result<&'l CellFuncVal, CtxError> {
        let Some(solver) = &self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(solver)
    }

    fn get_solver_mut(self) -> Result<&'l mut CellFuncVal, CtxError> {
        let Some(solver) = &mut self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(solver)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, CellFuncVal>, CtxError> {
        let Some(solver) = &mut self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(DynRef::new(solver, false))
    }

    fn set_solver(self, solver: Option<CellFuncVal>) -> Result<(), CtxError> {
        self.solver = solver;
        Ok(())
    }
}

impl<'l> CtxRef<'l> for &'l Ctx {
    fn get_variables(self) -> Result<&'l CtxMap, CtxError> {
        Ok(&self.variables)
    }

    fn get_variables_mut(self) -> Result<&'l mut CtxMap, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_variables_dyn(self) -> Result<DynRef<'l, CtxMap>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver(self) -> Result<&'l CellFuncVal, CtxError> {
        let Some(solver) = &self.solver else {
            return Err(CtxError::NotFound);
        };
        Ok(solver)
    }

    fn get_solver_mut(self) -> Result<&'l mut CellFuncVal, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn get_solver_dyn(self) -> Result<DynRef<'l, CellFuncVal>, CtxError> {
        Err(CtxError::AccessDenied)
    }

    fn set_solver(self, _solver: Option<CellFuncVal>) -> Result<(), CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl Ctx {
    pub(crate) fn new(variables: CtxMap, solver: Option<CellFuncVal>) -> Self {
        Self { variables, solver }
    }

    pub(crate) fn destruct(self) -> PubCtx {
        PubCtx {
            variables: self.variables,
            solver: self.solver,
        }
    }

    pub(crate) fn into_val(self, name: Symbol) -> Result<Val, CtxError> {
        self.variables.into_val(name)
    }

    #[allow(unused)]
    pub(crate) fn variables(&self) -> &CtxMap {
        &self.variables
    }

    pub(crate) fn variables_mut(&mut self) -> &mut CtxMap {
        &mut self.variables
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

impl Default for Ctx {
    fn default() -> Self {
        Self {
            variables: CtxMap::new(Map::default(), false),
            solver: None,
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
