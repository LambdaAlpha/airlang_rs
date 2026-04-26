use const_format::concatcp;
use rand::distr::Distribution;

_____!();

use rand::rng;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::cfg::prim::lib::value::arbitrary::Any;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxFreeInputAwareFunc;
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
use crate::semantics::val::QUOTE;
use crate::semantics::val::SOLVE;
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
use crate::type_::Quote;
use crate::type_::Solve;
use crate::type_::Text;
use crate::type_::Unit;

#[derive(Copy, Clone)]
pub struct ValueLib {
    /// should be overridden if there are extension types
    pub any: PrimFuncVal,
    pub get_type: PrimFuncVal,
    pub equal: PrimFuncVal,
}

const VALUE: &str = "value";

pub const ANY: &str = concatcp!(PREFIX_CELL, VALUE, ".any");
pub const GET_TYPE: &str = concatcp!(PREFIX_CELL, VALUE, ".get_type");
pub const EQUAL: &str = concatcp!(PREFIX_CELL, VALUE, ".equal");

impl Default for ValueLib {
    fn default() -> Self {
        Self {
            any: CtxFreeInputAwareFunc { fn_: any }.build(),
            get_type: CtxConstInputFreeFunc { fn_: get_type }.build(),
            equal: CtxFreeInputAwareFunc { fn_: equal }.build(),
        }
    }
}

impl CfgMod for ValueLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, ANY, self.any);
        extend_func(cfg, GET_TYPE, self.get_type);
        extend_func(cfg, EQUAL, self.equal);
    }
}

const TYPE_UNIT: &str = concatcp!(PREFIX_CELL, UNIT);
const TYPE_BIT: &str = concatcp!(PREFIX_CELL, BIT);
const TYPE_KEY: &str = concatcp!(PREFIX_CELL, KEY);
const TYPE_TEXT: &str = concatcp!(PREFIX_CELL, TEXT);
const TYPE_INT: &str = concatcp!(PREFIX_CELL, INT);
const TYPE_DECIMAL: &str = concatcp!(PREFIX_CELL, DECIMAL);
const TYPE_BYTE: &str = concatcp!(PREFIX_CELL, BYTE);
const TYPE_CELL: &str = concatcp!(PREFIX_CELL, CELL);
const TYPE_PAIR: &str = concatcp!(PREFIX_CELL, PAIR);
const TYPE_LIST: &str = concatcp!(PREFIX_CELL, LIST);
const TYPE_MAP: &str = concatcp!(PREFIX_CELL, MAP);
const TYPE_QUOTE: &str = concatcp!(PREFIX_CELL, QUOTE);
const TYPE_CALL: &str = concatcp!(PREFIX_CELL, CALL);
const TYPE_SOLVE: &str = concatcp!(PREFIX_CELL, SOLVE);
const TYPE_LINK: &str = concatcp!(PREFIX_CELL, LINK);
const TYPE_CFG: &str = concatcp!(PREFIX_CELL, CFG);
const TYPE_FUNC: &str = concatcp!(PREFIX_CELL, FUNC);

const SYNTAX: &str = concatcp!(PREFIX_CELL, "syntax");

pub fn any(cfg: &mut Cfg, input: Val) -> Val {
    let mut rng = rng();
    let rng = &mut rng;
    match input {
        Val::Unit(_) => Any { syntax: false }.sample(rng),
        Val::Key(s) => match &*s {
            SYNTAX => Any { syntax: true }.sample(rng),
            TYPE_UNIT => Val::Unit(Distribution::<Unit>::sample(&Any::default(), rng)),
            TYPE_BIT => Val::Bit(Distribution::<Bit>::sample(&Any::default(), rng)),
            TYPE_KEY => Val::Key(Distribution::<Key>::sample(&Any::default(), rng)),
            TYPE_TEXT => Val::Text(Distribution::<Text>::sample(&Any::default(), rng).into()),
            TYPE_INT => Val::Int(Distribution::<Int>::sample(&Any::default(), rng).into()),
            TYPE_DECIMAL => {
                Val::Decimal(Distribution::<Decimal>::sample(&Any::default(), rng).into())
            },
            TYPE_BYTE => Val::Byte(Distribution::<Byte>::sample(&Any::default(), rng).into()),
            TYPE_CELL => Val::Cell(Distribution::<Cell<Val>>::sample(&Any::default(), rng).into()),
            TYPE_PAIR => {
                Val::Pair(Distribution::<Pair<Val, Val>>::sample(&Any::default(), rng).into())
            },
            TYPE_LIST => Val::List(Distribution::<List<Val>>::sample(&Any::default(), rng).into()),
            TYPE_MAP => {
                Val::Map(Distribution::<Map<Key, Val>>::sample(&Any::default(), rng).into())
            },
            TYPE_QUOTE => {
                Val::Quote(Distribution::<Quote<Val>>::sample(&Any::default(), rng).into())
            },
            TYPE_CALL => {
                Val::Call(Distribution::<Call<Val, Val>>::sample(&Any::default(), rng).into())
            },
            TYPE_SOLVE => {
                Val::Solve(Distribution::<Solve<Val, Val>>::sample(&Any::default(), rng).into())
            },
            TYPE_LINK => Val::Link(Distribution::<LinkVal>::sample(&Any::default(), rng)),
            TYPE_CFG => Val::Cfg(Distribution::<Cfg>::sample(&Any::default(), rng).into()),
            TYPE_FUNC => Val::Func(Distribution::<FuncVal>::sample(&Any::default(), rng)),
            s => bug!(cfg, "{ANY}: unknown type {s}"),
        },
        v => bug!(cfg, "{ANY}: expected input to be a key or a unit, but got {v}"),
    }
}

pub fn get_type(_cfg: &mut Cfg, ctx: &Val) -> Val {
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
        Val::List(_) => TYPE_LIST,
        Val::Map(_) => TYPE_MAP,
        Val::Quote(_) => TYPE_QUOTE,
        Val::Call(_) => TYPE_CALL,
        Val::Solve(_) => TYPE_SOLVE,
        Val::Link(_) => TYPE_LINK,
        Val::Cfg(_) => TYPE_CFG,
        Val::Func(_) => TYPE_FUNC,
        Val::Dyn(val) => return Val::Key(val.type_name()),
    };
    Val::Key(Key::from_str_unchecked(s))
}

// todo design
pub fn equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{EQUAL}: expected input to be a pair, but got {input}");
    };
    Val::Bit(Bit::from(pair.left == pair.right))
}

mod arbitrary;
