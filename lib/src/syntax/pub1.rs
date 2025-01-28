pub use crate::syntax::{
    error::{
        ParseError,
        ReprError,
    },
    repr::{
        Repr,
        ask::AskRepr,
        call::CallRepr,
        change::ChangeRepr,
        list::ListRepr,
        map::MapRepr,
        pair::PairRepr,
    },
};
use crate::syntax::{
    generator,
    generator::{
        COMPACT_FMT,
        PRETTY_FMT,
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
