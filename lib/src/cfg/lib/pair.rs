use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxMutInputEvalFunc;
use crate::semantics::val::PAIR;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct PairLib {
    pub get_left: PrimFuncVal,
    pub set_left: PrimFuncVal,
    pub get_right: PrimFuncVal,
    pub set_right: PrimFuncVal,
}

pub const GET_LEFT: &str = concatcp!(PREFIX_ID, PAIR, ".get_left");
pub const SET_LEFT: &str = concatcp!(PREFIX_ID, PAIR, ".set_left");
pub const GET_RIGHT: &str = concatcp!(PREFIX_ID, PAIR, ".get_right");
pub const SET_RIGHT: &str = concatcp!(PREFIX_ID, PAIR, ".set_right");

impl Default for PairLib {
    fn default() -> Self {
        PairLib {
            get_left: CtxConstInputFreeFunc { fn_: get_left }.build(),
            set_left: CtxMutInputEvalFunc { fn_: set_left }.build(),
            get_right: CtxConstInputFreeFunc { fn_: get_right }.build(),
            set_right: CtxMutInputEvalFunc { fn_: set_right }.build(),
        }
    }
}

impl CfgMod for PairLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, GET_LEFT, self.get_left);
        extend_func(cfg, SET_LEFT, self.set_left);
        extend_func(cfg, GET_RIGHT, self.get_right);
        extend_func(cfg, SET_RIGHT, self.set_right);
    }
}

pub fn get_left(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return bug!(cfg, "{GET_LEFT}: expected context to be a pair, but got {ctx}");
    };
    pair.left.clone()
}

pub fn set_left(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return bug!(cfg, "{SET_LEFT}: expected context to be a pair, but got {ctx}");
    };
    swap(&mut pair.left, &mut input);
    input
}

pub fn get_right(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return bug!(cfg, "{GET_RIGHT}: expected context to be a pair, but got {ctx}");
    };
    pair.right.clone()
}

pub fn set_right(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return bug!(cfg, "{SET_RIGHT}: expected context to be a pair, but got {ctx}");
    };
    swap(&mut pair.right, &mut input);
    input
}
