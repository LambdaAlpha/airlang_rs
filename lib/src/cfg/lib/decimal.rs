use std::num::NonZeroU64;

use const_format::concatcp;
use log::error;

use crate::cfg::CfgMod;
use crate::cfg::error::illegal_cfg;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Decimal;
use crate::type_::DecimalConfig;
use crate::type_::Key;
use crate::type_::Pair;
use crate::type_::RoundingMode;

// todo design
#[derive(Clone)]
pub struct DecimalLib {
    pub add: FreePrimFuncVal,
    pub subtract: FreePrimFuncVal,
    pub multiply: FreePrimFuncVal,
    pub divide: FreePrimFuncVal,
    pub less: FreePrimFuncVal,
    pub less_equal: FreePrimFuncVal,
    pub greater: FreePrimFuncVal,
    pub greater_equal: FreePrimFuncVal,
    pub less_greater: FreePrimFuncVal,
}

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
        extend_func(cfg, "_decimal.add", self.add);
        extend_func(cfg, "_decimal.subtract", self.subtract);
        extend_func(cfg, "_decimal.multiply", self.multiply);
        extend_func(cfg, "_decimal.divide", self.divide);
        extend_func(cfg, "_decimal.less", self.less);
        extend_func(cfg, "_decimal.less_equal", self.less_equal);
        extend_func(cfg, "_decimal.greater", self.greater);
        extend_func(cfg, "_decimal.greater_equal", self.greater_equal);
        extend_func(cfg, "_decimal.less_greater", self.less_greater);
    }
}

impl DecimalLib {
    pub const ROUNDING_MODE: &str = "_decimal.rounding.mode";
    pub const ROUNDING_PRECISION: &str = "_decimal.rounding.precision";

    pub fn decimal_config(cfg: &Cfg) -> Option<DecimalConfig> {
        let mode = cfg.import(Key::from_str_unchecked(Self::ROUNDING_MODE))?;
        let Val::Key(mode) = mode else {
            return None;
        };
        let mode = parse_rounding_mode(&mode)?;
        let precision = cfg.import(Key::from_str_unchecked(Self::ROUNDING_PRECISION))?;
        let Val::Int(precision) = precision else {
            return None;
        };
        let precision: u64 = precision.unwrap().unwrap().try_into().ok()?;
        let precision = NonZeroU64::new(precision)?;
        let config = DecimalConfig::new(precision, mode);
        Some(config)
    }
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

pub fn add() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_add) }.free()
}

fn fn_add(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    let Some(config) = DecimalLib::decimal_config(cfg) else {
        error!("decimal config should exist and be valid");
        return illegal_cfg(cfg);
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    Val::Decimal(d1.add(d2, config).into())
}

pub fn subtract() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_subtract) }.free()
}

fn fn_subtract(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    let Some(config) = DecimalLib::decimal_config(cfg) else {
        error!("decimal config should exist and be valid");
        return illegal_cfg(cfg);
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    Val::Decimal(d1.subtract(d2, config).into())
}

pub fn multiply() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_multiply) }.free()
}

fn fn_multiply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    let Some(config) = DecimalLib::decimal_config(cfg) else {
        error!("decimal config should exist and be valid");
        return illegal_cfg(cfg);
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    Val::Decimal(d1.multiply(d2, config).into())
}

pub fn divide() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_divide) }.free()
}

fn fn_divide(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    if d2.is_zero() {
        error!("input.second should not be zero");
        return illegal_input(cfg);
    }
    let Some(config) = DecimalLib::decimal_config(cfg) else {
        error!("decimal config should exist and be valid");
        return illegal_cfg(cfg);
    };
    let d1 = Decimal::from(d1);
    let d2 = Decimal::from(d2);
    let d = d1.divide(d2, config);
    Val::Decimal(d.into())
}

pub fn less() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_less) }.free()
}

fn fn_less(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(d1.less_than(&d2))
}

pub fn less_equal() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_less_equal) }.free()
}

fn fn_less_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(d1.less_equal(&d2))
}

pub fn greater() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_greater) }.free()
}

fn fn_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(d1.greater_than(&d2))
}

pub fn greater_equal() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_greater_equal) }.free()
}

fn fn_greater_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(d1.greater_equal(&d2))
}

pub fn less_greater() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_less_greater) }.free()
}

fn fn_less_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Decimal(d1) = pair.first else {
        error!("input.first {:?} should be a decimal", pair.first);
        return illegal_input(cfg);
    };
    let Val::Decimal(d2) = pair.second else {
        error!("input.second {:?} should be a decimal", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(d1.less_greater(&d2))
}
