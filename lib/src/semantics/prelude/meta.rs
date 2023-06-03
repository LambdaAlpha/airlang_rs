use crate::{
    semantics::val::Val,
    types::Int,
};

pub(crate) fn version_major() -> Val {
    const MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
    Val::Int(Int::from_sign_string_radix(true, MAJOR, 10))
}

pub(crate) fn version_minor() -> Val {
    const MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
    Val::Int(Int::from_sign_string_radix(true, MINOR, 10))
}

pub(crate) fn version_patch() -> Val {
    const PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
    Val::Int(Int::from_sign_string_radix(true, PATCH, 10))
}
