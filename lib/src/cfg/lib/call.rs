use std::mem::swap;

use const_format::concatcp;
use log::error;

use super::ConstImpl;
use super::FreeImpl;
use super::MutImpl;
use super::abort_const;
use super::abort_free;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::val::CALL;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::ConstRef;
use crate::type_::Pair;

#[derive(Clone)]
pub struct CallLib {
    pub new: FreePrimFuncVal,
    pub get_function: ConstPrimFuncVal,
    pub set_function: MutPrimFuncVal,
    pub get_input: ConstPrimFuncVal,
    pub set_input: MutPrimFuncVal,
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
    FreeImpl { free: fn_new }.build()
}

fn fn_new(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return illegal_input(cfg);
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.left, pair.right).into())
}

pub fn get_function() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_FUNCTION), const_: fn_get_function }.build()
}

fn fn_get_function(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    call.func.clone()
}

pub fn set_function() -> MutPrimFuncVal {
    MutImpl {
        free: abort_free(SET_FUNCTION),
        const_: abort_const(SET_FUNCTION),
        mut_: fn_set_function,
    }
    .build()
}

fn fn_set_function(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    swap(&mut call.func, &mut input);
    input
}

pub fn get_input() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_INPUT), const_: fn_get_input }.build()
}

fn fn_get_input(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Call(call) = &*ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    call.input.clone()
}

pub fn set_input() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET_INPUT), const_: abort_const(SET_INPUT), mut_: fn_set_input }
        .build()
}

fn fn_set_input(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        error!("ctx {ctx:?} should be a call");
        return illegal_ctx(cfg);
    };
    swap(&mut call.input, &mut input);
    input
}
