use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Deref;
use std::str::FromStr;

use crate::cfg::repr::func::generate_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::CFG;
use crate::semantics::val::DynVal;
use crate::semantics::val::FUNC;
use crate::semantics::val::FuncVal;
use crate::semantics::val::LINK;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::syntax::FmtCtx;
use crate::syntax::FmtRepr;
use crate::syntax::ParseError;
use crate::syntax::ParseRepr;
use crate::syntax::parse;
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

impl ParseRepr for Val {}

impl FromStr for Val {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl Debug for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl FmtRepr for Val {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::Unit(unit) => <Unit as Display>::fmt(unit, f),
            Val::Bit(bit) => <Bit as Display>::fmt(bit, f),
            Val::Key(key) => <Key as Display>::fmt(key, f),
            Val::Text(text) => <Text as Display>::fmt(text, f),
            Val::Int(int) => <Int as Display>::fmt(int, f),
            Val::Decimal(decimal) => <Decimal as Display>::fmt(decimal, f),
            Val::Byte(byte) => <Byte as Display>::fmt(byte, f),
            Val::Cell(cell) => <Cell<Val> as FmtRepr>::fmt(cell, ctx, f),
            Val::Pair(pair) => <Pair<Val, Val> as FmtRepr>::fmt(pair, ctx, f),
            Val::List(list) => <List<Val> as FmtRepr>::fmt(list, ctx, f),
            Val::Map(map) => <Map<Key, Val> as FmtRepr>::fmt(map, ctx, f),
            Val::Quote(quote) => <Quote<Val> as FmtRepr>::fmt(quote, ctx, f),
            Val::Call(call) => <Call<Val, Val> as FmtRepr>::fmt(call, ctx, f),
            Val::Link(link) => <LinkVal as FmtRepr>::fmt(link, ctx, f),
            Val::Cfg(cfg) => <Cfg as FmtRepr>::fmt(cfg, ctx, f),
            Val::Func(func) => <FuncVal as FmtRepr>::fmt(func, ctx, f),
            Val::Dyn(val) => <dyn DynVal as Display>::fmt(val.deref(), f),
        }
    }

    fn is_call(&self) -> bool {
        matches!(self, Val::Call(_))
    }

    fn is_pair(&self) -> bool {
        matches!(self, Val::Pair(_))
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        let Val::Pair(pair) = self else { panic!("called `FmtRepr::to_pair()` on non-pair value") };
        Pair::new(&pair.left, &pair.right)
    }

    fn is_text_list_map(&self) -> bool {
        matches!(self, Val::Text(_) | Val::List(_) | Val::Map(_))
    }
}

impl Display for LinkVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl Debug for LinkVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl FmtRepr for LinkVal {
    fn fmt(&self, _ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(LINK)?;
        let id = self.ptr_addr();
        let id = Key::from_string_unchecked(format!("{id:x}"));
        let repr = Val::Key(id);
        write!(f, "{repr:+}")
    }
}

impl Display for Cfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl Debug for Cfg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl FmtRepr for Cfg {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(CFG)?;
        FmtRepr::fmt(&**self, ctx, f)
    }
}

impl Display for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl Debug for FuncVal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        FmtRepr::fmt(self, FmtCtx::default(), f)
    }
}

impl FmtRepr for FuncVal {
    fn fmt(&self, ctx: FmtCtx, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(FUNC)?;
        let repr = generate_func(self.clone());
        FmtRepr::fmt(&*repr, ctx, f)
    }
}

pub(in crate::cfg) mod func;
