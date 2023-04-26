use crate::{
    semantics::val::Val,
    types::{
        Int,
        Str,
    },
};

pub(crate) fn version_code() -> Val {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    let version_code = format!("{:0>3}{:0>3}{:0>3}", MAJOR, MINOR, PATCH);
    Val::Int(Int::from_sign_string_radix(true, &version_code, 10))
}

pub(crate) fn version_name() -> Val {
    Val::String(Str::from(env!("CARGO_PKG_VERSION")))
}
