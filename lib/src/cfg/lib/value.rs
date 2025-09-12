pub use self::arbitrary::Arbitrary;
pub use self::arbitrary::ArbitraryVal;

_____!();

use log::error;
use rand::SeedableRng;
use rand::prelude::SmallRng;

use self::arbitrary::arbitrary_ext_type;
use self::arbitrary::set_arbitrary_val;
use super::DynPrimFn;
use super::FreePrimFn;
use super::Library;
use super::const_impl;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::mode::CallPrimMode;
use crate::cfg::mode::FuncMode;
use crate::cfg::mode::SymbolMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::memo::Memo;
use crate::semantics::val::BIT;
use crate::semantics::val::BYTE;
use crate::semantics::val::CALL;
use crate::semantics::val::CFG;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FUNC;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::FuncVal;
use crate::semantics::val::INT;
use crate::semantics::val::LINK;
use crate::semantics::val::LIST;
use crate::semantics::val::MAP;
use crate::semantics::val::MEMO;
use crate::semantics::val::NUMBER;
use crate::semantics::val::PAIR;
use crate::semantics::val::SYMBOL;
use crate::semantics::val::TEXT;
use crate::semantics::val::UNIT;
use crate::semantics::val::Val;
use crate::type_::Bit;
use crate::type_::Byte;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Link;
use crate::type_::List;
use crate::type_::Map;
use crate::type_::Number;
use crate::type_::Pair;
use crate::type_::Symbol;
use crate::type_::Text;
use crate::type_::Unit;

pub fn init_arbitrary(arbitrary_val: Box<dyn ArbitraryVal>) {
    set_arbitrary_val(arbitrary_val);
}

#[derive(Clone)]
pub struct ValueLib {
    pub any: FreePrimFuncVal,
    pub type_: ConstPrimFuncVal,
    pub equal: FreePrimFuncVal,
}

impl Default for ValueLib {
    fn default() -> Self {
        ValueLib { any: any(), type_: type_(), equal: equal() }
    }
}

impl CfgMod for ValueLib {
    fn extend(self, cfg: &Cfg) {
        let any_adapter = FuncMode::prim_mode(SymbolMode::Literal, CallPrimMode::Form);
        CoreCfg::extend_adapter_mode(cfg, &self.any.id, any_adapter);
        self.any.extend(cfg);
        self.type_.extend(cfg);
        self.equal.extend(cfg);
    }
}

impl Library for ValueLib {
    fn prelude(&self, memo: &mut Memo) {
        self.any.prelude(memo);
        self.type_.prelude(memo);
        self.equal.prelude(memo);
    }
}

// todo design pick value from ctx
pub fn any() -> FreePrimFuncVal {
    FreePrimFn { id: "any", f: free_impl(fn_any) }.free()
}

fn fn_any(_cfg: &mut Cfg, input: Val) -> Val {
    const DEPTH: usize = 0;
    let mut rng = SmallRng::from_os_rng();
    let rng = &mut rng;
    match input {
        Val::Unit(_) => Val::any(rng, DEPTH),
        Val::Symbol(s) => match &*s {
            UNIT => Val::Unit(Unit::any(rng, DEPTH)),
            BIT => Val::Bit(Bit::any(rng, DEPTH)),
            SYMBOL => Val::Symbol(Symbol::any(rng, DEPTH)),
            TEXT => Val::Text(Text::any(rng, DEPTH).into()),
            INT => Val::Int(Int::any(rng, DEPTH).into()),
            NUMBER => Val::Number(Number::any(rng, DEPTH).into()),
            BYTE => Val::Byte(Byte::any(rng, DEPTH).into()),
            PAIR => Val::Pair(Pair::<Val, Val>::any(rng, DEPTH).into()),
            CALL => Val::Call(Call::<Val, Val>::any(rng, DEPTH).into()),
            LIST => Val::List(List::<Val>::any(rng, DEPTH).into()),
            MAP => Val::Map(Map::<Val, Val>::any(rng, DEPTH).into()),
            LINK => Val::Link(Link::any(rng, DEPTH)),
            CFG => Val::Cfg(Cfg::any(rng, DEPTH).into()),
            MEMO => Val::Memo(Memo::any(rng, DEPTH).into()),
            FUNC => Val::Func(FuncVal::any(rng, DEPTH)),
            _ => arbitrary_ext_type(s),
        },
        v => {
            error!("input {v:?} should be a symbol or a unit");
            Val::default()
        }
    }
}

pub fn type_() -> ConstPrimFuncVal {
    DynPrimFn { id: "type", f: const_impl(fn_type) }.const_()
}

fn fn_type(_cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let s = match &*ctx {
        Val::Unit(_) => UNIT,
        Val::Bit(_) => BIT,
        Val::Symbol(_) => SYMBOL,
        Val::Text(_) => TEXT,
        Val::Int(_) => INT,
        Val::Number(_) => NUMBER,
        Val::Byte(_) => BYTE,
        Val::Pair(_) => PAIR,
        Val::Call(_) => CALL,
        Val::List(_) => LIST,
        Val::Map(_) => MAP,
        Val::Link(_) => LINK,
        Val::Cfg(_) => CFG,
        Val::Memo(_) => MEMO,
        Val::Func(_) => FUNC,
        Val::Dyn(val) => return Val::Symbol(val.type_name()),
    };
    Val::Symbol(Symbol::from_str_unchecked(s))
}

// todo design
pub fn equal() -> FreePrimFuncVal {
    FreePrimFn { id: "==", f: free_impl(fn_equal) }.free()
}

fn fn_equal(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    Val::Bit(Bit::from(pair.first == pair.second))
}

mod arbitrary;
