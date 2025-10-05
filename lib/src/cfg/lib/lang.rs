use std::str::FromStr;

use num_bigint::BigInt;

use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ListVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct LangLib {
    pub version: ListVal,
}

impl Default for LangLib {
    fn default() -> Self {
        LangLib { version: version() }
    }
}

impl CfgMod for LangLib {
    fn extend(self, cfg: &Cfg) {
        cfg.extend_scope(Symbol::from_str_unchecked("language.version"), Val::List(self.version));
    }
}

pub fn version() -> ListVal {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    let major = Val::Int(Int::new(BigInt::from_str(MAJOR).unwrap()).into());
    let minor = Val::Int(Int::new(BigInt::from_str(MINOR).unwrap()).into());
    let patch = Val::Int(Int::new(BigInt::from_str(PATCH).unwrap()).into());
    let list = vec![major, minor, patch];
    List::from(list).into()
}
