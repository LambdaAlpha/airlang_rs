use std::str::FromStr;

use num_bigint::BigInt;

use crate::{
    List,
    ListVal,
    Map,
    Symbol,
    Val,
    ctx::map::CtxValue,
    int::Int,
    prelude::{
        Named,
        Prelude,
    },
};

#[derive(Clone)]
pub(crate) struct MetaPrelude {
    pub(crate) version: Named<ListVal>,
}

impl Default for MetaPrelude {
    fn default() -> Self {
        MetaPrelude { version: version() }
    }
}

impl Prelude for MetaPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.version.put(m);
    }
}

fn version() -> Named<ListVal> {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    let id = "air.version";
    let major = Val::Int(Int::new(BigInt::from_str(MAJOR).unwrap()).into());
    let minor = Val::Int(Int::new(BigInt::from_str(MINOR).unwrap()).into());
    let patch = Val::Int(Int::new(BigInt::from_str(PATCH).unwrap()).into());
    let list = vec![major, minor, patch];
    let list = List::from(list).into();
    Named::new(id, list)
}
