use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use crate::{
    Call,
    Int,
    List,
    Map,
    Pair,
    Solve,
    abstract1::Abstract,
    bit::Bit,
    byte::Byte,
    change::Change,
    extension::ValExt,
    number::Number,
    optimize::Optimize,
    symbol::Symbol,
    syntax::{
        ReprError,
        generator::GenRepr,
        parser::ParseRepr,
        repr::Repr,
    },
    text::Text,
    unit::Unit,
    val::{
        abstract1::AbstractVal,
        byte::ByteVal,
        call::CallVal,
        change::ChangeVal,
        ctx::CtxVal,
        func::FuncVal,
        int::IntVal,
        list::ListVal,
        map::MapVal,
        number::NumberVal,
        optimize::OptimizeVal,
        pair::PairVal,
        solve::SolveVal,
        text::TextVal,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Unit(Unit),
    Bit(Bit),

    Symbol(Symbol),

    Text(TextVal),
    Int(IntVal),
    Number(NumberVal),
    Byte(ByteVal),

    Pair(PairVal),
    Change(ChangeVal),

    Call(CallVal),
    Optimize(OptimizeVal),
    Solve(SolveVal),
    Abstract(AbstractVal),

    List(ListVal),
    Map(MapVal),

    Ctx(CtxVal),
    Func(FuncVal),

    Ext(Box<dyn ValExt>),
}

pub(crate) const UNIT: &str = "unit";
pub(crate) const BIT: &str = "bit";
pub(crate) const SYMBOL: &str = "symbol";
pub(crate) const TEXT: &str = "text";
pub(crate) const INT: &str = "integer";
pub(crate) const NUMBER: &str = "number";
pub(crate) const BYTE: &str = "byte";
pub(crate) const PAIR: &str = "pair";
pub(crate) const CHANGE: &str = "change";
pub(crate) const CALL: &str = "call";
pub(crate) const OPTIMIZE: &str = "optimize";
pub(crate) const SOLVE: &str = "solve";
pub(crate) const ABSTRACT: &str = "abstract";
pub(crate) const LIST: &str = "list";
pub(crate) const MAP: &str = "map";
pub(crate) const CTX: &str = "context";
pub(crate) const FUNC: &str = "function";
pub(crate) const EXT: &str = "extension";

impl Val {
    pub(crate) fn is_unit(&self) -> bool {
        matches!(self, Val::Unit(_))
    }
}

impl Default for Val {
    fn default() -> Self {
        Val::Unit(Unit)
    }
}

impl From<Unit> for Val {
    fn from(value: Unit) -> Self {
        Val::Unit(value)
    }
}

impl From<Bit> for Val {
    fn from(value: Bit) -> Self {
        Val::Bit(value)
    }
}

impl From<Symbol> for Val {
    fn from(value: Symbol) -> Self {
        Val::Symbol(value)
    }
}

impl From<Text> for Val {
    fn from(value: Text) -> Self {
        Val::Text(TextVal::from(value))
    }
}

impl From<TextVal> for Val {
    fn from(value: TextVal) -> Self {
        Val::Text(value)
    }
}

impl From<Int> for Val {
    fn from(value: Int) -> Self {
        Val::Int(IntVal::from(value))
    }
}

impl From<IntVal> for Val {
    fn from(value: IntVal) -> Self {
        Val::Int(value)
    }
}

impl From<Number> for Val {
    fn from(value: Number) -> Self {
        Val::Number(NumberVal::from(value))
    }
}

impl From<NumberVal> for Val {
    fn from(value: NumberVal) -> Self {
        Val::Number(value)
    }
}

impl From<Byte> for Val {
    fn from(value: Byte) -> Self {
        Val::Byte(ByteVal::from(value))
    }
}

impl From<ByteVal> for Val {
    fn from(value: ByteVal) -> Self {
        Val::Byte(value)
    }
}

impl From<Pair<Val, Val>> for Val {
    fn from(value: Pair<Val, Val>) -> Self {
        Val::Pair(PairVal::from(value))
    }
}

impl From<PairVal> for Val {
    fn from(value: PairVal) -> Self {
        Val::Pair(value)
    }
}

impl From<Change<Val, Val>> for Val {
    fn from(value: Change<Val, Val>) -> Self {
        Val::Change(ChangeVal::from(value))
    }
}

impl From<ChangeVal> for Val {
    fn from(value: ChangeVal) -> Self {
        Val::Change(value)
    }
}

impl From<Call<Val, Val>> for Val {
    fn from(value: Call<Val, Val>) -> Self {
        Val::Call(CallVal::from(value))
    }
}

impl From<CallVal> for Val {
    fn from(value: CallVal) -> Self {
        Val::Call(value)
    }
}

impl From<Optimize<Val, Val>> for Val {
    fn from(value: Optimize<Val, Val>) -> Self {
        Val::Optimize(OptimizeVal::from(value))
    }
}

impl From<OptimizeVal> for Val {
    fn from(value: OptimizeVal) -> Self {
        Val::Optimize(value)
    }
}

impl From<Solve<Val, Val>> for Val {
    fn from(value: Solve<Val, Val>) -> Self {
        Val::Solve(SolveVal::from(value))
    }
}

impl From<SolveVal> for Val {
    fn from(value: SolveVal) -> Self {
        Val::Solve(value)
    }
}

impl From<Abstract<Val>> for Val {
    fn from(value: Abstract<Val>) -> Self {
        Val::Abstract(AbstractVal::from(value))
    }
}

impl From<AbstractVal> for Val {
    fn from(value: AbstractVal) -> Self {
        Val::Abstract(value)
    }
}

impl From<List<Val>> for Val {
    fn from(value: List<Val>) -> Self {
        Val::List(ListVal::from(value))
    }
}

impl From<ListVal> for Val {
    fn from(value: ListVal) -> Self {
        Val::List(value)
    }
}

impl From<Map<Val, Val>> for Val {
    fn from(value: Map<Val, Val>) -> Self {
        Val::Map(MapVal::from(value))
    }
}

impl From<MapVal> for Val {
    fn from(value: MapVal) -> Self {
        Val::Map(value)
    }
}

impl From<CtxVal> for Val {
    fn from(value: CtxVal) -> Self {
        Val::Ctx(value)
    }
}

impl From<FuncVal> for Val {
    fn from(value: FuncVal) -> Self {
        Val::Func(value)
    }
}

impl From<Box<dyn ValExt>> for Val {
    fn from(value: Box<dyn ValExt>) -> Self {
        Val::Ext(value)
    }
}

impl From<&Repr> for Val {
    fn from(value: &Repr) -> Self {
        match value {
            Repr::Unit(unit) => Val::Unit(*unit),
            Repr::Bit(bit) => Val::Bit(*bit),
            Repr::Symbol(symbol) => Val::Symbol(symbol.clone()),
            Repr::Text(text) => Val::Text(TextVal::from(text.clone())),
            Repr::Int(int) => Val::Int(IntVal::from(int.clone())),
            Repr::Number(number) => Val::Number(NumberVal::from(number.clone())),
            Repr::Byte(byte) => Val::Byte(ByteVal::from(byte.clone())),
            Repr::Pair(pair) => Val::Pair(PairVal::from(&**pair)),
            Repr::Change(change) => Val::Change(ChangeVal::from(&**change)),
            Repr::Call(call) => Val::Call(CallVal::from(&**call)),
            Repr::Optimize(optimize) => Val::Optimize(OptimizeVal::from(&**optimize)),
            Repr::Solve(solve) => Val::Solve(SolveVal::from(&**solve)),
            Repr::Abstract(abstract1) => Val::Abstract(AbstractVal::from(&**abstract1)),
            Repr::List(list) => Val::List(ListVal::from(list)),
            Repr::Map(map) => Val::Map(MapVal::from(map)),
        }
    }
}

impl From<Repr> for Val {
    fn from(value: Repr) -> Self {
        match value {
            Repr::Unit(unit) => Val::Unit(unit),
            Repr::Bit(bit) => Val::Bit(bit),
            Repr::Symbol(symbol) => Val::Symbol(symbol),
            Repr::Text(text) => Val::Text(TextVal::from(text)),
            Repr::Int(int) => Val::Int(IntVal::from(int)),
            Repr::Number(number) => Val::Number(NumberVal::from(number)),
            Repr::Byte(byte) => Val::Byte(ByteVal::from(byte)),
            Repr::Pair(pair) => Val::Pair(PairVal::from(*pair)),
            Repr::Change(change) => Val::Change(ChangeVal::from(*change)),
            Repr::Call(call) => Val::Call(CallVal::from(*call)),
            Repr::Optimize(optimize) => Val::Optimize(OptimizeVal::from(*optimize)),
            Repr::Solve(solve) => Val::Solve(SolveVal::from(*solve)),
            Repr::Abstract(abstract1) => Val::Abstract(AbstractVal::from(*abstract1)),
            Repr::List(list) => Val::List(ListVal::from(list)),
            Repr::Map(map) => Val::Map(MapVal::from(map)),
        }
    }
}

impl TryInto<Repr> for &Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(unit) => Ok(Repr::Unit(*unit)),
            Val::Bit(bit) => Ok(Repr::Bit(*bit)),
            Val::Symbol(symbol) => Ok(Repr::Symbol(symbol.clone())),
            Val::Text(text) => Ok(Repr::Text(Text::clone(text))),
            Val::Int(int) => Ok(Repr::Int(Int::clone(int))),
            Val::Number(number) => Ok(Repr::Number(Number::clone(number))),
            Val::Byte(byte) => Ok(Repr::Byte(Byte::clone(byte))),
            Val::Pair(pair) => Ok(Repr::Pair(Box::new(pair.try_into()?))),
            Val::Change(change) => Ok(Repr::Change(Box::new(change.try_into()?))),
            Val::Call(call) => Ok(Repr::Call(Box::new(call.try_into()?))),
            Val::Optimize(optimize) => Ok(Repr::Optimize(Box::new(optimize.try_into()?))),
            Val::Solve(solve) => Ok(Repr::Solve(Box::new(solve.try_into()?))),
            Val::Abstract(abstract1) => Ok(Repr::Abstract(Box::new(abstract1.try_into()?))),
            Val::List(list) => Ok(Repr::List(list.try_into()?)),
            Val::Map(map) => Ok(Repr::Map(map.try_into()?)),
            _ => Err(ReprError {}),
        }
    }
}

impl TryInto<Repr> for Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(unit) => Ok(Repr::Unit(unit)),
            Val::Bit(bit) => Ok(Repr::Bit(bit)),
            Val::Symbol(symbol) => Ok(Repr::Symbol(symbol)),
            Val::Text(text) => Ok(Repr::Text(text.into())),
            Val::Int(int) => Ok(Repr::Int(int.into())),
            Val::Number(number) => Ok(Repr::Number(number.into())),
            Val::Byte(byte) => Ok(Repr::Byte(byte.into())),
            Val::Pair(pair) => Ok(Repr::Pair(Box::new(pair.try_into()?))),
            Val::Change(change) => Ok(Repr::Change(Box::new(change.try_into()?))),
            Val::Call(call) => Ok(Repr::Call(Box::new(call.try_into()?))),
            Val::Optimize(optimize) => Ok(Repr::Optimize(Box::new(optimize.try_into()?))),
            Val::Solve(solve) => Ok(Repr::Solve(Box::new(solve.try_into()?))),
            Val::Abstract(abstract1) => Ok(Repr::Abstract(Box::new(abstract1.try_into()?))),
            Val::List(list) => Ok(Repr::List(list.try_into()?)),
            Val::Map(map) => Ok(Repr::Map(map.try_into()?)),
            _ => Err(ReprError {}),
        }
    }
}

impl ParseRepr for Val {}

impl<'a> TryInto<GenRepr<'a>> for &'a Val {
    type Error = ReprError;

    fn try_into(self) -> Result<GenRepr<'a>, Self::Error> {
        let r = match self {
            Val::Unit(unit) => GenRepr::Unit(unit),
            Val::Bit(bit) => GenRepr::Bit(bit),
            Val::Symbol(symbol) => GenRepr::Symbol(symbol),
            Val::Text(text) => GenRepr::Text(text),
            Val::Int(int) => GenRepr::Int(int),
            Val::Number(number) => GenRepr::Number(number),
            Val::Byte(byte) => GenRepr::Byte(byte),
            Val::Pair(pair) => {
                let first = (&pair.first).try_into()?;
                let second = (&pair.second).try_into()?;
                GenRepr::Pair(Box::new(Pair::new(first, second)))
            }
            Val::Change(change) => {
                let from = (&change.from).try_into()?;
                let to = (&change.to).try_into()?;
                GenRepr::Change(Box::new(Change::new(from, to)))
            }
            Val::Call(call) => {
                let func = (&call.func).try_into()?;
                let input = (&call.input).try_into()?;
                GenRepr::Call(Box::new(Call::new(func, input)))
            }
            Val::Optimize(optimize) => {
                let func = (&optimize.func).try_into()?;
                let input = (&optimize.input).try_into()?;
                GenRepr::Optimize(Box::new(Optimize::new(func, input)))
            }
            Val::Solve(solve) => {
                let func = (&solve.func).try_into()?;
                let output = (&solve.output).try_into()?;
                GenRepr::Solve(Box::new(Solve::new(func, output)))
            }
            Val::Abstract(abstract1) => {
                let value = (&abstract1.value).try_into()?;
                GenRepr::Abstract(Box::new(Abstract::new(value)))
            }
            Val::List(list) => {
                let list: List<GenRepr> =
                    list.iter().map(TryInto::try_into).collect::<Result<_, _>>()?;
                GenRepr::List(list)
            }
            Val::Map(map) => {
                let map = map
                    .iter()
                    .map(|(k, v)| {
                        let k = k.try_into()?;
                        let v = v.try_into()?;
                        Ok((k, v))
                    })
                    .collect::<Result<_, _>>()?;
                GenRepr::Map(map)
            }
            _ => return Err(ReprError {}),
        };
        Ok(r)
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Unit(unit) => <_ as Debug>::fmt(unit, f),
            Val::Bit(bool) => <_ as Debug>::fmt(bool, f),
            Val::Symbol(symbol) => <_ as Debug>::fmt(symbol, f),
            Val::Text(text) => <_ as Debug>::fmt(text, f),
            Val::Int(int) => <_ as Debug>::fmt(int, f),
            Val::Number(number) => <_ as Debug>::fmt(number, f),
            Val::Byte(byte) => <_ as Debug>::fmt(byte, f),
            Val::Pair(pair) => <_ as Debug>::fmt(pair, f),
            Val::Change(change) => <_ as Debug>::fmt(change, f),
            Val::Call(call) => <_ as Debug>::fmt(call, f),
            Val::Optimize(optimize) => <_ as Debug>::fmt(optimize, f),
            Val::Solve(solve) => <_ as Debug>::fmt(solve, f),
            Val::Abstract(abstrac1) => <_ as Debug>::fmt(abstrac1, f),
            Val::List(list) => <_ as Debug>::fmt(list, f),
            Val::Map(map) => <_ as Debug>::fmt(map, f),
            Val::Ctx(ctx) => <_ as Debug>::fmt(ctx, f),
            Val::Func(func) => <_ as Debug>::fmt(func, f),
            Val::Ext(ext) => <_ as Debug>::fmt(ext, f),
        }
    }
}

pub(crate) mod text;

pub(crate) mod int;

pub(crate) mod number;

pub(crate) mod byte;

pub(crate) mod pair;

pub(crate) mod change;

pub(crate) mod call;

pub(crate) mod optimize;

pub(crate) mod solve;

pub(crate) mod abstract1;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod ctx;

pub(crate) mod func;
