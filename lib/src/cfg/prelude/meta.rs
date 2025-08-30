use std::str::FromStr;

use num_bigint::BigInt;

use super::Prelude;
use super::ctx_put_val;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::ListVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::List;

#[derive(Clone)]
pub struct MetaPrelude {
    pub version: ListVal,
}

impl Default for MetaPrelude {
    fn default() -> Self {
        MetaPrelude { version: version() }
    }
}

impl Prelude for MetaPrelude {
    fn put(self, ctx: &mut Ctx) {
        ctx_put_val(ctx, "air.version", self.version);
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
