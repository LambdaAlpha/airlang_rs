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
            add: add(),
            subtract: subtract(),
            multiply: multiply(),
            divide: divide(),
            less: less(),
            less_equal: less_equal(),
            greater: greater(),
            greater_equal: greater_equal(),
            less_greater: less_greater(),
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

fn decimal_config(tag: &str, cfg: &mut Cfg) -> Option<DecimalConfig> {
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

pub fn add() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_add }.build()
}

fn fn_add(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{ADD}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{ADD}: expected input.left to be a decimal, but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{ADD}: expected input.right to be a decimal, but got {}", pair.right);
    };
    let Some(config) = decimal_config(ADD, cfg) else {
        return Val::default();
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    Val::Decimal(d1.add(d2, config).into())
}

pub fn subtract() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_subtract }.build()
}

fn fn_subtract(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{SUBTRACT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{SUBTRACT}: expected input.left to be a decimal, but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{SUBTRACT}: expected input.right to be a decimal, \
            but got {}", pair.right);
    };
    let Some(config) = decimal_config(SUBTRACT, cfg) else {
        return Val::default();
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    Val::Decimal(d1.subtract(d2, config).into())
}

pub fn multiply() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_multiply }.build()
}

fn fn_multiply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{MULTIPLY}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{MULTIPLY}: expected input.left to be a decimal, but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{MULTIPLY}: expected input.right to be a decimal, \
            but got {}", pair.right);
    };
    let Some(config) = decimal_config(MULTIPLY, cfg) else {
        return Val::default();
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    Val::Decimal(d1.multiply(d2, config).into())
}

pub fn divide() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_divide }.build()
}

fn fn_divide(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{DIVIDE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{DIVIDE}: expected input.left to be a decimal, but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{DIVIDE}: expected input.right to be a decimal, but got {}", pair.right);
    };
    if d2.is_zero() {
        return bug!(cfg, "{DIVIDE}: expected input.right to be non-zero");
    }
    let Some(config) = decimal_config(DIVIDE, cfg) else {
        return Val::default();
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    let d = d1.divide(d2, config);
    Val::Decimal(d.into())
}

pub fn less() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_less }.build()
}

fn fn_less(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LESS}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{LESS}: expected input.left to be a decimal, but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{LESS}: expected input.right to be a decimal, but got {}", pair.right);
    };
    Val::Bit(d1.less_than(&d2))
}

pub fn less_equal() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_less_equal }.build()
}

fn fn_less_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LESS_EQUAL}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{LESS_EQUAL}: expected input.left to be a decimal, \
            but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{LESS_EQUAL}: expected input.right to be a decimal, \
            but got {}", pair.right);
    };
    Val::Bit(d1.less_equal(&d2))
}

pub fn greater() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_greater }.build()
}

fn fn_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{GREATER}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{GREATER}: expected input.left to be a decimal, but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{GREATER}: expected input.right to be a decimal, \
            but got {}", pair.right);
    };
    Val::Bit(d1.greater_than(&d2))
}

pub fn greater_equal() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_greater_equal }.build()
}

fn fn_greater_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{GREATER_EQUAL}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{GREATER_EQUAL}: expected input.left to be a decimal, \
            but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{GREATER_EQUAL}: expected input.right to be a decimal, \
            but got {}", pair.right);
    };
    Val::Bit(d1.greater_equal(&d2))
}

pub fn less_greater() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_less_greater }.build()
}

fn fn_less_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LESS_GREATER}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.left else {
        return bug!(cfg, "{LESS_GREATER}: expected input.left to be a decimal, \
            but got {}", pair.left);
    };
    let Val::Decimal(d2) = pair.right else {
        return bug!(cfg, "{LESS_GREATER}: expected input.right to be a decimal, \
            but got {}", pair.right);
    };
    Val::Bit(d1.less_greater(&d2))
}
