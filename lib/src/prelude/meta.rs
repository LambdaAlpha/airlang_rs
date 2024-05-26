use std::str::FromStr;

use num_bigint::BigInt;

use crate::{
    ctx::CtxMap,
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
            version_major: Named::new("air.version_major", version_major()),
            version_minor: Named::new("air.version_minor", version_minor()),
            version_patch: Named::new("air.version_patch", version_patch()),
        }
    }
}

impl Prelude for MetaPrelude {
    fn put(&self, m: &mut CtxMap) {
        self.version_major.put(m);
        self.version_minor.put(m);
        self.version_patch.put(m);
    }
}

fn version_major() -> Int {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    Int::new(BigInt::from_str(MAJOR).unwrap())
}

fn version_minor() -> Int {
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    Int::new(BigInt::from_str(MINOR).unwrap())
}

fn version_patch() -> Int {
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    Int::new(BigInt::from_str(PATCH).unwrap())
}
