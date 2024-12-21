use std::fmt::Debug;

use crate::{
    Cache,
    Case,
    Val,
    types::wrap::rc_wrap,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaseVal {
    Trivial(TrivialCaseVal),
    Cache(CacheCaseVal),
}

rc_wrap!(pub TrivialCaseVal(Case<Val, Val, Val>));

rc_wrap!(pub CacheCaseVal(Cache<Val, Val, Val>));

impl AsRef<Case<Val, Val, Val>> for CaseVal {
    fn as_ref(&self) -> &Case<Val, Val, Val> {
        match self {
            CaseVal::Trivial(c) => c,
            CaseVal::Cache(c) => c,
        }
    }
}
