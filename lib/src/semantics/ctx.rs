pub use map::Contract;
pub use map::CtxMap;
pub use map::CtxValue;

_____!();

use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::ops::BitAnd;

use crate::type_::Map;

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

impl Ctx {
    pub(crate) fn new(variables: CtxMap) -> Self {
        Self { variables }
    }

    pub(crate) fn destruct(self) -> PubCtx {
        PubCtx { variables: self.variables }
    }

    pub fn variables(&self) -> &CtxMap {
        &self.variables
    }

    pub fn variables_mut(&mut self) -> &mut CtxMap {
        &mut self.variables
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self { variables: CtxMap::new(Map::default()) }
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

mod map;
