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
    Trivial(TrivialCaseVal),
    Cache(CacheCaseVal),
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TrivialCaseVal(Rc<Case<Val, Val, Val>>);

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CacheCaseVal(Rc<Cache<Val, Val, Val>>);

impl TrivialCaseVal {
    #[allow(unused)]
    pub(crate) fn new(case: Rc<Case<Val, Val, Val>>) -> Self {
        Self(case)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<Case<Val, Val, Val>> {
        self.0
    }
}

impl From<Case<Val, Val, Val>> for TrivialCaseVal {
    fn from(value: Case<Val, Val, Val>) -> Self {
        Self(Rc::new(value))
    }
}

impl From<TrivialCaseVal> for Case<Val, Val, Val> {
    fn from(value: TrivialCaseVal) -> Self {
        Rc::unwrap_or_clone(value.0)
    }
}

impl Debug for TrivialCaseVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Case::fmt(self, f)
    }
}

impl Deref for TrivialCaseVal {
    type Target = Case<Val, Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TrivialCaseVal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        Rc::make_mut(&mut self.0)
    }
}

impl CacheCaseVal {
    #[allow(unused)]
    pub(crate) fn new(case: Rc<Cache<Val, Val, Val>>) -> Self {
        Self(case)
    }

    #[allow(unused)]
    pub(crate) fn unwrap(self) -> Rc<Cache<Val, Val, Val>> {
        self.0
    }
}

impl From<Cache<Val, Val, Val>> for CacheCaseVal {
    fn from(value: Cache<Val, Val, Val>) -> Self {
        Self(Rc::new(value))
    }
}

impl From<CacheCaseVal> for Cache<Val, Val, Val> {
    fn from(value: CacheCaseVal) -> Self {
        Rc::unwrap_or_clone(value.0)
    }
}

impl Debug for CacheCaseVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Cache::fmt(self, f)
    }
}

impl Deref for CacheCaseVal {
    type Target = Cache<Val, Val, Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CacheCaseVal {
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
