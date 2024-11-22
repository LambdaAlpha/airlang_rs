use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use crate::{
    Ask,
    Call,
    Int,
    List,
    Map,
    Pair,
    ReprError,
    adapt::Adapt,
    bool::Bool,
    byte::Byte,
    extension::ValExt,
    number::Number,
    symbol::Symbol,
    syntax::{
        generator::GenRepr,
        parser::ParseRepr,
        repr::Repr,
    },
    text::Text,
    unit::Unit,
    val::{
        adapt::AdaptVal,
        answer::AnswerVal,
        ask::AskVal,
        byte::ByteVal,
        call::CallVal,
        case::CaseVal,
        ctx::CtxVal,
        func::FuncVal,
        int::IntVal,
        list::ListVal,
        map::MapVal,
        number::NumberVal,
        pair::PairVal,
        text::TextVal,
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Unit(Unit),
    Bool(Bool),
    Symbol(Symbol),
    Text(TextVal),

    Int(IntVal),
    Number(NumberVal),
    Byte(ByteVal),

    Pair(PairVal),
    Call(CallVal),
    Adapt(AdaptVal),
    Ask(AskVal),

    List(ListVal),
    Map(MapVal),

    Ctx(CtxVal),
    Func(FuncVal),

    Case(CaseVal),

    Answer(AnswerVal),

    Ext(Box<dyn ValExt>),
}

pub(crate) const UNIT: &str = "unit";
pub(crate) const BOOL: &str = "boolean";
pub(crate) const SYMBOL: &str = "symbol";
pub(crate) const TEXT: &str = "text";
pub(crate) const INT: &str = "integer";
pub(crate) const NUMBER: &str = "number";
pub(crate) const BYTE: &str = "byte";
pub(crate) const PAIR: &str = "pair";
pub(crate) const CALL: &str = "call";
pub(crate) const ADAPT: &str = "adapt";
pub(crate) const ASK: &str = "ask";
pub(crate) const LIST: &str = "list";
pub(crate) const MAP: &str = "map";
pub(crate) const CTX: &str = "context";
pub(crate) const FUNC: &str = "function";
pub(crate) const CASE: &str = "case";
pub(crate) const ANSWER: &str = "answer";
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

impl From<Bool> for Val {
    fn from(value: Bool) -> Self {
        Val::Bool(value)
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

impl From<Adapt<Val, Val>> for Val {
    fn from(value: Adapt<Val, Val>) -> Self {
        Val::Adapt(AdaptVal::from(value))
    }
}

impl From<AdaptVal> for Val {
    fn from(value: AdaptVal) -> Self {
        Val::Adapt(value)
    }
}

impl From<Ask<Val, Val>> for Val {
    fn from(value: Ask<Val, Val>) -> Self {
        Val::Ask(AskVal::from(value))
    }
}

impl From<AskVal> for Val {
    fn from(value: AskVal) -> Self {
        Val::Ask(value)
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

impl From<CaseVal> for Val {
    fn from(value: CaseVal) -> Self {
        Val::Case(value)
    }
}

impl From<AnswerVal> for Val {
    fn from(value: AnswerVal) -> Self {
        Val::Answer(value)
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
            Repr::Bool(bool) => Val::Bool(*bool),
            Repr::Symbol(symbol) => Val::Symbol(symbol.clone()),
            Repr::Text(text) => Val::Text(TextVal::from(text.clone())),
            Repr::Int(int) => Val::Int(IntVal::from(int.clone())),
            Repr::Number(number) => Val::Number(NumberVal::from(number.clone())),
            Repr::Byte(byte) => Val::Byte(ByteVal::from(byte.clone())),
            Repr::Pair(pair) => Val::Pair(PairVal::from(&**pair)),
            Repr::Call(call) => Val::Call(CallVal::from(&**call)),
            Repr::Adapt(adapt) => Val::Adapt(AdaptVal::from(&**adapt)),
            Repr::Ask(ask) => Val::Ask(AskVal::from(&**ask)),
            Repr::List(list) => Val::List(ListVal::from(list)),
            Repr::Map(map) => Val::Map(MapVal::from(map)),
        }
    }
}

impl From<Repr> for Val {
    fn from(value: Repr) -> Self {
        match value {
            Repr::Unit(unit) => Val::Unit(unit),
            Repr::Bool(bool) => Val::Bool(bool),
            Repr::Symbol(symbol) => Val::Symbol(symbol),
            Repr::Text(text) => Val::Text(TextVal::from(text)),
            Repr::Int(int) => Val::Int(IntVal::from(int)),
            Repr::Number(number) => Val::Number(NumberVal::from(number)),
            Repr::Byte(byte) => Val::Byte(ByteVal::from(byte)),
            Repr::Pair(pair) => Val::Pair(PairVal::from(*pair)),
            Repr::Call(call) => Val::Call(CallVal::from(*call)),
            Repr::Adapt(adapt) => Val::Adapt(AdaptVal::from(*adapt)),
            Repr::Ask(ask) => Val::Ask(AskVal::from(*ask)),
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
            Val::Bool(bool) => Ok(Repr::Bool(*bool)),
            Val::Symbol(symbol) => Ok(Repr::Symbol(symbol.clone())),
            Val::Text(text) => Ok(Repr::Text(text.into())),
            Val::Int(int) => Ok(Repr::Int(int.into())),
            Val::Number(number) => Ok(Repr::Number(number.into())),
            Val::Byte(byte) => Ok(Repr::Byte(byte.into())),
            Val::Pair(pair) => Ok(Repr::Pair(Box::new(pair.try_into()?))),
            Val::Call(call) => Ok(Repr::Call(Box::new(call.try_into()?))),
            Val::Adapt(adapt) => Ok(Repr::Adapt(Box::new(adapt.try_into()?))),
            Val::Ask(ask) => Ok(Repr::Ask(Box::new(ask.try_into()?))),
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
            Val::Bool(bool) => Ok(Repr::Bool(bool)),
            Val::Symbol(symbol) => Ok(Repr::Symbol(symbol)),
            Val::Text(text) => Ok(Repr::Text(text.into())),
            Val::Int(int) => Ok(Repr::Int(int.into())),
            Val::Number(number) => Ok(Repr::Number(number.into())),
            Val::Byte(byte) => Ok(Repr::Byte(byte.into())),
            Val::Pair(pair) => Ok(Repr::Pair(Box::new(pair.try_into()?))),
            Val::Call(call) => Ok(Repr::Call(Box::new(call.try_into()?))),
            Val::Adapt(adapt) => Ok(Repr::Adapt(Box::new(adapt.try_into()?))),
            Val::Ask(ask) => Ok(Repr::Ask(Box::new(ask.try_into()?))),
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
            Val::Bool(bool) => GenRepr::Bool(bool),
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
            Val::Call(call) => {
                let func = (&call.func).try_into()?;
                let input = (&call.input).try_into()?;
                GenRepr::Call(Box::new(Call::new(func, input)))
            }
            Val::Adapt(adapt) => {
                let spec = (&adapt.spec).try_into()?;
                let value = (&adapt.value).try_into()?;
                GenRepr::Adapt(Box::new(Adapt::new(spec, value)))
            }
            Val::Ask(ask) => {
                let func = (&ask.func).try_into()?;
                let output = (&ask.output).try_into()?;
                GenRepr::Ask(Box::new(Ask::new(func, output)))
            }
            Val::List(list) => {
                let list: List<GenRepr> = list
                    .iter()
                    .map(TryInto::try_into)
                    .collect::<Result<_, _>>()?;
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
            Val::Bool(bool) => <_ as Debug>::fmt(bool, f),
            Val::Symbol(symbol) => <_ as Debug>::fmt(symbol, f),
            Val::Text(text) => <_ as Debug>::fmt(text, f),
            Val::Int(int) => <_ as Debug>::fmt(int, f),
            Val::Number(number) => <_ as Debug>::fmt(number, f),
            Val::Byte(byte) => <_ as Debug>::fmt(byte, f),
            Val::Pair(pair) => <_ as Debug>::fmt(pair, f),
            Val::Call(call) => <_ as Debug>::fmt(call, f),
            Val::Adapt(adapt) => <_ as Debug>::fmt(adapt, f),
            Val::Ask(ask) => <_ as Debug>::fmt(ask, f),
            Val::List(list) => <_ as Debug>::fmt(list, f),
            Val::Map(map) => <_ as Debug>::fmt(map, f),
            Val::Ctx(ctx) => <_ as Debug>::fmt(ctx, f),
            Val::Func(func) => <_ as Debug>::fmt(func, f),
            Val::Case(case) => <_ as Debug>::fmt(case, f),
            Val::Answer(answer) => <_ as Debug>::fmt(answer, f),
            Val::Ext(ext) => <_ as Debug>::fmt(ext, f),
        }
    }
}

pub(crate) mod text;

pub(crate) mod int;

pub(crate) mod number;

pub(crate) mod byte;

pub(crate) mod pair;

pub(crate) mod call;

pub(crate) mod adapt;

pub(crate) mod ask;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod case;

pub(crate) mod ctx;

pub(crate) mod func;

pub(crate) mod answer;
