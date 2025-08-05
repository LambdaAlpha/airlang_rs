pub use self::map::Contract;
pub use self::map::CtxMap;
pub use self::map::CtxValue;

_____!();

use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;
use std::ops::BitAnd;

use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::IsVariant;

use crate::type_::DynRef;
use crate::type_::Map;

pub trait DynCtx<Input, Output> {
    fn ref_(&mut self, input: Input) -> Option<DynRef<'_, Output>>;
}

#[derive(Copy, Clone, Debug)]
pub enum CtxError {
    NotFound,
    AccessDenied,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deref, DerefMut)]
pub struct Ctx {
    variables: CtxMap,
}

pub(crate) struct PubCtx {
    pub(crate) variables: CtxMap,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash, IsVariant)]
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

    pub(crate) fn reverse(self) -> Self {
        Self { variables: self.variables.reverse() }
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
