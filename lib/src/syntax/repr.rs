use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::str::FromStr;

use derive_more::From;
use derive_more::IsVariant;

use super::ParseError;
use super::ReprType;
use super::generator::FmtOptions;
use super::generator::FmtRepr;
use super::impl_display_debug_for_fmt_repr;
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
use crate::type_::Quote;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(Clone, PartialEq, Eq, Hash, From, IsVariant)]
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
    List(ListRepr),
    Map(MapRepr),

    Quote(Box<QuoteRepr>),
    Call(Box<CallRepr>),
}

pub type CellRepr = Cell<Repr>;

pub type PairRepr = Pair<Repr, Repr>;

pub type ListRepr = List<Repr>;

pub type MapRepr = Map<Key, Repr>;

pub type QuoteRepr = Quote<Repr>;

pub type CallRepr = Call<Repr, Repr>;

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

impl From<QuoteRepr> for Repr {
    fn from(quote: QuoteRepr) -> Self {
        Repr::Quote(Box::new(quote))
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

impl FmtRepr for Repr {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        match self {
            Repr::Unit(unit) => <Unit as FmtRepr>::fmt(unit, options, f),
            Repr::Bit(bit) => <Bit as FmtRepr>::fmt(bit, options, f),
            Repr::Key(key) => <Key as FmtRepr>::fmt(key, options, f),
            Repr::Text(text) => <Text as FmtRepr>::fmt(text, options, f),
            Repr::Int(int) => <Int as FmtRepr>::fmt(int, options, f),
            Repr::Decimal(decimal) => <Decimal as FmtRepr>::fmt(decimal, options, f),
            Repr::Byte(byte) => <Byte as FmtRepr>::fmt(byte, options, f),
            Repr::Cell(cell) => <CellRepr as FmtRepr>::fmt(cell, options, f),
            Repr::Pair(pair) => <PairRepr as FmtRepr>::fmt(pair, options, f),
            Repr::List(list) => <ListRepr as FmtRepr>::fmt(list, options, f),
            Repr::Map(map) => <MapRepr as FmtRepr>::fmt(map, options, f),
            Repr::Quote(quote) => <QuoteRepr as FmtRepr>::fmt(quote, options, f),
            Repr::Call(call) => <CallRepr as FmtRepr>::fmt(call, options, f),
        }
    }

    fn get_type(&self) -> ReprType {
        match self {
            Repr::Unit(_) => ReprType::Unit,
            Repr::Bit(_) => ReprType::Bit,
            Repr::Key(_) => ReprType::Key,
            Repr::Text(_) => ReprType::Text,
            Repr::Int(_) => ReprType::Int,
            Repr::Decimal(_) => ReprType::Decimal,
            Repr::Byte(_) => ReprType::Byte,
            Repr::Cell(_) => ReprType::Cell,
            Repr::Pair(_) => ReprType::Pair,
            Repr::List(_) => ReprType::List,
            Repr::Map(_) => ReprType::Map,
            Repr::Quote(_) => ReprType::Quote,
            Repr::Call(_) => ReprType::Call,
        }
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        let Repr::Pair(pair) = self else {
            panic!("called `FmtRepr::to_pair()` on non-pair value")
        };
        Pair::new(&pair.left, &pair.right)
    }
}

impl_display_debug_for_fmt_repr!(Repr);
