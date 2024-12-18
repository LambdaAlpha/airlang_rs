use std::fmt::{
    Debug,
    Formatter,
};

use crate::{
    Cache,
    Case,
    Val,
    rc_wrap,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaseVal {
    Trivial(TrivialCaseVal),
    Cache(CacheCaseVal),
}

rc_wrap!(pub TrivialCaseVal(Case<Val, Val, Val>));

rc_wrap!(pub CacheCaseVal(Cache<Val, Val, Val>));

impl Debug for TrivialCaseVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Case::fmt(self, f)
    }
}

impl Debug for CacheCaseVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Cache::fmt(self, f)
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
