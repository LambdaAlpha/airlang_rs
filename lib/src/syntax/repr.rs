use std::{
    convert::Infallible,
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use crate::{
    annotation::Annotation,
    bool::Bool,
    bytes::Bytes,
    float::Float,
    int::Int,
    string::Str,
    symbol::Symbol,
    syntax::{
        generate,
        generator::GenerateRepr,
        parse,
        parser::ParseRepr,
        repr::{
            ask::AskRepr,
            call::CallRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
        },
        ParseError,
    },
    unit::Unit,
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Repr {
    Unit(Unit),
    Bool(Bool),
    Symbol(Symbol),
    Int(Int),
    Float(Float),
    String(Str),
    Pair(Box<PairRepr>),
    List(ListRepr),
    Map(MapRepr),
    Bytes(Bytes),
    Call(Box<CallRepr>),
    Ask(Box<AskRepr>),
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

impl From<Symbol> for Repr {
    fn from(s: Symbol) -> Self {
        Repr::Symbol(s)
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

impl From<Str> for Repr {
    fn from(s: Str) -> Self {
        Repr::String(s)
    }
}

impl From<PairRepr> for Repr {
    fn from(p: PairRepr) -> Self {
        Repr::Pair(Box::new(p))
    }
}

impl From<Box<PairRepr>> for Repr {
    fn from(p: Box<PairRepr>) -> Self {
        Repr::Pair(p)
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

impl From<Annotation<Repr, Repr>> for Repr {
    fn from(a: Annotation<Repr, Repr>) -> Self {
        a.value
    }
}

impl From<Bytes> for Repr {
    fn from(b: Bytes) -> Self {
        Repr::Bytes(b)
    }
}

impl From<CallRepr> for Repr {
    fn from(c: CallRepr) -> Self {
        Repr::Call(Box::new(c))
    }
}

impl From<Box<CallRepr>> for Repr {
    fn from(c: Box<CallRepr>) -> Self {
        Repr::Call(c)
    }
}

impl From<AskRepr> for Repr {
    fn from(a: AskRepr) -> Self {
        Repr::Ask(Box::new(a))
    }
}

impl From<Box<AskRepr>> for Repr {
    fn from(a: Box<AskRepr>) -> Self {
        Repr::Ask(a)
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate(self))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate(self))
    }
}

impl TryFrom<&str> for Repr {
    type Error = ParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        parse(value)
    }
}

impl FromStr for Repr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl From<&Repr> for String {
    fn from(value: &Repr) -> Self {
        generate(value)
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
            Repr::Ask(a) => GenerateRepr::Ask(a),
            Repr::List(l) => GenerateRepr::List(l),
            Repr::Map(m) => GenerateRepr::Map(m),
        };
        Ok(r)
    }
}

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod call;

pub(crate) mod ask;
