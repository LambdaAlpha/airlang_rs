use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
    ops::{
        Deref,
        DerefMut,
    },
    rc::Rc,
};

use crate::{
    Cache,
    Case,
    Val,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaseVal {
    Trivial(TrivialCase),
    Cache(CacheCase),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TrivialCase(Rc<Case<Val, Val, Val>>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CacheCase(Rc<Cache<Val, Val, Val>>);

impl TrivialCase {
    #[allow(unused)]
    pub(crate) fn new(case: Rc<Case<Val, Val, Val>>) -> Self {
        Self(case)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<Case<Val, Val, Val>> {
        self.0
    }
}

impl From<Case<Val, Val, Val>> for TrivialCase {
    fn from(value: Case<Val, Val, Val>) -> Self {
        Self(Rc::new(value))
    }
}

impl From<TrivialCase> for Case<Val, Val, Val> {
    fn from(value: TrivialCase) -> Self {
        Rc::unwrap_or_clone(value.0)
    }
}

impl Debug for TrivialCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Case::fmt(self, f)
    }
}

impl Deref for TrivialCase {
    type Target = Case<Val, Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TrivialCase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}

impl CacheCase {
    #[allow(unused)]
    pub(crate) fn new(case: Rc<Cache<Val, Val, Val>>) -> Self {
        Self(case)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<Cache<Val, Val, Val>> {
        self.0
    }
}

impl From<Cache<Val, Val, Val>> for CacheCase {
    fn from(value: Cache<Val, Val, Val>) -> Self {
        Self(Rc::new(value))
    }
}

impl From<CacheCase> for Cache<Val, Val, Val> {
    fn from(value: CacheCase) -> Self {
        Rc::unwrap_or_clone(value.0)
    }
}

impl Debug for CacheCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Cache::fmt(self, f)
    }
}

impl Deref for CacheCase {
    type Target = Cache<Val, Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CacheCase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}

impl AsRef<Case<Val, Val, Val>> for CaseVal {
    fn as_ref(&self) -> &Case<Val, Val, Val> {
        match self {
            CaseVal::Trivial(c) => c,
            CaseVal::Cache(c) => c,
        }
    }
}
