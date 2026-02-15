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
            add: CtxFreeInputEvalFunc { fn_: add }.build(),
            subtract: CtxFreeInputEvalFunc { fn_: subtract }.build(),
            multiply: CtxFreeInputEvalFunc { fn_: multiply }.build(),
            divide: CtxFreeInputEvalFunc { fn_: divide }.build(),
            remainder: CtxFreeInputEvalFunc { fn_: remainder }.build(),
            divide_remainder: CtxFreeInputEvalFunc { fn_: divide_remainder }.build(),
            less: CtxFreeInputEvalFunc { fn_: less }.build(),
            less_equal: CtxFreeInputEvalFunc { fn_: less_equal }.build(),
            greater: CtxFreeInputEvalFunc { fn_: greater }.build(),
            greater_equal: CtxFreeInputEvalFunc { fn_: greater_equal }.build(),
            less_greater: CtxFreeInputEvalFunc { fn_: less_greater }.build(),
        }
    }
}

impl CfgMod for IntLib {
    fn extend(self, cfg: &mut Cfg) {
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

pub fn add(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, ADD, input) else {
        return Val::default();
    };
    Val::Int(i1.add(i2).into())
}

pub fn subtract(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, SUBTRACT, input) else {
        return Val::default();
    };
    Val::Int(i1.subtract(i2).into())
}

pub fn multiply(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, MULTIPLY, input) else {
        return Val::default();
    };
    Val::Int(i1.multiply(i2).into())
}

pub fn divide(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, DIVIDE, input) else {
        return Val::default();
    };
    if i2.is_zero() {
        return bug!(cfg, "{DIVIDE}: expected input.right to be non-zero");
    }
    let i = i1.divide(i2);
    Val::Int(i.into())
}

pub fn remainder(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, REMAINDER, input) else {
        return Val::default();
    };
    if i2.is_zero() {
        return bug!(cfg, "{REMAINDER}: expected input.right to be non-zero");
    }
    let i = i1.remainder(i2);
    Val::Int(i.into())
}

pub fn divide_remainder(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, DIVIDE_REMAINDER, input) else {
        return Val::default();
    };
    if i2.is_zero() {
        return bug!(cfg, "{DIVIDE_REMAINDER}: expected input.right to be non-zero");
    }
    let (quotient, rem) = i1.divide_remainder(i2);
    Val::Pair(Pair::new(Val::Int(quotient.into()), Val::Int(rem.into())).into())
}

pub fn less(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, LESS, input) else {
        return Val::default();
    };
    Val::Bit(i1.less(&i2))
}

pub fn less_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, LESS_EQUAL, input) else {
        return Val::default();
    };
    Val::Bit(i1.less_equal(&i2))
}

pub fn greater(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, GREATER, input) else {
        return Val::default();
    };
    Val::Bit(i1.greater(&i2))
}

pub fn greater_equal(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, GREATER_EQUAL, input) else {
        return Val::default();
    };
    Val::Bit(i1.greater_equal(&i2))
}

pub fn less_greater(cfg: &mut Cfg, input: Val) -> Val {
    let Some((i1, i2)) = int_pair(cfg, LESS_GREATER, input) else {
        return Val::default();
    };
    Val::Bit(i1.less_greater(&i2))
}

fn int_pair(cfg: &mut Cfg, tag: &str, input: Val) -> Option<(Int, Int)> {
    let Val::Pair(pair) = input else {
        bug!(cfg, "{tag}: expected input to be a pair, but got {input}");
        return None;
    };
    let pair = Pair::from(pair);
    let Val::Int(i1) = pair.left else {
        bug!(cfg, "{tag}: expected input.left to be an integer, but got {}", pair.left);
        return None;
    };
    let Val::Int(i2) = pair.right else {
        bug!(cfg, "{tag}: expected input.right to be an integer, but got {}", pair.right);
        return None;
    };
    Some((i1.into(), i2.into()))
}
