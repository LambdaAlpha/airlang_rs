pub use self::byte::ByteVal;
pub use self::call::CallVal;
pub use self::ctx::CtxVal;
pub use self::func::ConstCellCompFuncVal;
pub use self::func::ConstCellPrimFuncVal;
pub use self::func::ConstStaticCompFuncVal;
pub use self::func::ConstStaticPrimFuncVal;
pub use self::func::FreeCellCompFuncVal;
pub use self::func::FreeCellPrimFuncVal;
pub use self::func::FreeStaticCompFuncVal;
pub use self::func::FreeStaticPrimFuncVal;
pub use self::func::FuncVal;
pub use self::func::MutCellCompFuncVal;
pub use self::func::MutCellPrimFuncVal;
pub use self::func::MutStaticCompFuncVal;
pub use self::func::MutStaticPrimFuncVal;
pub use self::int::IntVal;
pub use self::list::ListVal;
pub use self::map::MapVal;
pub use self::number::NumberVal;
pub use self::pair::PairVal;
pub use self::text::TextVal;

_____!();

use std::fmt::Debug;
use std::fmt::Formatter;
use std::hash::Hash;

use crate::trait_::dyn_safe::dyn_any_debug_clone_eq_hash;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Int;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Text;
use crate::type_::Unit;

pub trait Type {
    fn type_name(&self) -> Symbol;
}

dyn_any_debug_clone_eq_hash!(pub ValExt : Type);

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
    Call(CallVal),

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
pub(crate) const CALL: &str = "call";
pub(crate) const LIST: &str = "list";
pub(crate) const MAP: &str = "map";
pub(crate) const CTX: &str = "context";
pub(crate) const FUNC: &str = "function";

impl Val {
    pub fn is_unit(&self) -> bool {
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
            Val::Call(call) => <_ as Debug>::fmt(call, f),
            Val::List(list) => <_ as Debug>::fmt(list, f),
            Val::Map(map) => <_ as Debug>::fmt(map, f),
            Val::Ctx(ctx) => <_ as Debug>::fmt(ctx, f),
            Val::Func(func) => <_ as Debug>::fmt(func, f),
            Val::Ext(ext) => <_ as Debug>::fmt(ext, f),
        }
    }
}

mod text;

mod int;

mod number;

mod byte;

mod pair;

mod call;

mod list;

mod map;

mod ctx;

mod func;
