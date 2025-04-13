pub use crate::syntax::{
    error::{
        ParseError,
        ReprError,
    },
    repr::{
        Repr,
        abstract1::AbstractRepr,
        call::CallRepr,
        change::ChangeRepr,
        generate::GenerateRepr,
        inverse::InverseRepr,
        list::ListRepr,
        map::MapRepr,
        pair::PairRepr,
        reify::ReifyRepr,
    },
};

// https://github.com/rust-lang/rustfmt/issues/4070
mod __ {}

use crate::syntax::{
    generator,
    generator::{
        COMPACT_FMT,
        PRETTY_FMT,
        SYMBOL_FMT,
    },
    parser,
};

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
