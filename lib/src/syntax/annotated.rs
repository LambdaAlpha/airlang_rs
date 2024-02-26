use std::{
    convert::Infallible,
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use crate::{
    syntax::{
        annotated::{
            annotation::AnnotatedRepr,
            call::CallRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
            reverse::ReverseRepr,
        },
        generate_all,
        generator::GenerateRepr,
        parse_all,
        parser::ParseRepr,
        ParseError,
    },
    Bool,
    Bytes,
    Float,
    Int,
    Str,
    Symbol,
    Unit,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Repr {
    Unit(Unit),
    Bool(Bool),
    Int(Int),
    Float(Float),
    Bytes(Bytes),
    Symbol(Symbol),
    String(Str),
    Pair(Box<PairRepr>),
    Call(Box<CallRepr>),
    Reverse(Box<ReverseRepr>),
    List(ListRepr),
    Map(MapRepr),
    Annotated(Box<AnnotatedRepr>),
}

impl Repr {
    pub fn is_unit(&self) -> bool {
        matches!(self, Repr::Unit(_))
    }
}

impl From<Unit> for Repr {
    fn from(u: Unit) -> Self {
        Repr::Unit(u)
    }
}

impl From<Bool> for Repr {
    fn from(b: Bool) -> Self {
        Repr::Bool(b)
    }
}

impl From<Int> for Repr {
    fn from(i: Int) -> Self {
        Repr::Int(i)
    }
}

impl From<Float> for Repr {
    fn from(f: Float) -> Self {
        Repr::Float(f)
    }
}

impl From<Bytes> for Repr {
    fn from(b: Bytes) -> Self {
        Repr::Bytes(b)
    }
}

impl From<Symbol> for Repr {
    fn from(s: Symbol) -> Self {
        Repr::Symbol(s)
    }
}

impl From<Str> for Repr {
    fn from(s: Str) -> Self {
        Repr::String(s)
    }
}

impl From<Box<PairRepr>> for Repr {
    fn from(p: Box<PairRepr>) -> Self {
        Repr::Pair(p)
    }
}

impl From<Box<CallRepr>> for Repr {
    fn from(a: Box<CallRepr>) -> Self {
        Repr::Call(a)
    }
}

impl From<Box<ReverseRepr>> for Repr {
    fn from(i: Box<ReverseRepr>) -> Self {
        Repr::Reverse(i)
    }
}

impl From<ListRepr> for Repr {
    fn from(l: ListRepr) -> Self {
        Repr::List(l)
    }
}

impl From<MapRepr> for Repr {
    fn from(m: MapRepr) -> Self {
        Repr::Map(m)
    }
}

impl From<Box<AnnotatedRepr>> for Repr {
    fn from(a: Box<AnnotatedRepr>) -> Self {
        Repr::Annotated(a)
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate_all(self))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate_all(self))
    }
}

impl TryFrom<&str> for Repr {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse_all(value)
    }
}

impl FromStr for Repr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_all(s)
    }
}

impl From<&Repr> for String {
    fn from(value: &Repr) -> Self {
        generate_all(value)
    }
}

impl Default for Repr {
    fn default() -> Self {
        Repr::Unit(Unit)
    }
}

impl ParseRepr for Repr {
    fn try_into_pair(self) -> Result<(Self, Self), Self> {
        match self {
            Repr::Pair(pair) => Ok((pair.first, pair.second)),
            other => Err(other),
        }
    }
}

impl<'a> TryInto<GenerateRepr<'a, Repr>> for &'a Repr {
    type Error = Infallible;

    fn try_into(self) -> Result<GenerateRepr<'a, Repr>, Self::Error> {
        let r = match self {
            Repr::Unit(u) => GenerateRepr::Unit(u),
            Repr::Bool(b) => GenerateRepr::Bool(b),
            Repr::Int(i) => GenerateRepr::Int(i),
            Repr::Float(f) => GenerateRepr::Float(f),
            Repr::Bytes(b) => GenerateRepr::Bytes(b),
            Repr::Symbol(s) => GenerateRepr::Symbol(s),
            Repr::String(s) => GenerateRepr::String(s),
            Repr::Pair(p) => GenerateRepr::Pair(p),
            Repr::Call(c) => GenerateRepr::Call(c),
            Repr::Reverse(r) => GenerateRepr::Reverse(r),
            Repr::List(l) => GenerateRepr::List(l),
            Repr::Map(m) => GenerateRepr::Map(m),
            Repr::Annotated(a) => GenerateRepr::Annotated(a),
        };
        Ok(r)
    }
}

pub(crate) mod pair;

pub(crate) mod call;

pub(crate) mod reverse;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod annotation;
