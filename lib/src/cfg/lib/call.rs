use std::mem::swap;

use const_format::concatcp;

use super::ConstImpl;
use super::FreeImpl;
use super::ImplExtra;
use super::MutImpl;
use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::CALL;
use crate::semantics::val::CtxPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CallLib {
    pub new: FreePrimFuncVal,
    pub get_function: CtxPrimFuncVal,
    pub set_function: CtxPrimFuncVal,
    pub get_input: CtxPrimFuncVal,
    pub set_input: CtxPrimFuncVal,
}

pub const NEW: &str = concatcp!(PREFIX_ID, CALL, ".new");
pub const GET_FUNCTION: &str = concatcp!(PREFIX_ID, CALL, ".get_function");
pub const SET_FUNCTION: &str = concatcp!(PREFIX_ID, CALL, ".set_function");
pub const GET_INPUT: &str = concatcp!(PREFIX_ID, CALL, ".get_input");
pub const SET_INPUT: &str = concatcp!(PREFIX_ID, CALL, ".set_input");

impl Default for CallLib {
    fn default() -> Self {
        CallLib {
            new: new(),
            get_function: get_function(),
            set_function: set_function(),
            get_input: get_input(),
            set_input: set_input(),
        }
    }
}

impl CfgMod for CallLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, NEW, self.new);
        extend_func(cfg, GET_FUNCTION, self.get_function);
        extend_func(cfg, SET_FUNCTION, self.set_function);
        extend_func(cfg, GET_INPUT, self.get_input);
        extend_func(cfg, SET_INPUT, self.set_input);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreeImpl { fn_: fn_new }.build(ImplExtra { raw_input: false })
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{NEW}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.left, pair.right).into())
}

pub fn get_function() -> CtxPrimFuncVal {
    ConstImpl { fn_: fn_get_function }.build(ImplExtra { raw_input: false })
}

fn fn_get_function(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{GET_FUNCTION}: expected context to be a call, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_FUNCTION}: expected input to be a unit, but got {input}");
    }
    call.func.clone()
}

pub fn set_function() -> CtxPrimFuncVal {
    MutImpl { fn_: fn_set_function }.build(ImplExtra { raw_input: false })
}

fn fn_set_function(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{SET_FUNCTION}: expected context to be a call, but got {ctx}");
    };
    swap(&mut call.func, &mut input);
    input
}

pub fn get_input() -> CtxPrimFuncVal {
    ConstImpl { fn_: fn_get_input }.build(ImplExtra { raw_input: false })
}

fn fn_get_input(cfg: &mut Cfg, ctx: &Val, input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{GET_INPUT}: expected context to be a call, but got {ctx}");
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_INPUT}: expected input to be a unit, but got {input}");
    }
    call.input.clone()
}

pub fn set_input() -> CtxPrimFuncVal {
    MutImpl { fn_: fn_set_input }.build(ImplExtra { raw_input: false })
}

fn fn_set_input(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{SET_INPUT}: expected context to be a call, but got {ctx}");
    };
    swap(&mut call.input, &mut input);
    input
}
