use std::{
    fmt::{
        Debug,
        Formatter,
    },
    hash::Hash,
};

use crate::{
    bool::Bool,
    bytes::Bytes,
    comment::Comment,
    extension::ValExt,
    number::Number,
    string::Str,
    symbol::Symbol,
    syntax::{
        generator::GenerateRepr,
        parser::ParseRepr,
        repr::Repr,
    },
    unit::Unit,
    val::{
        answer::AnswerVal,
        ask::AskVal,
        assert::AssertVal,
        bytes::BytesVal,
        call::CallVal,
        comment::CommentVal,
        ctx::CtxVal,
        func::FuncVal,
        int::IntVal,
        list::ListVal,
        map::MapVal,
        number::NumberVal,
        pair::PairVal,
        string::StrVal,
    },
    Ask,
    Call,
    Int,
    List,
    Map,
    Pair,
    ReprError,
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Val {
    Unit(Unit),
    Bool(Bool),
    Symbol(Symbol),

    Int(IntVal),
    Number(NumberVal),
    String(StrVal),
    Pair(PairVal),
    List(ListVal),
    Map(MapVal),

    Bytes(BytesVal),
    Call(CallVal),
    Ask(AskVal),

    Comment(CommentVal),

    Ctx(CtxVal),
    Func(FuncVal),

    Assert(AssertVal),

    Answer(AnswerVal),

    Ext(Box<dyn ValExt>),
}

pub(crate) const UNIT: &str = "unit";
pub(crate) const BOOL: &str = "boolean";
pub(crate) const SYMBOL: &str = "symbol";
pub(crate) const INT: &str = "integer";
pub(crate) const NUMBER: &str = "number";
pub(crate) const STRING: &str = "string";
pub(crate) const PAIR: &str = "pair";
pub(crate) const LIST: &str = "list";
pub(crate) const MAP: &str = "map";
pub(crate) const BYTES: &str = "bytes";
pub(crate) const CALL: &str = "call";
pub(crate) const ASK: &str = "ask";
pub(crate) const COMMENT: &str = "comment";
pub(crate) const CTX: &str = "context";
pub(crate) const FUNC: &str = "function";
pub(crate) const ASSERT: &str = "assert";
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

impl From<Str> for Val {
    fn from(value: Str) -> Self {
        Val::String(StrVal::from(value))
    }
}

impl From<StrVal> for Val {
    fn from(value: StrVal) -> Self {
        Val::String(value)
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

impl From<Comment<Val, Val>> for Val {
    fn from(value: Comment<Val, Val>) -> Self {
        Val::Comment(CommentVal::from(value))
    }
}

impl From<CommentVal> for Val {
    fn from(value: CommentVal) -> Self {
        Val::Comment(value)
    }
}

impl From<Bytes> for Val {
    fn from(value: Bytes) -> Self {
        Val::Bytes(BytesVal::from(value))
    }
}

impl From<BytesVal> for Val {
    fn from(value: BytesVal) -> Self {
        Val::Bytes(value)
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

impl From<AssertVal> for Val {
    fn from(value: AssertVal) -> Self {
        Val::Assert(value)
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
            Repr::Unit(u) => Val::Unit(*u),
            Repr::Bool(b) => Val::Bool(*b),
            Repr::Symbol(s) => Val::Symbol(s.clone()),
            Repr::Int(i) => Val::Int(IntVal::from(i.clone())),
            Repr::Number(n) => Val::Number(NumberVal::from(n.clone())),
            Repr::String(s) => Val::String(StrVal::from(s.clone())),
            Repr::Pair(p) => Val::Pair(PairVal::from(&**p)),
            Repr::List(l) => Val::List(ListVal::from(l)),
            Repr::Map(m) => Val::Map(MapVal::from(m)),
            Repr::Bytes(b) => Val::Bytes(BytesVal::from(b.clone())),
            Repr::Call(c) => Val::Call(CallVal::from(&**c)),
            Repr::Ask(a) => Val::Ask(AskVal::from(&**a)),
            Repr::Comment(a) => Val::Comment(CommentVal::from(&**a)),
        }
    }
}

impl From<Repr> for Val {
    fn from(value: Repr) -> Self {
        match value {
            Repr::Unit(u) => Val::Unit(u),
            Repr::Bool(b) => Val::Bool(b),
            Repr::Symbol(s) => Val::Symbol(s),
            Repr::Int(i) => Val::Int(IntVal::from(i)),
            Repr::Number(n) => Val::Number(NumberVal::from(n)),
            Repr::String(s) => Val::String(StrVal::from(s)),
            Repr::Pair(p) => Val::Pair(PairVal::from(*p)),
            Repr::List(l) => Val::List(ListVal::from(l)),
            Repr::Map(m) => Val::Map(MapVal::from(m)),
            Repr::Bytes(b) => Val::Bytes(BytesVal::from(b)),
            Repr::Call(c) => Val::Call(CallVal::from(*c)),
            Repr::Ask(a) => Val::Ask(AskVal::from(*a)),
            Repr::Comment(a) => Val::Comment(CommentVal::from(*a)),
        }
    }
}

impl TryInto<Repr> for &Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(u) => Ok(Repr::Unit(*u)),
            Val::Bool(b) => Ok(Repr::Bool(*b)),
            Val::Symbol(s) => Ok(Repr::Symbol(s.clone())),
            Val::Int(i) => Ok(Repr::Int(i.into())),
            Val::Number(n) => Ok(Repr::Number(n.into())),
            Val::String(s) => Ok(Repr::String(s.into())),
            Val::Pair(p) => Ok(Repr::Pair(Box::new(p.try_into()?))),
            Val::List(l) => Ok(Repr::List(l.try_into()?)),
            Val::Map(m) => Ok(Repr::Map(m.try_into()?)),
            Val::Bytes(b) => Ok(Repr::Bytes(b.into())),
            Val::Call(c) => Ok(Repr::Call(Box::new(c.try_into()?))),
            Val::Ask(a) => Ok(Repr::Ask(Box::new(a.try_into()?))),
            Val::Comment(a) => Ok(Repr::Comment(Box::new(a.try_into()?))),
            _ => Err(ReprError {}),
        }
    }
}

impl TryInto<Repr> for Val {
    type Error = ReprError;
    fn try_into(self) -> Result<Repr, Self::Error> {
        match self {
            Val::Unit(u) => Ok(Repr::Unit(u)),
            Val::Bool(b) => Ok(Repr::Bool(b)),
            Val::Symbol(s) => Ok(Repr::Symbol(s)),
            Val::Int(i) => Ok(Repr::Int(i.into())),
            Val::Number(n) => Ok(Repr::Number(n.into())),
            Val::String(s) => Ok(Repr::String(s.into())),
            Val::Pair(p) => Ok(Repr::Pair(Box::new(p.try_into()?))),
            Val::List(l) => Ok(Repr::List(l.try_into()?)),
            Val::Map(m) => Ok(Repr::Map(m.try_into()?)),
            Val::Bytes(b) => Ok(Repr::Bytes(b.into())),
            Val::Call(c) => Ok(Repr::Call(Box::new(c.try_into()?))),
            Val::Ask(a) => Ok(Repr::Ask(Box::new(a.try_into()?))),
            Val::Comment(a) => Ok(Repr::Comment(Box::new(a.try_into()?))),
            _ => Err(ReprError {}),
        }
    }
}

impl ParseRepr for Val {
    fn try_into_pair(self) -> Result<(Self, Self), Self> {
        match self {
            Val::Pair(pair) => {
                let pair = Pair::from(pair);
                Ok((pair.first, pair.second))
            }
            other => Err(other),
        }
    }
}

impl<'a> TryInto<GenerateRepr<'a, Val>> for &'a Val {
    type Error = ReprError;

    fn try_into(self) -> Result<GenerateRepr<'a, Val>, Self::Error> {
        let r = match self {
            Val::Unit(u) => GenerateRepr::Unit(u),
            Val::Bool(b) => GenerateRepr::Bool(b),
            Val::Symbol(s) => GenerateRepr::Symbol(s),
            Val::Int(i) => GenerateRepr::Int(i),
            Val::Number(n) => GenerateRepr::Number(n),
            Val::String(s) => GenerateRepr::String(s),
            Val::Pair(p) => GenerateRepr::Pair(p),
            Val::List(l) => GenerateRepr::List(l),
            Val::Map(m) => GenerateRepr::Map(m),
            Val::Bytes(b) => GenerateRepr::Bytes(b),
            Val::Call(c) => GenerateRepr::Call(c),
            Val::Ask(a) => GenerateRepr::Ask(a),
            Val::Comment(a) => GenerateRepr::Comment(a),
            _ => return Err(ReprError {}),
        };
        Ok(r)
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Unit(u) => <_ as Debug>::fmt(u, f),
            Val::Bool(b) => <_ as Debug>::fmt(b, f),
            Val::Symbol(s) => <_ as Debug>::fmt(s, f),
            Val::Int(i) => <_ as Debug>::fmt(i, f),
            Val::Number(n) => <_ as Debug>::fmt(n, f),
            Val::String(s) => <_ as Debug>::fmt(s, f),
            Val::Pair(p) => <_ as Debug>::fmt(p, f),
            Val::List(l) => <_ as Debug>::fmt(l, f),
            Val::Map(m) => <_ as Debug>::fmt(m, f),
            Val::Bytes(b) => <_ as Debug>::fmt(b, f),
            Val::Call(c) => <_ as Debug>::fmt(c, f),
            Val::Ask(a) => <_ as Debug>::fmt(a, f),
            Val::Comment(a) => <_ as Debug>::fmt(a, f),
            Val::Ctx(c) => <_ as Debug>::fmt(c, f),
            Val::Func(func) => <_ as Debug>::fmt(func, f),
            Val::Assert(a) => <_ as Debug>::fmt(a, f),
            Val::Answer(a) => <_ as Debug>::fmt(a, f),
            Val::Ext(e) => <_ as Debug>::fmt(e, f),
        }
    }
}

pub(crate) mod int;

pub(crate) mod number;

pub(crate) mod string;

pub(crate) mod pair;

pub(crate) mod list;

pub(crate) mod map;

pub(crate) mod bytes;

pub(crate) mod call;

pub(crate) mod ask;

pub(crate) mod comment;

pub(crate) mod ctx;

pub(crate) mod func;

pub(crate) mod assert;

pub(crate) mod answer;
