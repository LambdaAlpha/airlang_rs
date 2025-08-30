pub use self::pub_::*;

// https://github.com/rust-lang/rustfmt/issues/4070
#[macro_export]
macro_rules! _____ {
    () => {};
}

// use solve, prelude, semantics, syntax, type, trait
mod pub_;

// use semantics, syntax, type, trait
pub mod cfg;

// use prelude, semantics, type, trait
pub mod solve;

// use type, trait
pub mod semantics;

// use type, trait
pub mod syntax;

pub mod type_;

pub mod trait_;

#[expect(dead_code)]
pub(crate) mod utils;

#[cfg(test)]
pub(crate) mod test;
