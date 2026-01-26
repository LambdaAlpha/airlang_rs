use std::mem::swap;
use std::ops::Deref;

use const_format::concatcp;

use super::ConstImpl;
use super::MutImpl;
use super::abort_const;
use super::abort_free;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::PAIR;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct PairLib {
    pub get_left: ConstPrimFuncVal,
    pub set_left: MutPrimFuncVal,
    pub get_right: ConstPrimFuncVal,
    pub set_right: MutPrimFuncVal,
}

pub const GET_LEFT: &str = concatcp!(PREFIX_ID, PAIR, ".get_left");
pub const SET_LEFT: &str = concatcp!(PREFIX_ID, PAIR, ".set_left");
pub const GET_RIGHT: &str = concatcp!(PREFIX_ID, PAIR, ".get_right");
pub const SET_RIGHT: &str = concatcp!(PREFIX_ID, PAIR, ".set_right");

impl Default for PairLib {
    fn default() -> Self {
        PairLib {
            get_left: get_left(),
            set_left: set_left(),
            get_right: get_right(),
            set_right: set_right(),
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

pub fn get_left() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_LEFT), const_: fn_get_left }.build()
}

fn fn_get_left(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return bug!(cfg, "{GET_LEFT}: expected context to be a pair, but got {:?}", ctx.deref());
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_LEFT}: expected input to be a unit, but got {input:?}");
    }
    pair.left.clone()
}

pub fn set_left() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET_LEFT), const_: abort_const(SET_LEFT), mut_: fn_set_left }.build()
}

fn fn_set_left(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return bug!(cfg, "{SET_LEFT}: expected context to be a pair, but got {ctx:?}");
    };
    swap(&mut pair.left, &mut input);
    input
}

pub fn get_right() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_RIGHT), const_: fn_get_right }.build()
}

fn fn_get_right(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Pair(pair) = &*ctx else {
        return bug!(cfg, "{GET_RIGHT}: expected context to be a pair, but got {:?}", ctx.deref());
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_RIGHT}: expected input to be a unit, but got {input:?}");
    }
    pair.right.clone()
}

pub fn set_right() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET_RIGHT), const_: abort_const(SET_RIGHT), mut_: fn_set_right }
        .build()
}

fn fn_set_right(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Pair(pair) = ctx else {
        return bug!(cfg, "{SET_RIGHT}: expected context to be a pair, but got {ctx:?}");
    };
    swap(&mut pair.right, &mut input);
    input
}
