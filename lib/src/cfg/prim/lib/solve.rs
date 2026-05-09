use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::ConstInputFreeFunc;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::MutFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::SOLVE;
use crate::semantics::val::Val;
use crate::type_::Pair;
use crate::type_::Solve;

#[derive(Copy, Clone)]
pub struct SolveLib {
    pub make: PrimFuncVal,
    pub get_function: PrimFuncVal,
    pub set_function: PrimFuncVal,
    pub get_output: PrimFuncVal,
    pub set_output: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, SOLVE, ".make");
pub const GET_FUNCTION: &str = concatcp!(PREFIX_CELL, SOLVE, ".get_function");
pub const SET_FUNCTION: &str = concatcp!(PREFIX_CELL, SOLVE, ".set_function");
pub const GET_OUTPUT: &str = concatcp!(PREFIX_CELL, SOLVE, ".get_output");
pub const SET_OUTPUT: &str = concatcp!(PREFIX_CELL, SOLVE, ".set_output");

impl Default for SolveLib {
    fn default() -> Self {
        Self {
            make: CtxFreeFunc { fn_: make }.build(),
            get_function: ConstInputFreeFunc { fn_: get_function }.build(),
            set_function: MutFunc { fn_: set_function }.build(),
            get_output: ConstInputFreeFunc { fn_: get_output }.build(),
            set_output: MutFunc { fn_: set_output }.build(),
        }
    }
}

impl CfgMod for SolveLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, MAKE, self.make);
        extend_func(cfg, GET_FUNCTION, self.get_function);
        extend_func(cfg, SET_FUNCTION, self.set_function);
        extend_func(cfg, GET_OUTPUT, self.get_output);
        extend_func(cfg, SET_OUTPUT, self.set_output);
    }
}

pub fn make(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{MAKE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    Val::Solve(Solve::new(pair.left, pair.right).into())
}

pub fn get_function(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Solve(solve) = ctx else {
        return bug!(cfg, "{GET_FUNCTION}: expected context to be a solve, but got {ctx}");
    };
    solve.func.clone()
}

pub fn set_function(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Solve(solve) = ctx else {
        return bug!(cfg, "{SET_FUNCTION}: expected context to be a solve, but got {ctx}");
    };
    swap(&mut solve.func, &mut input);
    input
}

pub fn get_output(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Solve(solve) = ctx else {
        return bug!(cfg, "{GET_OUTPUT}: expected context to be a solve, but got {ctx}");
    };
    solve.output.clone()
}

pub fn set_output(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Solve(solve) = ctx else {
        return bug!(cfg, "{SET_OUTPUT}: expected context to be a solve, but got {ctx}");
    };
    swap(&mut solve.output, &mut input);
    input
}
