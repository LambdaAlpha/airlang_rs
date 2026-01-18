use std::convert::Infallible;
use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

use derive_more::From;
use derive_more::IsVariant;

use super::ParseError;
use super::generate_pretty;
use super::generator::GenRepr;
use super::parse;
use super::parser::ParseRepr;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::Decimal;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(PartialEq, Eq, Clone, From, IsVariant)]
pub enum Repr {
    Unit(Unit),
    Bit(Bit),

    Key(Key),

    Text(Text),
    Int(Int),
    Decimal(Decimal),
    Byte(Byte),

    Cell(Box<CellRepr>),
    Pair(Box<PairRepr>),
    Call(Box<CallRepr>),

    List(ListRepr),
    Map(MapRepr),
}

pub type CellRepr = Cell<Repr>;

pub type PairRepr = Pair<Repr, Repr>;

pub type CallRepr = Call<Repr, Repr>;

pub type ListRepr = List<Repr>;

pub type MapRepr = Map<Key, Repr>;

impl From<CellRepr> for Repr {
    fn from(cell: CellRepr) -> Self {
        Repr::Cell(Box::new(cell))
    }
}

impl From<PairRepr> for Repr {
    fn from(pair: PairRepr) -> Self {
        Repr::Pair(Box::new(pair))
    }
}

impl From<CallRepr> for Repr {
    fn from(call: CallRepr) -> Self {
        Repr::Call(Box::new(call))
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
            Repr::Key(key) => GenRepr::Key(key),
            Repr::Text(text) => GenRepr::Text(text),
            Repr::Int(int) => GenRepr::Int(int),
            Repr::Decimal(decimal) => GenRepr::Decimal(decimal),
            Repr::Byte(byte) => GenRepr::Byte(byte),
            Repr::Cell(cell) => {
                let value = (&cell.value).try_into()?;
                GenRepr::Cell(Box::new(Cell::new(value)))
            }
            Repr::Pair(pair) => {
                let left = (&pair.left).try_into()?;
                let right = (&pair.right).try_into()?;
                GenRepr::Pair(Box::new(Pair::new(left, right)))
            }
            Repr::Call(call) => {
                let func = (&call.func).try_into()?;
                let input = (&call.input).try_into()?;
                GenRepr::Call(Box::new(Call { func, input }))
            }
            Repr::List(list) => {
                let list = list.iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
                GenRepr::List(list)
            }
            Repr::Map(map) => {
                let map = map
                    .iter()
                    .map(|(k, v)| {
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
