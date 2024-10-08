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
            ask::AskRepr,
            call::CallRepr,
            comment::CommentRepr,
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
    Int(Int),
    Number(Number),
    Text(Text),
    Pair(Box<PairRepr>),
    List(ListRepr),
    Map(MapRepr),
    Byte(Byte),
    Call(Box<CallRepr>),
    Ask(Box<AskRepr>),
    Comment(Box<CommentRepr>),
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

impl From<Number> for Repr {
    fn from(n: Number) -> Self {
        Repr::Number(n)
    }
}

impl From<Text> for Repr {
    fn from(t: Text) -> Self {
        Repr::Text(t)
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

impl From<CommentRepr> for Repr {
    fn from(a: CommentRepr) -> Self {
        Repr::Comment(Box::new(a))
    }
}

impl From<Byte> for Repr {
    fn from(b: Byte) -> Self {
        Repr::Byte(b)
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
            Repr::Unit(u) => GenerateRepr::Unit(u),
            Repr::Bool(b) => GenerateRepr::Bool(b),
            Repr::Symbol(s) => GenerateRepr::Symbol(s),
            Repr::Int(i) => GenerateRepr::Int(i),
            Repr::Number(n) => GenerateRepr::Number(n),
            Repr::Text(t) => GenerateRepr::Text(t),
            Repr::Pair(p) => GenerateRepr::Pair(p),
            Repr::List(l) => GenerateRepr::List(l),
            Repr::Map(m) => GenerateRepr::Map(m),
            Repr::Byte(b) => GenerateRepr::Byte(b),
            Repr::Call(c) => GenerateRepr::Call(c),
            Repr::Ask(a) => GenerateRepr::Ask(a),
            Repr::Comment(a) => GenerateRepr::Comment(a),
        };
        Ok(r)
    }
}

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod call;

pub(crate) mod ask;

pub(crate) mod comment;
