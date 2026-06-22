use const_format::concatcp;
use num_traits::Zero;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::export_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::val::INT;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Copy, Clone)]
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

pub const ADD: &str = concatcp!(PREFIX_CELL, INT, ".add");
pub const SUBTRACT: &str = concatcp!(PREFIX_CELL, INT, ".subtract");
pub const MULTIPLY: &str = concatcp!(PREFIX_CELL, INT, ".multiply");
pub const DIVIDE: &str = concatcp!(PREFIX_CELL, INT, ".divide");
pub const REMAINDER: &str = concatcp!(PREFIX_CELL, INT, ".remainder");
pub const DIVIDE_REMAINDER: &str = concatcp!(PREFIX_CELL, INT, ".divide_remainder");
pub const LESS: &str = concatcp!(PREFIX_CELL, INT, ".less");
pub const LESS_EQUAL: &str = concatcp!(PREFIX_CELL, INT, ".less_equal");
pub const GREATER: &str = concatcp!(PREFIX_CELL, INT, ".greater");
pub const GREATER_EQUAL: &str = concatcp!(PREFIX_CELL, INT, ".greater_equal");
pub const LESS_GREATER: &str = concatcp!(PREFIX_CELL, INT, ".less_greater");

impl Default for IntLib {
    fn default() -> Self {
        Self {
            add: CtxFreeFunc { fn_: add }.build(),
            subtract: CtxFreeFunc { fn_: subtract }.build(),
            multiply: CtxFreeFunc { fn_: multiply }.build(),
            divide: CtxFreeFunc { fn_: divide }.build(),
            remainder: CtxFreeFunc { fn_: remainder }.build(),
            divide_remainder: CtxFreeFunc { fn_: divide_remainder }.build(),
            less: CtxFreeFunc { fn_: less }.build(),
            less_equal: CtxFreeFunc { fn_: less_equal }.build(),
            greater: CtxFreeFunc { fn_: greater }.build(),
            greater_equal: CtxFreeFunc { fn_: greater_equal }.build(),
            less_greater: CtxFreeFunc { fn_: less_greater }.build(),
        }
    }
}

impl CfgMod for IntLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, ADD, self.add);
        export_func(cfg, SUBTRACT, self.subtract);
        export_func(cfg, MULTIPLY, self.multiply);
        export_func(cfg, DIVIDE, self.divide);
        export_func(cfg, REMAINDER, self.remainder);
        export_func(cfg, DIVIDE_REMAINDER, self.divide_remainder);
        export_func(cfg, LESS, self.less);
        export_func(cfg, LESS_EQUAL, self.less_equal);
        export_func(cfg, GREATER, self.greater);
        export_func(cfg, GREATER_EQUAL, self.greater_equal);
        export_func(cfg, LESS_GREATER, self.less_greater);
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
