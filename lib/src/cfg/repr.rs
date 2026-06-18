use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Write;
use std::ops::Deref;
use std::str::FromStr;

use crate::cfg::repr::func::generate_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::fact::Fact;
use crate::semantics::val::CFG;
use crate::semantics::val::FACT;
use crate::semantics::val::FUNC;
use crate::semantics::val::FuncVal;
use crate::semantics::val::LINK;
use crate::semantics::val::LinkVal;
use crate::semantics::val::Val;
use crate::syntax::FmtOptions;
use crate::syntax::FmtRepr;
use crate::syntax::ParseError;
use crate::syntax::ParseRepr;
use crate::syntax::ReprType;
use crate::syntax::impl_display_debug_for_fmt_repr;
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
use crate::type_::Solve;
use crate::type_::Text;
use crate::type_::Unit;

impl ParseRepr for Val {}

impl FromStr for Val {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s)
    }
}

impl FmtRepr for Val {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        match self {
            Val::Unit(unit) => <Unit as FmtRepr>::fmt(unit, options, f),
            Val::Bit(bit) => <Bit as FmtRepr>::fmt(bit, options, f),
            Val::Key(key) => <Key as FmtRepr>::fmt(key, options, f),
            Val::Text(text) => <Text as FmtRepr>::fmt(text, options, f),
            Val::Int(int) => <Int as FmtRepr>::fmt(int, options, f),
            Val::Decimal(decimal) => <Decimal as FmtRepr>::fmt(decimal, options, f),
            Val::Byte(byte) => <Byte as FmtRepr>::fmt(byte, options, f),
            Val::Cell(cell) => <Cell<Val> as FmtRepr>::fmt(cell, options, f),
            Val::Pair(pair) => <Pair<Val, Val> as FmtRepr>::fmt(pair, options, f),
            Val::List(list) => <List<Val> as FmtRepr>::fmt(list, options, f),
            Val::Map(map) => <Map<Key, Val> as FmtRepr>::fmt(map, options, f),
            Val::Quote(quote) => <Quote<Val> as FmtRepr>::fmt(quote, options, f),
            Val::Call(call) => <Call<Val, Val> as FmtRepr>::fmt(call, options, f),
            Val::Solve(solve) => <Solve<Val, Val> as FmtRepr>::fmt(solve, options, f),
            Val::Fact(fact) => <Fact as FmtRepr>::fmt(fact, options, f),
            Val::Link(link) => <LinkVal as FmtRepr>::fmt(link, options, f),
            Val::Cfg(cfg) => <Cfg as FmtRepr>::fmt(cfg, options, f),
            Val::Func(func) => <FuncVal as FmtRepr>::fmt(func, options, f),
            Val::Dyn(val) => write!(f, "{}", val.deref()),
        }
    }

    fn get_type(&self) -> ReprType {
        match self {
            Val::Unit(_) => ReprType::Unit,
            Val::Bit(_) => ReprType::Bit,
            Val::Key(_) => ReprType::Key,
            Val::Text(_) => ReprType::Text,
            Val::Int(_) => ReprType::Int,
            Val::Decimal(_) => ReprType::Decimal,
            Val::Byte(_) => ReprType::Byte,
            Val::Cell(_) => ReprType::Cell,
            Val::Pair(_) => ReprType::Pair,
            Val::List(_) => ReprType::List,
            Val::Map(_) => ReprType::Map,
            Val::Quote(_) => ReprType::Quote,
            Val::Call(_) => ReprType::Call,
            Val::Solve(_) => ReprType::Solve,
            Val::Fact(_) => ReprType::Other,
            Val::Link(_) => ReprType::Other,
            Val::Cfg(_) => ReprType::Other,
            Val::Func(_) => ReprType::Other,
            Val::Dyn(_) => ReprType::Other,
        }
    }

    fn to_pair(&self) -> Pair<&dyn FmtRepr, &dyn FmtRepr> {
        let Val::Pair(pair) = self else { panic!("called `FmtRepr::to_pair()` on non-pair value") };
        Pair::new(&pair.left, &pair.right)
    }
}

impl FmtRepr for Fact {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(FACT)?;
        let mut map: Map<Key, &dyn FmtRepr> = Map::default();
        map.insert(Key::from_str_unchecked("function"), self.func());
        map.insert(Key::from_str_unchecked("input"), self.input());
        map.insert(Key::from_str_unchecked("output"), self.output());
        FmtRepr::fmt(&map, options, f)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Other
    }
}

impl FmtRepr for LinkVal {
    fn fmt(&self, mut options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(LINK)?;
        let id = self.ptr_addr();
        let id = Key::from_string_unchecked(format!("{id:x}"));
        let repr = Val::Key(id);
        options.normalized = true;
        options.key_ctx = true;
        FmtRepr::fmt(&repr, options, f)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Other
    }
}

impl FmtRepr for Cfg {
    fn fmt(&self, options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(CFG)?;
        let mut map: Map<Key, &dyn FmtRepr> = Map::default();
        let aborted = Bit::from(self.is_aborted());
        map.insert(Key::from_str_unchecked("aborted"), &aborted);
        map.insert(Key::from_str_unchecked("map"), &**self);
        FmtRepr::fmt(&map, options, f)
    }

    fn get_type(&self) -> ReprType {
        ReprType::Other
    }
}

impl FmtRepr for FuncVal {
    fn fmt(&self, mut options: FmtOptions, f: &mut dyn Write) -> std::fmt::Result {
        f.write_str(FUNC)?;
        if options.id_mode {
            let id = self.id();
            let id = Key::from_string_unchecked(format!("{id:x}"));
            options.normalized = true;
            options.key_ctx = true;
            FmtRepr::fmt(&id, options, f)
        } else {
            let repr = generate_func(self.clone());
            FmtRepr::fmt(&*repr, options, f)
        }
    }

    fn get_type(&self) -> ReprType {
        ReprType::Other
    }
}

impl_display_debug_for_fmt_repr!(Val);
impl_display_debug_for_fmt_repr!(Fact);
impl_display_debug_for_fmt_repr!(LinkVal);
impl_display_debug_for_fmt_repr!(Cfg);
impl_display_debug_for_fmt_repr!(FuncVal);

pub(in crate::cfg) mod func;
