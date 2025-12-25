use log::error;

use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct IntLib {
    pub add: FreePrimFuncVal,
    pub subtract: FreePrimFuncVal,
    pub multiply: FreePrimFuncVal,
    pub divide: FreePrimFuncVal,
    pub remainder: FreePrimFuncVal,
    pub divide_remainder: FreePrimFuncVal,
    pub less: FreePrimFuncVal,
    pub less_equal: FreePrimFuncVal,
    pub greater: FreePrimFuncVal,
    pub greater_equal: FreePrimFuncVal,
    pub less_greater: FreePrimFuncVal,
}

impl Default for IntLib {
    fn default() -> Self {
        IntLib {
            add: add(),
            subtract: subtract(),
            multiply: multiply(),
            divide: divide(),
            remainder: remainder(),
            divide_remainder: divide_remainder(),
            less: less(),
            less_equal: less_equal(),
            greater: greater(),
            greater_equal: greater_equal(),
            less_greater: less_greater(),
        }
    }
}

impl CfgMod for IntLib {
    fn extend(self, cfg: &Cfg) {
        self.add.extend(cfg);
        self.subtract.extend(cfg);
        self.multiply.extend(cfg);
        self.divide.extend(cfg);
        self.remainder.extend(cfg);
        self.divide_remainder.extend(cfg);
        self.less.extend(cfg);
        self.less_equal.extend(cfg);
        self.greater.extend(cfg);
        self.greater_equal.extend(cfg);
        self.less_greater.extend(cfg);
    }
}

pub fn add() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.add", raw_input: false, f: free_impl(fn_add) }.free()
}

fn fn_add(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.add(i2).into())
}

pub fn subtract() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.subtract", raw_input: false, f: free_impl(fn_subtract) }.free()
}

fn fn_subtract(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.subtract(i2).into())
}

pub fn multiply() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.multiply", raw_input: false, f: free_impl(fn_multiply) }.free()
}

fn fn_multiply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.multiply(i2).into())
}

pub fn divide() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.divide", raw_input: false, f: free_impl(fn_divide) }.free()
}

fn fn_divide(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.divide(i2) else {
        return illegal_input(cfg);
    };
    Val::Int(i.into())
}

pub fn remainder() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.remainder", raw_input: false, f: free_impl(fn_remainder) }.free()
}

fn fn_remainder(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.remainder(i2) else {
        return illegal_input(cfg);
    };
    Val::Int(i.into())
}

pub fn divide_remainder() -> FreePrimFuncVal {
    FreePrimFn {
        id: "_integer.divide_remainder",
        raw_input: false,
        f: free_impl(fn_divide_remainder),
    }
    .free()
}

fn fn_divide_remainder(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return illegal_input(cfg);
    };
    Val::Pair(Pair::new(Val::Int(quotient.into()), Val::Int(rem.into())).into())
}

pub fn less() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.less", raw_input: false, f: free_impl(fn_less) }.free()
}

fn fn_less(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(i1.less_than(&i2))
}

pub fn less_equal() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.less_equal", raw_input: false, f: free_impl(fn_less_equal) }.free()
}

fn fn_less_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(i1.less_equal(&i2))
}

pub fn greater() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.greater", raw_input: false, f: free_impl(fn_greater) }.free()
}

fn fn_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(i1.greater_than(&i2))
}

pub fn greater_equal() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.greater_equal", raw_input: false, f: free_impl(fn_greater_equal) }
        .free()
}

fn fn_greater_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(i1.greater_equal(&i2))
}

pub fn less_greater() -> FreePrimFuncVal {
    FreePrimFn { id: "_integer.less_greater", raw_input: false, f: free_impl(fn_less_greater) }
        .free()
}

fn fn_less_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return illegal_input(cfg);
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return illegal_input(cfg);
    };
    Val::Bit(i1.less_greater(&i2))
}
