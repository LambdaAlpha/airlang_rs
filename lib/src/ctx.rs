use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::ops::BitAnd;

use map::CtxValue;
use map::DynRef;
use ref1::CtxRef;

use crate::Map;
use crate::ctx::map::CtxMap;
use crate::symbol::Symbol;

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ctx {
    variables: CtxMap,
}

pub(crate) struct PubCtx {
    pub(crate) variables: CtxMap,
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
}

impl Ctx {
    pub(crate) fn new(variables: CtxMap) -> Self {
        Self { variables }
    }

    pub(crate) fn destruct(self) -> PubCtx {
        PubCtx { variables: self.variables }
    }

    pub(crate) fn remove_unchecked(&mut self, name: &Symbol) -> Option<CtxValue> {
        self.variables.remove_unchecked(name)
    }

    pub(crate) fn variables(&self) -> &CtxMap {
        &self.variables
    }

    pub(crate) fn variables_mut(&mut self) -> &mut CtxMap {
        &mut self.variables
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self { variables: CtxMap::new(Map::default(), false) }
    }
}

impl Display for CtxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CtxError::NotFound => write!(f, "not found"),
            CtxError::AccessDenied => write!(f, "access denied"),
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

pub(crate) mod main;

pub(crate) mod repr;

pub(crate) mod pattern;
