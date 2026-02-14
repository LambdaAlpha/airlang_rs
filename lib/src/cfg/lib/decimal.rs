use std::num::NonZeroU64;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::val::DECIMAL;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Decimal;
use crate::type_::DecimalConfig;
use crate::type_::Key;
use crate::type_::Pair;
use crate::type_::RoundingMode;

// todo design
#[derive(Clone)]
pub struct DecimalLib {
    pub add: PrimFuncVal,
    pub subtract: PrimFuncVal,
    pub multiply: PrimFuncVal,
    pub divide: PrimFuncVal,
    pub less: PrimFuncVal,
    pub less_equal: PrimFuncVal,
    pub greater: PrimFuncVal,
    pub greater_equal: PrimFuncVal,
    pub less_greater: PrimFuncVal,
}

pub const ADD: &str = concatcp!(PREFIX_ID, DECIMAL, ".add");
pub const SUBTRACT: &str = concatcp!(PREFIX_ID, DECIMAL, ".subtract");
pub const MULTIPLY: &str = concatcp!(PREFIX_ID, DECIMAL, ".multiply");
pub const DIVIDE: &str = concatcp!(PREFIX_ID, DECIMAL, ".divide");
pub const LESS: &str = concatcp!(PREFIX_ID, DECIMAL, ".less");
pub const LESS_EQUAL: &str = concatcp!(PREFIX_ID, DECIMAL, ".less_equal");
pub const GREATER: &str = concatcp!(PREFIX_ID, DECIMAL, ".greater");
pub const GREATER_EQUAL: &str = concatcp!(PREFIX_ID, DECIMAL, ".greater_equal");
pub const LESS_GREATER: &str = concatcp!(PREFIX_ID, DECIMAL, ".less_greater");

impl Default for DecimalLib {
    fn default() -> Self {
        DecimalLib {
            add: CtxFreeInputEvalFunc { fn_: add }.build(),
            subtract: CtxFreeInputEvalFunc { fn_: subtract }.build(),
            multiply: CtxFreeInputEvalFunc { fn_: multiply }.build(),
            divide: CtxFreeInputEvalFunc { fn_: divide }.build(),
            less: CtxFreeInputEvalFunc { fn_: less }.build(),
            less_equal: CtxFreeInputEvalFunc { fn_: less_equal }.build(),
            greater: CtxFreeInputEvalFunc { fn_: greater }.build(),
            greater_equal: CtxFreeInputEvalFunc { fn_: greater_equal }.build(),
            less_greater: CtxFreeInputEvalFunc { fn_: less_greater }.build(),
        }
    }
}

impl CfgMod for DecimalLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, ADD, self.add);
        extend_func(cfg, SUBTRACT, self.subtract);
        extend_func(cfg, MULTIPLY, self.multiply);
        extend_func(cfg, DIVIDE, self.divide);
        extend_func(cfg, LESS, self.less);
        extend_func(cfg, LESS_EQUAL, self.less_equal);
        extend_func(cfg, GREATER, self.greater);
        extend_func(cfg, GREATER_EQUAL, self.greater_equal);
        extend_func(cfg, LESS_GREATER, self.less_greater);
    }
}

pub const ROUNDING_MODE: &str = "_decimal.rounding.mode";
pub const ROUNDING_PRECISION: &str = "_decimal.rounding.precision";

fn decimal_config(cfg: &mut Cfg, tag: &str) -> Option<DecimalConfig> {
    let Some(mode) = cfg.import(Key::from_str_unchecked(ROUNDING_MODE)) else {
        bug!(cfg, "{tag}: config {ROUNDING_MODE} not found");
        return None;
    };
    let Val::Key(mode) = mode else {
        bug!(cfg, "{tag}: expected config {ROUNDING_MODE} to be a key, but got {mode}");
        return None;
    };
    let mode = parse_rounding_mode(&mode)?;
    let Some(precision) = cfg.import(Key::from_str_unchecked(ROUNDING_PRECISION)) else {
        bug!(cfg, "{tag}: config {ROUNDING_PRECISION} not found");
        return None;
    };
    let Val::Int(precision) = precision else {
        bug!(cfg, "{tag}: expected config {ROUNDING_PRECISION} to be an integer, \
            but got {precision}");
        return None;
    };
    let precision: u64 = precision.unwrap().unwrap().try_into().ok()?;
    let precision = NonZeroU64::new(precision)?;
    let config = DecimalConfig::new(precision, mode);
    Some(config)
}

const MODE_INFINITY: &str = concatcp!(PREFIX_ID, "infinity");
const MODE_ZERO: &str = concatcp!(PREFIX_ID, "zero");
const MODE_POSITIVE: &str = concatcp!(PREFIX_ID, "positive");
const MODE_NEGATIVE: &str = concatcp!(PREFIX_ID, "negative");
const MODE_HALF_INFINITY: &str = concatcp!(PREFIX_ID, "half_infinity");
const MODE_HALF_ZERO: &str = concatcp!(PREFIX_ID, "half_zero");
const MODE_HALF_EVEN: &str = concatcp!(PREFIX_ID, "half_even");

fn parse_rounding_mode(key: &str) -> Option<RoundingMode> {
    let mode = match key {
        MODE_INFINITY => RoundingMode::Infinity,
        MODE_ZERO => RoundingMode::Zero,
        MODE_POSITIVE => RoundingMode::Positive,
        MODE_NEGATIVE => RoundingMode::Negative,
        MODE_HALF_INFINITY => RoundingMode::HalfInfinity,
        MODE_HALF_ZERO => RoundingMode::HalfZero,
        MODE_HALF_EVEN => RoundingMode::HalfEven,
        _ => return None,
    };
    Some(mode)
}

pub fn add(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, ADD, input) else {
        return Val::default();
    };
    let Some(config) = decimal_config(cfg, ADD) else {
        return Val::default();
    };
    Val::Decimal(d1.add(d2, config).into())
}

pub fn subtract(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, SUBTRACT, input) else {
        return Val::default();
    };
    let Some(config) = decimal_config(cfg, SUBTRACT) else {
        return Val::default();
    };
    Val::Decimal(d1.subtract(d2, config).into())
}

pub fn multiply(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, MULTIPLY, input) else {
        return Val::default();
    };
    let Some(config) = decimal_config(cfg, MULTIPLY) else {
        return Val::default();
    };
    Val::Decimal(d1.multiply(d2, config).into())
}

pub fn divide(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, DIVIDE, input) else {
        return Val::default();
    };
    if d2.is_zero() {
        return bug!(cfg, "{DIVIDE}: expected input.right to be non-zero");
    }
    let Some(config) = decimal_config(cfg, DIVIDE) else {
        return Val::default();
    };
    let d = d1.divide(d2, config);
    Val::Decimal(d.into())
}

pub fn less(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, LESS, input) else {
        return Val::default();
    };
    Val::Bit(d1.less(&d2))
}

pub fn less_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, LESS_EQUAL, input) else {
        return Val::default();
    };
    Val::Bit(d1.less_equal(&d2))
}

pub fn greater(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, GREATER, input) else {
        return Val::default();
    };
    Val::Bit(d1.greater(&d2))
}

pub fn greater_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, GREATER_EQUAL, input) else {
        return Val::default();
    };
    Val::Bit(d1.greater_equal(&d2))
}

pub fn less_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Some((d1, d2)) = decimal_pair(cfg, LESS_GREATER, input) else {
        return Val::default();
    };
    Val::Bit(d1.less_greater(&d2))
}

fn decimal_pair(cfg: &mut Cfg, tag: &str, input: Val) -> Option<(Decimal, Decimal)> {
    let Val::Pair(pair) = input else {
        bug!(cfg, "{tag}: expected input to be a pair, but got {input}");
        return None;
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        bug!(cfg, "{tag}: expected input.left to be a decimal, but got {}", pair.left);
        return None;
    };
    let Val::Decimal(d2) = pair.right else {
        bug!(cfg, "{tag}: expected input.right to be a decimal, but got {}", pair.right);
        return None;
    };
    Some((d1.into(), d2.into()))
}
