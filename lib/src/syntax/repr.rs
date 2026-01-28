use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

use derive_more::From;
use derive_more::IsVariant;

use super::ParseError;
use super::generator::FmtCtx;
use super::generator::FmtRepr;
use super::parser::ParseRepr;
use super::parser::parse;
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

impl Default for Repr {
    fn default() -> Self {
        Repr::Unit(Unit)
    }
}

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

impl ParseRepr for Repr {}

impl FromStr for Repr {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl Display for Repr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl Debug for Repr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl FmtRepr for Repr {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Repr::Unit(unit) => <Unit as Display>::fmt(unit, f),
            Repr::Bit(bit) => <Bit as Display>::fmt(bit, f),
            Repr::Key(key) => <Key as Display>::fmt(key, f),
            Repr::Text(text) => <Text as Display>::fmt(text, f),
            Repr::Int(int) => <Int as Display>::fmt(int, f),
            Repr::Decimal(decimal) => <Decimal as Display>::fmt(decimal, f),
            Repr::Byte(byte) => <Byte as Display>::fmt(byte, f),
            Repr::Cell(cell) => <CellRepr as FmtRepr>::fmt(cell, ctx, f),
            Repr::Pair(pair) => <PairRepr as FmtRepr>::fmt(pair, ctx, f),
            Repr::Call(call) => <CallRepr as FmtRepr>::fmt(call, ctx, f),
            Repr::List(list) => <ListRepr as FmtRepr>::fmt(list, ctx, f),
            Repr::Map(map) => <MapRepr as FmtRepr>::fmt(map, ctx, f),
        }
    }

    fn is_call(&self) -> bool {
        matches!(self, Repr::Call(_))
    }

    fn is_pair(&self) -> bool {
        matches!(self, Repr::Pair(_))
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        let Repr::Pair(pair) = self else {
            panic!("called `FmtRepr::to_pair()` on non-pair value")
        };
        Pair::new(&pair.left, &pair.right)
    }
}
