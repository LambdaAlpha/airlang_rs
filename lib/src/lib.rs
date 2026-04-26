pub const VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");

pub const VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");

pub const VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");

// https://github.com/rust-lang/rustfmt/issues/4070
#[macro_export]
macro_rules! _____ {
    () => {};
}

// use semantics, syntax, type, trait
pub mod cfg;

// use type, trait
pub mod semantics;

// use type, trait
pub mod syntax;

pub mod type_;

pub mod trait_;

#[expect(dead_code)]
pub(crate) mod utils;
