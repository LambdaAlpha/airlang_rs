use std::{
    convert::Infallible,
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use crate::{
    Abstract,
    Ask,
    Bit,
    Byte,
    Call,
    Int,
    Number,
    Pair,
    Symbol,
    Text,
    Unit,
    change::Change,
    syntax::{
        ParseError,
        generate_pretty,
        generator::GenRepr,
        parse,
        parser::ParseRepr,
        repr::{
            abstract1::AbstractRepr,
            ask::AskRepr,
            call::CallRepr,
            change::ChangeRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
        },
    },
};

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum Repr {
    Unit(Unit),
    Bit(Bit),
    Symbol(Symbol),
    Text(Text),
    Int(Int),
    Number(Number),
    Byte(Byte),
    Pair(Box<PairRepr>),
    Call(Box<CallRepr>),
    Abstract(Box<AbstractRepr>),
    Ask(Box<AskRepr>),
    Change(Box<ChangeRepr>),
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

impl From<Bit> for Repr {
    fn from(b: Bit) -> Self {
        Repr::Bit(b)
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

impl From<AbstractRepr> for Repr {
    fn from(a: AbstractRepr) -> Self {
        Repr::Abstract(Box::new(a))
    }
}

impl From<Box<AbstractRepr>> for Repr {
    fn from(a: Box<AbstractRepr>) -> Self {
        Repr::Abstract(a)
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

impl From<ChangeRepr> for Repr {
    fn from(c: ChangeRepr) -> Self {
        Repr::Change(Box::new(c))
    }
}

impl From<Box<ChangeRepr>> for Repr {
    fn from(c: Box<ChangeRepr>) -> Self {
        Repr::Change(c)
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

impl<'a> TryInto<GenRepr<'a>> for &'a Repr {
    type Error = Infallible;

    fn try_into(self) -> Result<GenRepr<'a>, Self::Error> {
        let r = match self {
            Repr::Unit(unit) => GenRepr::Unit(unit),
            Repr::Bit(bit) => GenRepr::Bit(bit),
            Repr::Symbol(symbol) => GenRepr::Symbol(symbol),
            Repr::Text(text) => GenRepr::Text(text),
            Repr::Int(int) => GenRepr::Int(int),
            Repr::Number(number) => GenRepr::Number(number),
            Repr::Byte(byte) => GenRepr::Byte(byte),
            Repr::Pair(pair) => {
                let first = (&pair.first).try_into()?;
                let second = (&pair.second).try_into()?;
                GenRepr::Pair(Box::new(Pair::new(first, second)))
            }
            Repr::Call(call) => {
                let func = (&call.func).try_into()?;
                let input = (&call.input).try_into()?;
                GenRepr::Call(Box::new(Call::new(func, input)))
            }
            Repr::Abstract(abstract1) => {
                let func = (&abstract1.func).try_into()?;
                let input = (&abstract1.input).try_into()?;
                GenRepr::Abstract(Box::new(Abstract::new(func, input)))
            }
            Repr::Ask(ask) => {
                let func = (&ask.func).try_into()?;
                let output = (&ask.output).try_into()?;
                GenRepr::Ask(Box::new(Ask::new(func, output)))
            }
            Repr::Change(change) => {
                let from = (&change.from).try_into()?;
                let to = (&change.to).try_into()?;
                GenRepr::Change(Box::new(Change::new(from, to)))
            }
            Repr::List(list) => {
                let list = list.iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
                GenRepr::List(list)
            }
            Repr::Map(map) => {
                let map = map
                    .iter()
                    .map(|(k, v)| {
                        let k = k.try_into()?;
                        let v = v.try_into()?;
                        Ok((k, v))
                    })
                    .collect::<Result<_, Infallible>>()?;
                GenRepr::Map(map)
            }
        };
        Ok(r)
    }
}

pub(crate) mod pair;

pub(crate) mod call;

pub(crate) mod abstract1;

pub(crate) mod ask;

pub(crate) mod change;

pub(crate) mod list;

pub(crate) mod map;
