pub use self::map::Contract;
pub use self::map::MemoMap;
pub use self::map::MemoValue;

_____!();

use std::error::Error;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::hash::Hash;

use derive_more::Deref;
use derive_more::DerefMut;

#[derive(Copy, Clone, Debug)]
pub enum MemoError {
    NotFound,
    AccessDenied,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, Deref, DerefMut)]
pub struct Memo {
    variables: MemoMap,
}

impl Memo {
    pub(crate) fn new(variables: MemoMap) -> Self {
        Self { variables }
    }

    pub(crate) fn unwrap(self) -> MemoMap {
        self.variables
    }

    pub(crate) fn reverse(self) -> Self {
        Self { variables: self.variables.reverse() }
    }
}

impl Display for MemoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoError::NotFound => write!(f, "not found"),
            MemoError::AccessDenied => write!(f, "access denied"),
        }
    }
}

impl Error for MemoError {}

mod map;
