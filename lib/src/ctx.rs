use std::{
    error::Error,
    fmt::{
        Debug,
        Display,
        Formatter,
    },
    hash::Hash,
    mem::swap,
    ops::BitAnd,
};

use map::{
    CtxValue,
    DynRef,
};
use ref1::CtxRef;

use crate::{
    FuncVal,
    Map,
    ctx::map::CtxMap,
    symbol::Symbol,
};

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ctx {
    variables: CtxMap,
    solver: Option<FuncVal>,
}

pub(crate) struct PubCtx {
    pub(crate) variables: CtxMap,
    pub(crate) solver: Option<FuncVal>,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CtxAccess {
    Free,
    Const,
    #[default]
    Mut,
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

    fn set_solver(self, mut solver: Option<FuncVal>) -> Result<Option<FuncVal>, CtxError> {
        swap(&mut self.solver, &mut solver);
        Ok(solver)
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

    fn set_solver(self, _solver: Option<FuncVal>) -> Result<Option<FuncVal>, CtxError> {
        Err(CtxError::AccessDenied)
    }
}

impl Ctx {
    pub(crate) fn new(variables: CtxMap, solver: Option<FuncVal>) -> Self {
        Self { variables, solver }
    }

    pub(crate) fn destruct(self) -> PubCtx {
        PubCtx {
            variables: self.variables,
            solver: self.solver,
        }
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.variables.remove_unchecked(name)
    }

    #[allow(unused)]
    pub(crate) fn variables(&self) -> &CtxMap {
        &self.variables
    }

    pub(crate) fn variables_mut(&mut self) -> &mut CtxMap {
        &mut self.variables
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

impl BitAnd for CtxAccess {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        if self == CtxAccess::Mut || rhs == CtxAccess::Mut {
            return CtxAccess::Mut;
        }
        if self == CtxAccess::Const || rhs == CtxAccess::Const {
            return CtxAccess::Const;
        }
        CtxAccess::Free
    }
}

impl Error for CtxError {}

pub(crate) mod map;

pub(crate) mod ref1;

pub(crate) mod free;

pub(crate) mod const1;

pub(crate) mod mut1;

pub(crate) mod default;

pub(crate) mod repr;
