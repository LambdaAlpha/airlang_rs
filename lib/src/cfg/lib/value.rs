use const_format::concatcp;

pub use self::arbitrary::Arbitrary;

_____!();

use rand::SeedableRng;
use rand::prelude::SmallRng;

use super::ConstImpl;
use super::FreeImpl;
use super::ImplExtra;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::BIT;
use crate::semantics::val::BYTE;
use crate::semantics::val::CALL;
use crate::semantics::val::CELL;
use crate::semantics::val::CFG;
use crate::semantics::val::DECIMAL;
use crate::semantics::val::FUNC;
use crate::semantics::val::FuncVal;
use crate::semantics::val::INT;
use crate::semantics::val::KEY;
use crate::semantics::val::LINK;
use crate::semantics::val::LIST;
use crate::semantics::val::LinkVal;
use crate::semantics::val::MAP;
use crate::semantics::val::PAIR;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::TEXT;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
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

#[derive(Clone)]
pub struct ValueLib {
    /// should be overridden if there are extension types
    pub any: PrimFuncVal,
    pub get_type: PrimFuncVal,
    pub equal: PrimFuncVal,
}

const VALUE: &str = "value";

pub const ANY: &str = concatcp!(PREFIX_ID, VALUE, ".any");
pub const GET_TYPE: &str = concatcp!(PREFIX_ID, VALUE, ".get_type");
pub const EQUAL: &str = concatcp!(PREFIX_ID, VALUE, ".equal");

impl Default for ValueLib {
    fn default() -> Self {
        ValueLib { any: any(), get_type: get_type(), equal: equal() }
    }
}

impl CfgMod for ValueLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, ANY, self.any);
        extend_func(cfg, GET_TYPE, self.get_type);
        extend_func(cfg, EQUAL, self.equal);
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
pub fn any() -> PrimFuncVal {
    FreeImpl { fn_: fn_any }.build(ImplExtra { raw_input: false })
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
            s => bug!(cfg, "{ANY}: unknown type {s}"),
        },
        v => bug!(cfg, "{ANY}: expected input to be a key or a unit, but got {v}"),
    }
}

pub fn get_type() -> PrimFuncVal {
    ConstImpl { fn_: fn_get_type }.build(ImplExtra { raw_input: false })
}

fn fn_get_type(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    if !input.is_unit() {
        return bug!(cfg, "{GET_TYPE}: expected input to be a unit, but got {input}");
    }
    let s = match ctx {
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
pub fn equal() -> PrimFuncVal {
    FreeImpl { fn_: fn_equal }.build(ImplExtra { raw_input: false })
}

fn fn_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{EQUAL}: expected input to be a pair, but got {input}");
    };
    Val::Bit(Bit::from(pair.left == pair.right))
}

mod arbitrary;
