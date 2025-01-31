use std::str::FromStr;

use num_bigint::BigInt;

use crate::{
    Map,
    Symbol,
    ctx::map::CtxValue,
    int::Int,
    prelude::{
        Named,
        Prelude,
    },
};

#[derive(Clone)]
pub(crate) struct MetaPrelude {
    pub(crate) version_major: Named<Int>,
    pub(crate) version_minor: Named<Int>,
    pub(crate) version_patch: Named<Int>,
}

impl Default for MetaPrelude {
    fn default() -> Self {
        MetaPrelude {
            version_major: version_major(),
            version_minor: version_minor(),
            version_patch: version_patch(),
        }
    }
}

impl Prelude for MetaPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.version_major.put(m);
        self.version_minor.put(m);
        self.version_patch.put(m);
    }
}

fn version_major() -> Named<Int> {
    let id = "air.version_major";
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    let v = Int::new(BigInt::from_str(MAJOR).unwrap());
    Named::new(id, v)
}

fn version_minor() -> Named<Int> {
    let id = "air.version_minor";
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    let v = Int::new(BigInt::from_str(MINOR).unwrap());
    Named::new(id, v)
}

fn version_patch() -> Named<Int> {
    let id = "air.version_patch";
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    let v = Int::new(BigInt::from_str(PATCH).unwrap());
    Named::new(id, v)
}
