use std::str::FromStr;

use num_bigint::BigInt;

use super::Library;
use crate::cfg::CfgMod;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::ListVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Symbol;

#[derive(Clone)]
pub struct MetaLib {
    pub version: ListVal,
}

impl Default for MetaLib {
    fn default() -> Self {
        MetaLib { version: version() }
    }
}

impl CfgMod for MetaLib {
    fn extend(self, cfg: &Cfg) {
        cfg.extend_scope(Symbol::from_str_unchecked("air.version"), Val::List(self.version));
    }
}

impl Library for MetaLib {
    fn prelude(&self, _ctx: &mut Ctx) {}
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
