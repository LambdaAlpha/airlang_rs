pub use self::byte::ByteVal;
pub use self::call::CallVal;
pub use self::cfg::CfgVal;
pub use self::ctx::CtxVal;
pub use self::func::ConstCompFuncVal;
pub use self::func::ConstPrimFuncVal;
pub use self::func::FreeCompFuncVal;
pub use self::func::FreePrimFuncVal;
pub use self::func::FuncVal;
pub use self::func::MutCompFuncVal;
pub use self::func::MutPrimFuncVal;
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

use derive_more::From;
use derive_more::IsVariant;

use crate::semantics::ctx::DynCtx;
use crate::trait_::dyn_safe::dyn_any_debug_clone_eq_hash;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Int;
use crate::type_::Link;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Text;
use crate::type_::Unit;

pub trait Value: DynCtx<Val, Val> {
    fn type_name(&self) -> Symbol;
}

dyn_any_debug_clone_eq_hash!(pub DynVal : Value);

#[derive(Clone, PartialEq, Eq, Hash, From, IsVariant)]
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

    Link(LinkVal),
    Cfg(CfgVal),
    Ctx(CtxVal),
    Func(FuncVal),

    Dyn(Box<dyn DynVal>),
}

pub type LinkVal = Link<Val>;

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
pub(crate) const LINK: &str = "link";
pub(crate) const CFG: &str = "configuration";
pub(crate) const CTX: &str = "context";
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

impl From<Number> for Val {
    fn from(value: Number) -> Self {
        Val::Number(NumberVal::from(value))
    }
}

impl From<Byte> for Val {
    fn from(value: Byte) -> Self {
        Val::Byte(ByteVal::from(value))
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

impl From<Map<Val, Val>> for Val {
    fn from(value: Map<Val, Val>) -> Self {
        Val::Map(MapVal::from(value))
    }
}

macro_rules! match_val {
    ($self:ident, $name:ident => $body:expr) => {
        match $self {
            $crate::semantics::val::Val::Unit($name) => $body,
            $crate::semantics::val::Val::Bit($name) => $body,
            $crate::semantics::val::Val::Symbol($name) => $body,
            $crate::semantics::val::Val::Text($name) => $body,
            $crate::semantics::val::Val::Int($name) => $body,
            $crate::semantics::val::Val::Number($name) => $body,
            $crate::semantics::val::Val::Byte($name) => $body,
            $crate::semantics::val::Val::Pair($name) => $body,
            $crate::semantics::val::Val::Call($name) => $body,
            $crate::semantics::val::Val::List($name) => $body,
            $crate::semantics::val::Val::Map($name) => $body,
            $crate::semantics::val::Val::Link($name) => $body,
            $crate::semantics::val::Val::Cfg($name) => $body,
            $crate::semantics::val::Val::Ctx($name) => $body,
            $crate::semantics::val::Val::Func($name) => $body,
            $crate::semantics::val::Val::Dyn($name) => $body,
        }
    };
}

#[expect(redundant_imports)]
pub(crate) use match_val;

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match_val!(self, v => <_ as Debug>::fmt(v, f))
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

mod cfg;

mod ctx;

mod func;
