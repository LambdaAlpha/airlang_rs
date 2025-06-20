use super::FreeFn;
use super::FuncMode;
use super::Prelude;
use super::PreludeCtx;
use super::free_impl;
use crate::semantics::val::FreeStaticPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct IntPrelude {
    pub add: FreeStaticPrimFuncVal,
    pub subtract: FreeStaticPrimFuncVal,
    pub multiply: FreeStaticPrimFuncVal,
    pub divide: FreeStaticPrimFuncVal,
    pub remainder: FreeStaticPrimFuncVal,
    pub divide_remainder: FreeStaticPrimFuncVal,
    pub less_than: FreeStaticPrimFuncVal,
    pub less_equal: FreeStaticPrimFuncVal,
    pub greater_than: FreeStaticPrimFuncVal,
    pub greater_equal: FreeStaticPrimFuncVal,
    pub less_greater: FreeStaticPrimFuncVal,
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
    fn put(&self, ctx: &mut dyn PreludeCtx) {
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

pub fn add() -> FreeStaticPrimFuncVal {
    FreeFn { id: "+", f: free_impl(fn_add), mode: FuncMode::default() }.free_static()
}

fn fn_add(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.add(i2).into())
}

pub fn subtract() -> FreeStaticPrimFuncVal {
    FreeFn { id: "-", f: free_impl(fn_subtract), mode: FuncMode::default() }.free_static()
}

fn fn_subtract(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.subtract(i2).into())
}

pub fn multiply() -> FreeStaticPrimFuncVal {
    FreeFn { id: "*", f: free_impl(fn_multiply), mode: FuncMode::default() }.free_static()
}

fn fn_multiply(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.multiply(i2).into())
}

pub fn divide() -> FreeStaticPrimFuncVal {
    FreeFn { id: "/", f: free_impl(fn_divide), mode: FuncMode::default() }.free_static()
}

fn fn_divide(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.divide(i2) else {
        return Val::default();
    };
    Val::Int(i.into())
}

pub fn remainder() -> FreeStaticPrimFuncVal {
    FreeFn { id: "%", f: free_impl(fn_remainder), mode: FuncMode::default() }.free_static()
}

fn fn_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some(i) = i1.remainder(i2) else {
        return Val::default();
    };
    Val::Int(i.into())
}

pub fn divide_remainder() -> FreeStaticPrimFuncVal {
    FreeFn { id: "/%", f: free_impl(fn_divide_remainder), mode: FuncMode::default() }.free_static()
}

fn fn_divide_remainder(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let Some((quotient, rem)) = i1.divide_remainder(i2) else {
        return Val::default();
    };
    Val::Pair(Pair::new(Val::Int(quotient.into()), Val::Int(rem.into())).into())
}

pub fn less_than() -> FreeStaticPrimFuncVal {
    FreeFn { id: "<", f: free_impl(fn_less_than), mode: FuncMode::default() }.free_static()
}

fn fn_less_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.less_than(&i2))
}

pub fn less_equal() -> FreeStaticPrimFuncVal {
    FreeFn { id: "<=", f: free_impl(fn_less_equal), mode: FuncMode::default() }.free_static()
}

fn fn_less_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.less_equal(&i2))
}

pub fn greater_than() -> FreeStaticPrimFuncVal {
    FreeFn { id: ">", f: free_impl(fn_greater_than), mode: FuncMode::default() }.free_static()
}

fn fn_greater_than(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.greater_than(&i2))
}

pub fn greater_equal() -> FreeStaticPrimFuncVal {
    FreeFn { id: ">=", f: free_impl(fn_greater_equal), mode: FuncMode::default() }.free_static()
}

fn fn_greater_equal(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.greater_equal(&i2))
}

pub fn less_greater() -> FreeStaticPrimFuncVal {
    FreeFn { id: "<>", f: free_impl(fn_less_greater), mode: FuncMode::default() }.free_static()
}

fn fn_less_greater(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.first else {
        return Val::default();
    };
    let Val::Int(i2) = pair.second else {
        return Val::default();
    };
    Val::Bit(i1.less_greater(&i2))
}
