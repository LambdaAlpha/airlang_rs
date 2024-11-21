use std::{
    convert::Infallible,
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use crate::{
    Bool,
    Byte,
    Int,
    Number,
    Symbol,
    Text,
    Unit,
    syntax::{
        ParseError,
        generate_pretty,
        generator::GenerateRepr,
        parse,
        parser::ParseRepr,
        repr::{
            adapt::AdaptRepr,
            ask::AskRepr,
            call::CallRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
        },
    },
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Repr {
    Unit(Unit),
    Bool(Bool),
    Symbol(Symbol),
    Text(Text),
    Int(Int),
    Number(Number),
    Byte(Byte),
    Pair(Box<PairRepr>),
    Adapt(Box<AdaptRepr>),
    Call(Box<CallRepr>),
    Ask(Box<AskRepr>),
    List(ListRepr),
    Map(MapRepr),
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

impl From<Text> for Repr {
    fn from(t: Text) -> Self {
        Repr::Text(t)
    }
}

impl From<Int> for Repr {
    fn from(i: Int) -> Self {
        Repr::Int(i)
    }
}

impl From<Number> for Repr {
    fn from(n: Number) -> Self {
        Repr::Number(n)
    }
}

impl From<Byte> for Repr {
    fn from(b: Byte) -> Self {
        Repr::Byte(b)
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

impl From<AdaptRepr> for Repr {
    fn from(a: AdaptRepr) -> Self {
        Repr::Adapt(Box::new(a))
    }
}

impl From<Box<AdaptRepr>> for Repr {
    fn from(a: Box<AdaptRepr>) -> Self {
        Repr::Adapt(a)
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

impl Display for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate_pretty(self))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate_pretty(self))
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
        generate_pretty(value)
    }
}

impl Default for Repr {
    fn default() -> Self {
        Repr::Unit(Unit)
    }
}

impl ParseRepr for Repr {}

impl<'a> TryInto<GenerateRepr<'a, Repr>> for &'a Repr {
    type Error = Infallible;

    fn try_into(self) -> Result<GenerateRepr<'a, Repr>, Self::Error> {
        let r = match self {
            Repr::Unit(unit) => GenerateRepr::Unit(unit),
            Repr::Bool(bool) => GenerateRepr::Bool(bool),
            Repr::Symbol(symbol) => GenerateRepr::Symbol(symbol),
            Repr::Text(text) => GenerateRepr::Text(text),
            Repr::Int(int) => GenerateRepr::Int(int),
            Repr::Number(number) => GenerateRepr::Number(number),
            Repr::Byte(byte) => GenerateRepr::Byte(byte),
            Repr::Pair(pair) => GenerateRepr::Pair(pair),
            Repr::Adapt(adapt) => GenerateRepr::Adapt(adapt),
            Repr::Call(call) => GenerateRepr::Call(call),
            Repr::Ask(ask) => GenerateRepr::Ask(ask),
            Repr::List(list) => GenerateRepr::List(list),
            Repr::Map(map) => GenerateRepr::Map(map),
        };
        Ok(r)
    }
}

pub(crate) mod pair;

pub(crate) mod adapt;

pub(crate) mod call;

pub(crate) mod ask;

pub(crate) mod list;

pub(crate) mod map;
