use const_format::concatcp;
use num_traits::Zero;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::val::INT;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct IntLib {
    pub add: PrimFuncVal,
    pub subtract: PrimFuncVal,
    pub multiply: PrimFuncVal,
    pub divide: PrimFuncVal,
    pub remainder: PrimFuncVal,
    pub divide_remainder: PrimFuncVal,
    pub less: PrimFuncVal,
    pub less_equal: PrimFuncVal,
    pub greater: PrimFuncVal,
    pub greater_equal: PrimFuncVal,
    pub less_greater: PrimFuncVal,
}

pub const ADD: &str = concatcp!(PREFIX_ID, INT, ".add");
pub const SUBTRACT: &str = concatcp!(PREFIX_ID, INT, ".subtract");
pub const MULTIPLY: &str = concatcp!(PREFIX_ID, INT, ".multiply");
pub const DIVIDE: &str = concatcp!(PREFIX_ID, INT, ".divide");
pub const REMAINDER: &str = concatcp!(PREFIX_ID, INT, ".remainder");
pub const DIVIDE_REMAINDER: &str = concatcp!(PREFIX_ID, INT, ".divide_remainder");
pub const LESS: &str = concatcp!(PREFIX_ID, INT, ".less");
pub const LESS_EQUAL: &str = concatcp!(PREFIX_ID, INT, ".less_equal");
pub const GREATER: &str = concatcp!(PREFIX_ID, INT, ".greater");
pub const GREATER_EQUAL: &str = concatcp!(PREFIX_ID, INT, ".greater_equal");
pub const LESS_GREATER: &str = concatcp!(PREFIX_ID, INT, ".less_greater");

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
        extend_func(cfg, ADD, self.add);
        extend_func(cfg, SUBTRACT, self.subtract);
        extend_func(cfg, MULTIPLY, self.multiply);
        extend_func(cfg, DIVIDE, self.divide);
        extend_func(cfg, REMAINDER, self.remainder);
        extend_func(cfg, DIVIDE_REMAINDER, self.divide_remainder);
        extend_func(cfg, LESS, self.less);
        extend_func(cfg, LESS_EQUAL, self.less_equal);
        extend_func(cfg, GREATER, self.greater);
        extend_func(cfg, GREATER_EQUAL, self.greater_equal);
        extend_func(cfg, LESS_GREATER, self.less_greater);
    }
}

pub fn add() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_add }.build()
}

fn fn_add(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{ADD}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{ADD}: expected input.left to be an integer, but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{ADD}: expected input.right to be an integer, but got {}", pair.right);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.add(i2).into())
}

pub fn subtract() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_subtract }.build()
}

fn fn_subtract(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{SUBTRACT}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{SUBTRACT}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{SUBTRACT}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.subtract(i2).into())
}

pub fn multiply() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_multiply }.build()
}

fn fn_multiply(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{MULTIPLY}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{MULTIPLY}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{MULTIPLY}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    Val::Int(i1.multiply(i2).into())
}

pub fn divide() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_divide }.build()
}

fn fn_divide(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{DIVIDE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{DIVIDE}: expected input.left to be an integer, but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{DIVIDE}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    if i2.is_zero() {
        return bug!(cfg, "{DIVIDE}: expected input.right to be non-zero");
    }
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let i = i1.divide(i2);
    Val::Int(i.into())
}

pub fn remainder() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_remainder }.build()
}

fn fn_remainder(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{REMAINDER}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{REMAINDER}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{REMAINDER}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    if i2.is_zero() {
        return bug!(cfg, "{REMAINDER}: expected input.right to be non-zero");
    }
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let i = i1.remainder(i2);
    Val::Int(i.into())
}

pub fn divide_remainder() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_divide_remainder }.build()
}

fn fn_divide_remainder(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{DIVIDE_REMAINDER}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{DIVIDE_REMAINDER}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{DIVIDE_REMAINDER}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    if i2.is_zero() {
        return bug!(cfg, "{DIVIDE_REMAINDER}: expected input.right to be non-zero");
    }
    let i1 = Int::from(i1);
    let i2 = Int::from(i2);
    let (quotient, rem) = i1.divide_remainder(i2);
    Val::Pair(Pair::new(Val::Int(quotient.into()), Val::Int(rem.into())).into())
}

pub fn less() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_less }.build()
}

fn fn_less(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LESS}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{LESS}: expected input.left to be an integer, but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{LESS}: expected input.right to be an integer, but got {}", pair.right);
    };
    Val::Bit(i1.less_than(&i2))
}

pub fn less_equal() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_less_equal }.build()
}

fn fn_less_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LESS_EQUAL}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{LESS_EQUAL}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{LESS_EQUAL}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    Val::Bit(i1.less_equal(&i2))
}

pub fn greater() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_greater }.build()
}

fn fn_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{GREATER}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{GREATER}: expected input.left to be an integer, but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{GREATER}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    Val::Bit(i1.greater_than(&i2))
}

pub fn greater_equal() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_greater_equal }.build()
}

fn fn_greater_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{GREATER_EQUAL}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{GREATER_EQUAL}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{GREATER_EQUAL}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    Val::Bit(i1.greater_equal(&i2))
}

pub fn less_greater() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_less_greater }.build()
}

fn fn_less_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{LESS_GREATER}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        return bug!(cfg, "{LESS_GREATER}: expected input.left to be an integer, \
            but got {}", pair.left);
    };
    let Val::Int(i2) = pair.right else {
        return bug!(cfg, "{LESS_GREATER}: expected input.right to be an integer, \
            but got {}", pair.right);
    };
    Val::Bit(i1.less_greater(&i2))
}
