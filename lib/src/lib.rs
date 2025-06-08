pub use pub_::*;

// https://github.com/rust-lang/rustfmt/issues/4070
#[macro_export]
macro_rules! _____ {
    () => {};
}

// use solver, prelude, semantics, syntax, type, trait
pub mod pub_;

// use prelude, semantics, type, trait
pub mod solver;

// use semantics, syntax, type, trait
pub mod prelude;

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
