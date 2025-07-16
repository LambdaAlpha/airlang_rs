use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

use super::ParseError;
use super::generate_pretty;
use super::generator::GenRepr;
use super::parse;
use super::parser::ParseRepr;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Task;
use crate::type_::Text;
use crate::type_::Unit;

// todo impl derive from
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
    Task(Box<TaskRepr>),

    List(ListRepr),
    Map(MapRepr),
}

pub type PairRepr = Pair<Repr, Repr>;

pub type TaskRepr = Task<Repr, Repr, Repr>;

pub type ListRepr = List<Repr>;

pub type MapRepr = Map<Repr, Repr>;

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

impl From<TaskRepr> for Repr {
    fn from(t: TaskRepr) -> Self {
        Repr::Task(Box::new(t))
    }
}

impl From<Box<TaskRepr>> for Repr {
    fn from(t: Box<TaskRepr>) -> Self {
        Repr::Task(t)
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
        write!(f, "{}", generate_pretty(self.try_into().unwrap()))
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", generate_pretty(self.try_into().unwrap()))
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
        generate_pretty(value.try_into().unwrap())
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
            Repr::Task(task) => {
                let func = (&task.func).try_into()?;
                let ctx = (&task.ctx).try_into()?;
                let input = (&task.input).try_into()?;
                GenRepr::Task(Box::new(Task { action: task.action, func, ctx, input }))
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
