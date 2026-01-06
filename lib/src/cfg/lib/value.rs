use const_format::concatcp;

pub use self::arbitrary::Arbitrary;

_____!();

use log::error;
use rand::SeedableRng;
use rand::prelude::SmallRng;

use super::DynPrimFn;
use super::FreePrimFn;
use super::const_impl;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::fail;
use crate::cfg::exception::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::BIT;
use crate::semantics::val::BYTE;
use crate::semantics::val::CALL;
use crate::semantics::val::CELL;
use crate::semantics::val::CFG;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::DECIMAL;
use crate::semantics::val::FUNC;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::INT;
use crate::semantics::val::KEY;
use crate::semantics::val::LINK;
use crate::semantics::val::LIST;
use crate::semantics::val::LinkVal;
use crate::semantics::val::MAP;
use crate::semantics::val::PAIR;
use crate::semantics::val::TEXT;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::Cell;
use crate::type_::ConstRef;
use crate::type_::Decimal;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Pair;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(Clone)]
pub struct ValueLib {
    /// should be overridden if there are extension types
    pub any: FreePrimFuncVal,
    pub get_type: ConstPrimFuncVal,
    pub equal: FreePrimFuncVal,
}

impl Default for ValueLib {
    fn default() -> Self {
        ValueLib { any: any(), get_type: get_type(), equal: equal() }
    }
}

impl CfgMod for ValueLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_value.any", self.any);
        extend_func(cfg, "_value.get_type", self.get_type);
        extend_func(cfg, "_value.equal", self.equal);
    }
}

const TYPE_UNIT: &str = concatcp!(PREFIX_ID, UNIT);
const TYPE_BIT: &str = concatcp!(PREFIX_ID, BIT);
const TYPE_KEY: &str = concatcp!(PREFIX_ID, KEY);
const TYPE_TEXT: &str = concatcp!(PREFIX_ID, TEXT);
const TYPE_INT: &str = concatcp!(PREFIX_ID, INT);
const TYPE_DECIMAL: &str = concatcp!(PREFIX_ID, DECIMAL);
const TYPE_BYTE: &str = concatcp!(PREFIX_ID, BYTE);
const TYPE_CELL: &str = concatcp!(PREFIX_ID, CELL);
const TYPE_PAIR: &str = concatcp!(PREFIX_ID, PAIR);
const TYPE_CALL: &str = concatcp!(PREFIX_ID, CALL);
const TYPE_LIST: &str = concatcp!(PREFIX_ID, LIST);
const TYPE_MAP: &str = concatcp!(PREFIX_ID, MAP);
const TYPE_LINK: &str = concatcp!(PREFIX_ID, LINK);
const TYPE_CFG: &str = concatcp!(PREFIX_ID, CFG);
const TYPE_FUNC: &str = concatcp!(PREFIX_ID, FUNC);

// todo design pick value from cfg or ctx
pub fn any() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_any) }.free()
}

fn fn_any(cfg: &mut Cfg, input: Val) -> Val {
    const DEPTH: usize = 0;
    let mut rng = SmallRng::from_os_rng();
    let rng = &mut rng;
    match input {
        Val::Unit(_) => Val::any(rng, DEPTH),
        Val::Key(s) => match &*s {
            TYPE_UNIT => Val::Unit(Unit::any(rng, DEPTH)),
            TYPE_BIT => Val::Bit(Bit::any(rng, DEPTH)),
            TYPE_KEY => Val::Key(Key::any(rng, DEPTH)),
            TYPE_TEXT => Val::Text(Text::any(rng, DEPTH).into()),
            TYPE_INT => Val::Int(Int::any(rng, DEPTH).into()),
            TYPE_DECIMAL => Val::Decimal(Decimal::any(rng, DEPTH).into()),
            TYPE_BYTE => Val::Byte(Byte::any(rng, DEPTH).into()),
            TYPE_CELL => Val::Cell(Cell::<Val>::any(rng, DEPTH).into()),
            TYPE_PAIR => Val::Pair(Pair::<Val, Val>::any(rng, DEPTH).into()),
            TYPE_CALL => Val::Call(Call::<Val, Val>::any(rng, DEPTH).into()),
            TYPE_LIST => Val::List(List::<Val>::any(rng, DEPTH).into()),
            TYPE_MAP => Val::Map(Map::<Key, Val>::any(rng, DEPTH).into()),
            TYPE_LINK => Val::Link(LinkVal::any(rng, DEPTH)),
            TYPE_CFG => Val::Cfg(Cfg::any(rng, DEPTH).into()),
            TYPE_FUNC => Val::Func(FuncVal::any(rng, DEPTH)),
            _ => fail(cfg),
        },
        v => {
            error!("input {v:?} should be a key or a unit");
            illegal_input(cfg)
        }
    }
}

pub fn get_type() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_type) }.const_()
}

fn fn_get_type(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let s = match &*ctx {
        Val::Unit(_) => TYPE_UNIT,
        Val::Bit(_) => TYPE_BIT,
        Val::Key(_) => TYPE_KEY,
        Val::Text(_) => TYPE_TEXT,
        Val::Int(_) => TYPE_INT,
        Val::Decimal(_) => TYPE_DECIMAL,
        Val::Byte(_) => TYPE_BYTE,
        Val::Cell(_) => TYPE_CELL,
        Val::Pair(_) => TYPE_PAIR,
        Val::Call(_) => TYPE_CALL,
        Val::List(_) => TYPE_LIST,
        Val::Map(_) => TYPE_MAP,
        Val::Link(_) => TYPE_LINK,
        Val::Cfg(_) => TYPE_CFG,
        Val::Func(_) => TYPE_FUNC,
        Val::Dyn(val) => return Val::Key(val.type_name()),
    };
    Val::Key(Key::from_str_unchecked(s))
}

// todo design
pub fn equal() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_equal) }.free()
}

fn fn_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    Val::Bit(Bit::from(pair.first == pair.second))
}

mod arbitrary;
