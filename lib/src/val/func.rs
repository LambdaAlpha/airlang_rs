use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::{
        Hash,
        Hasher,
    },
    ops::Deref,
};

use crate::{
    func::Func,
    types::refer::Reader,
};

#[derive(Clone, Eq)]
pub struct FuncVal(pub(crate) Reader<Func>);

impl From<Reader<Func>> for FuncVal {
    fn from(value: Reader<Func>) -> Self {
        FuncVal(value)
    }
}

impl PartialEq for FuncVal {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }
        *self.0 == *other.0
    }
}

impl Hash for FuncVal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.deref().hash(state);
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        <_ as Debug>::fmt(self.0.deref(), f)
    }
}

impl Deref for FuncVal {
    type Target = Func;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
