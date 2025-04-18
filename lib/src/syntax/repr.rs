use std::{
    convert::Infallible,
    fmt::{
        Debug,
        Display,
    },
    str::FromStr,
};

use crate::{
    Bit,
    Byte,
    Call,
    Either,
    Equiv,
    Generate,
    Int,
    Inverse,
    Number,
    Pair,
    Symbol,
    Text,
    Unit,
    abstract1::Abstract,
    change::Change,
    reify::Reify,
    syntax::{
        ParseError,
        generate_pretty,
        generator::GenRepr,
        parse,
        parser::ParseRepr,
        repr::{
            abstract1::AbstractRepr,
            call::CallRepr,
            change::ChangeRepr,
            either::EitherRepr,
            equiv::EquivRepr,
            generate::GenerateRepr,
            inverse::InverseRepr,
            list::ListRepr,
            map::MapRepr,
            pair::PairRepr,
            reify::ReifyRepr,
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
    Either(Box<EitherRepr>),
    Change(Box<ChangeRepr>),

    Call(Box<CallRepr>),

    Reify(Box<ReifyRepr>),
    Equiv(Box<EquivRepr>),
    Inverse(Box<InverseRepr>),
    Generate(Box<GenerateRepr>),
    Abstract(Box<AbstractRepr>),

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

impl From<EitherRepr> for Repr {
    fn from(e: EitherRepr) -> Self {
        Repr::Either(Box::new(e))
    }
}

impl From<Box<EitherRepr>> for Repr {
    fn from(e: Box<EitherRepr>) -> Self {
        Repr::Either(e)
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

impl From<ReifyRepr> for Repr {
    fn from(a: ReifyRepr) -> Self {
        Repr::Reify(Box::new(a))
    }
}

impl From<Box<ReifyRepr>> for Repr {
    fn from(a: Box<ReifyRepr>) -> Self {
        Repr::Reify(a)
    }
}

impl From<EquivRepr> for Repr {
    fn from(a: EquivRepr) -> Self {
        Repr::Equiv(Box::new(a))
    }
}

impl From<Box<EquivRepr>> for Repr {
    fn from(a: Box<EquivRepr>) -> Self {
        Repr::Equiv(a)
    }
}

impl From<InverseRepr> for Repr {
    fn from(a: InverseRepr) -> Self {
        Repr::Inverse(Box::new(a))
    }
}

impl From<Box<InverseRepr>> for Repr {
    fn from(a: Box<InverseRepr>) -> Self {
        Repr::Inverse(a)
    }
}

impl From<GenerateRepr> for Repr {
    fn from(a: GenerateRepr) -> Self {
        Repr::Generate(Box::new(a))
    }
}

impl From<Box<GenerateRepr>> for Repr {
    fn from(a: Box<GenerateRepr>) -> Self {
        Repr::Generate(a)
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
            Repr::Either(either) => match &**either {
                Either::This(this) => GenRepr::Either(Box::new(Either::This(this.try_into()?))),
                Either::That(that) => GenRepr::Either(Box::new(Either::That(that.try_into()?))),
            },
            Repr::Change(change) => {
                let from = (&change.from).try_into()?;
                let to = (&change.to).try_into()?;
                GenRepr::Change(Box::new(Change::new(from, to)))
            }
            Repr::Call(call) => {
                let func = (&call.func).try_into()?;
                let input = (&call.input).try_into()?;
                GenRepr::Call(Box::new(Call::new(func, input)))
            }
            Repr::Reify(reify) => {
                let func = (&reify.func).try_into()?;
                GenRepr::Reify(Box::new(Reify::new(func)))
            }
            Repr::Equiv(equiv) => {
                let func = (&equiv.func).try_into()?;
                GenRepr::Equiv(Box::new(Equiv::new(func)))
            }
            Repr::Inverse(inverse) => {
                let func = (&inverse.func).try_into()?;
                GenRepr::Inverse(Box::new(Inverse::new(func)))
            }
            Repr::Generate(generate) => {
                let func = (&generate.func).try_into()?;
                GenRepr::Generate(Box::new(Generate::new(func)))
            }
            Repr::Abstract(abstract1) => {
                let func = (&abstract1.func).try_into()?;
                GenRepr::Abstract(Box::new(Abstract::new(func)))
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

pub(crate) mod either;

pub(crate) mod change;

pub(crate) mod call;

pub(crate) mod reify;

pub(crate) mod equiv;

pub(crate) mod inverse;

pub(crate) mod generate;

pub(crate) mod abstract1;

pub(crate) mod list;

pub(crate) mod map;
