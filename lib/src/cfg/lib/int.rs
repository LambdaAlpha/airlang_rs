use log::error;

use super::FreePrimFn;
use super::Library;
use super::ctx_put_func;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::mode::FuncMode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
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
    pub less_than: FreePrimFuncVal,
    pub less_equal: FreePrimFuncVal,
    pub greater_than: FreePrimFuncVal,
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
            less_than: less_than(),
            less_equal: less_equal(),
            greater_than: greater_than(),
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
        self.less_than.extend(cfg);
        self.less_equal.extend(cfg);
        self.greater_than.extend(cfg);
        self.greater_equal.extend(cfg);
        self.less_greater.extend(cfg);
    }
}

impl Library for IntLib {
    fn prelude(&self, ctx: &mut Ctx) {
        ctx_put_func(ctx, "+", &self.add);
        ctx_put_func(ctx, "-", &self.subtract);
        ctx_put_func(ctx, "*", &self.multiply);
        ctx_put_func(ctx, "/", &self.divide);
        ctx_put_func(ctx, "<", &self.less_than);
        ctx_put_func(ctx, "<=", &self.less_equal);
        ctx_put_func(ctx, ">", &self.greater_than);
        ctx_put_func(ctx, ">=", &self.greater_equal);
    }
}

pub fn add() -> FreePrimFuncVal {
    FreePrimFn { id: "integer.add", f: free_impl(fn_add), mode: FuncMode::default_mode() }.free()
}

fn fn_add(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.add(i2).into())
}

pub fn subtract() -> FreePrimFuncVal {
    FreePrimFn { id: "integer.subtract", f: free_impl(fn_subtract), mode: FuncMode::default_mode() }
        .free()
}

fn fn_subtract(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.subtract(i2).into())
}

pub fn multiply() -> FreePrimFuncVal {
    FreePrimFn { id: "integer.multiply", f: free_impl(fn_multiply), mode: FuncMode::default_mode() }
        .free()
}

fn fn_multiply(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.multiply(i2).into())
}

pub fn divide() -> FreePrimFuncVal {
    FreePrimFn { id: "integer.divide", f: free_impl(fn_divide), mode: FuncMode::default_mode() }
        .free()
}

fn fn_divide(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.divide(i2) else {
        return Val::default();
    };
    Val::Int(i.into())
}

pub fn remainder() -> FreePrimFuncVal {
    FreePrimFn {
        id: "integer.remainder",
        f: free_impl(fn_remainder),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_remainder(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.remainder(i2) else {
        return Val::default();
    };
    Val::Int(i.into())
}

pub fn divide_remainder() -> FreePrimFuncVal {
    FreePrimFn {
        id: "integer.divide_remainder",
        f: free_impl(fn_divide_remainder),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_divide_remainder(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return Val::default();
    };
    Val::Pair(Pair::new(Val::Int(quotient.into()), Val::Int(rem.into())).into())
}

pub fn less_than() -> FreePrimFuncVal {
    FreePrimFn { id: "integer.less", f: free_impl(fn_less_than), mode: FuncMode::default_mode() }
        .free()
}

fn fn_less_than(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    Val::Bit(i1.less_than(&i2))
}

pub fn less_equal() -> FreePrimFuncVal {
    FreePrimFn {
        id: "integer.less_equal",
        f: free_impl(fn_less_equal),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_less_equal(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    Val::Bit(i1.less_equal(&i2))
}

pub fn greater_than() -> FreePrimFuncVal {
    FreePrimFn {
        id: "integer.greater",
        f: free_impl(fn_greater_than),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_greater_than(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    Val::Bit(i1.greater_than(&i2))
}

pub fn greater_equal() -> FreePrimFuncVal {
    FreePrimFn {
        id: "integer.greater_equal",
        f: free_impl(fn_greater_equal),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_greater_equal(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    Val::Bit(i1.greater_equal(&i2))
}

pub fn less_greater() -> FreePrimFuncVal {
    FreePrimFn {
        id: "integer.less_greater",
        f: free_impl(fn_less_greater),
        mode: FuncMode::default_mode(),
    }
    .free()
}

fn fn_less_greater(_cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        error!("input.first {:?} should be a int", pair.first);
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        error!("input.second {:?} should be a int", pair.second);
        return Val::default();
    };
    Val::Bit(i1.less_greater(&i2))
}
