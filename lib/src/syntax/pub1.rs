pub use crate::syntax::error::ParseError;
pub use crate::syntax::error::ReprError;
pub use crate::syntax::repr::Repr;
pub use crate::syntax::repr::call::CallRepr;
pub use crate::syntax::repr::list::ListRepr;
pub use crate::syntax::repr::map::MapRepr;
pub use crate::syntax::repr::pair::PairRepr;

// https://github.com/rust-lang/rustfmt/issues/4070
mod __ {}

use crate::syntax::generator;
use crate::syntax::generator::COMPACT_FMT;
use crate::syntax::generator::PRETTY_FMT;
use crate::syntax::generator::SYMBOL_FMT;
use crate::syntax::parser;

pub fn parse(src: &str) -> Result<Repr, ParseError> {
    parser::parse(src)
}

pub fn generate_pretty(src: &Repr) -> String {
    generator::generate(src.try_into().unwrap(), PRETTY_FMT)
}

pub fn generate_compact(src: &Repr) -> String {
    generator::generate(src.try_into().unwrap(), COMPACT_FMT)
}

pub fn generate_symbol(src: &Repr) -> String {
    generator::generate(src.try_into().unwrap(), SYMBOL_FMT)
}
