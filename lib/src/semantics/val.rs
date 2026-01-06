pub use self::byte::ByteVal;
pub use self::call::CallVal;
pub use self::cell::CellVal;
pub use self::cfg::CfgVal;
pub use self::decimal::DecimalVal;
pub use self::func::ConstCompFuncVal;
pub use self::func::ConstPrimFuncVal;
pub use self::func::FreeCompFuncVal;
pub use self::func::FreePrimFuncVal;
pub use self::func::FuncVal;
pub use self::func::MutCompFuncVal;
pub use self::func::MutPrimFuncVal;
pub use self::int::IntVal;
pub use self::link::LinkVal;
pub use self::list::ListVal;
pub use self::map::MapVal;
pub use self::pair::PairVal;
pub use self::text::TextVal;

_____!();

use std::fmt::Debug;
use std::fmt::Formatter;

use derive_more::From;
use derive_more::IsVariant;

use crate::semantics::ctx::DynCtx;
use crate::trait_::dyn_safe::dyn_any_debug_clone_eq;
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

pub trait Value: DynCtx<Val, Val> {
    fn type_name(&self) -> Key;
}

dyn_any_debug_clone_eq!(pub DynVal : Value);

#[derive(Clone, PartialEq, Eq, From, IsVariant)]
pub enum Val {
    Unit(Unit),
    Bit(Bit),

    Key(Key),

    Text(TextVal),
    Int(IntVal),
    Decimal(DecimalVal),
    Byte(ByteVal),

    Cell(CellVal),
    Pair(PairVal),
    Call(CallVal),

    List(ListVal),
    Map(MapVal),

    Link(LinkVal),
    Cfg(CfgVal),
    Func(FuncVal),

    Dyn(Box<dyn DynVal>),
}

pub(crate) const UNIT: &str = "unit";
pub(crate) const BIT: &str = "bit";
pub(crate) const KEY: &str = "key";
pub(crate) const TEXT: &str = "text";
pub(crate) const INT: &str = "integer";
pub(crate) const DECIMAL: &str = "decimal";
pub(crate) const BYTE: &str = "byte";
pub(crate) const CELL: &str = "cell";
pub(crate) const PAIR: &str = "pair";
pub(crate) const CALL: &str = "call";
pub(crate) const LIST: &str = "list";
pub(crate) const MAP: &str = "map";
pub(crate) const LINK: &str = "link";
pub(crate) const CFG: &str = "config";
pub(crate) const FUNC: &str = "function";

impl Default for Val {
    fn default() -> Self {
        Val::Unit(Unit)
    }
}

impl From<Text> for Val {
    fn from(value: Text) -> Self {
        Val::Text(TextVal::from(value))
    }
}

impl From<Int> for Val {
    fn from(value: Int) -> Self {
        Val::Int(IntVal::from(value))
    }
}

impl From<Decimal> for Val {
    fn from(value: Decimal) -> Self {
        Val::Decimal(DecimalVal::from(value))
    }
}

impl From<Byte> for Val {
    fn from(value: Byte) -> Self {
        Val::Byte(ByteVal::from(value))
    }
}

impl From<Cell<Val>> for Val {
    fn from(value: Cell<Val>) -> Self {
        Val::Cell(CellVal::from(value))
    }
}

impl From<Pair<Val, Val>> for Val {
    fn from(value: Pair<Val, Val>) -> Self {
        Val::Pair(PairVal::from(value))
    }
}

impl From<Call<Val, Val>> for Val {
    fn from(value: Call<Val, Val>) -> Self {
        Val::Call(CallVal::from(value))
    }
}

impl From<List<Val>> for Val {
    fn from(value: List<Val>) -> Self {
        Val::List(ListVal::from(value))
    }
}

impl From<Map<Key, Val>> for Val {
    fn from(value: Map<Key, Val>) -> Self {
        Val::Map(MapVal::from(value))
    }
}

macro_rules! match_val {
    ($self:ident, $name:ident => $body:expr) => {
        match $self {
            $crate::semantics::val::Val::Unit($name) => $body,
            $crate::semantics::val::Val::Bit($name) => $body,
            $crate::semantics::val::Val::Key($name) => $body,
            $crate::semantics::val::Val::Text($name) => $body,
            $crate::semantics::val::Val::Int($name) => $body,
            $crate::semantics::val::Val::Decimal($name) => $body,
            $crate::semantics::val::Val::Byte($name) => $body,
            $crate::semantics::val::Val::Cell($name) => $body,
            $crate::semantics::val::Val::Pair($name) => $body,
            $crate::semantics::val::Val::Call($name) => $body,
            $crate::semantics::val::Val::List($name) => $body,
            $crate::semantics::val::Val::Map($name) => $body,
            $crate::semantics::val::Val::Link($name) => $body,
            $crate::semantics::val::Val::Cfg($name) => $body,
            $crate::semantics::val::Val::Func($name) => $body,
            $crate::semantics::val::Val::Dyn($name) => $body,
        }
    };
}

// clippy stable: redundant_imports
// clippy nightly: unused_imports
#[allow(unused_imports, redundant_imports)]
pub(crate) use match_val;

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match_val!(self, v => <_ as Debug>::fmt(v, f))
    }
}

mod text;

mod int;

mod decimal;

mod byte;

mod cell;

mod pair;

mod call;

mod list;

mod map;

mod link;

mod cfg;

mod func;
