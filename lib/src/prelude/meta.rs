use std::str::FromStr;

use num_bigint::BigInt;

use crate::List;
use crate::ListVal;
use crate::Val;
use crate::int::Int;
use crate::prelude::Prelude;
use crate::prelude::PreludeCtx;
use crate::prelude::ctx_put;

#[derive(Clone)]
pub(crate) struct MetaPrelude {
    pub(crate) version: ListVal,
}

impl Default for MetaPrelude {
    fn default() -> Self {
        MetaPrelude { version: version() }
    }
}

impl Prelude for MetaPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        ctx_put(ctx, "air.version", &self.version);
    }
}

fn version() -> ListVal {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    let major = Val::Int(Int::new(BigInt::from_str(MAJOR).unwrap()).into());
    let minor = Val::Int(Int::new(BigInt::from_str(MINOR).unwrap()).into());
    let patch = Val::Int(Int::new(BigInt::from_str(PATCH).unwrap()).into());
    let list = vec![major, minor, patch];
    List::from(list).into()
}
