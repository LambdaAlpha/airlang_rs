use log::error;

use super::FreePrimFn;
use super::Prelude;
use super::free_impl;
use crate::cfg::prelude::setup::default_free_mode;
use crate::semantics::cfg::Cfg;
use crate::semantics::ctx::Ctx;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct IntPrelude {
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

impl Default for IntPrelude {
    fn default() -> Self {
        IntPrelude {
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

impl Prelude for IntPrelude {
    fn put(self, ctx: &mut Ctx) {
        self.add.put(ctx);
        self.subtract.put(ctx);
        self.multiply.put(ctx);
        self.divide.put(ctx);
        self.remainder.put(ctx);
        self.divide_remainder.put(ctx);
        self.less_than.put(ctx);
        self.less_equal.put(ctx);
        self.greater_than.put(ctx);
        self.greater_equal.put(ctx);
        self.less_greater.put(ctx);
    }
}

pub fn add() -> FreePrimFuncVal {
    FreePrimFn { id: "+", f: free_impl(fn_add), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "-", f: free_impl(fn_subtract), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "*", f: free_impl(fn_multiply), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "/", f: free_impl(fn_divide), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "%", f: free_impl(fn_remainder), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "/%", f: free_impl(fn_divide_remainder), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "<", f: free_impl(fn_less_than), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "<=", f: free_impl(fn_less_equal), mode: default_free_mode() }.free()
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
    FreePrimFn { id: ">", f: free_impl(fn_greater_than), mode: default_free_mode() }.free()
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
    FreePrimFn { id: ">=", f: free_impl(fn_greater_equal), mode: default_free_mode() }.free()
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
    FreePrimFn { id: "<>", f: free_impl(fn_less_greater), mode: default_free_mode() }.free()
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
