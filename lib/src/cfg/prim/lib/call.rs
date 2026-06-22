use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::export_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_CELL;
use crate::semantics::func::ConstInputFreeFunc;
use crate::semantics::func::CtxFreeFunc;
use crate::semantics::func::MutFunc;
use crate::semantics::val::CALL;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Call;
use crate::type_::Key;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Copy, Clone)]
pub struct CallLib {
    pub make: PrimFuncVal,
    pub get_function: PrimFuncVal,
    pub set_function: PrimFuncVal,
    pub get_input: PrimFuncVal,
    pub set_input: PrimFuncVal,
}

pub const MAKE: &str = concatcp!(PREFIX_CELL, CALL, ".make");
pub const GET_FUNCTION: &str = concatcp!(PREFIX_CELL, CALL, ".get_function");
pub const SET_FUNCTION: &str = concatcp!(PREFIX_CELL, CALL, ".set_function");
pub const GET_INPUT: &str = concatcp!(PREFIX_CELL, CALL, ".get_input");
pub const SET_INPUT: &str = concatcp!(PREFIX_CELL, CALL, ".set_input");

impl Default for CallLib {
    fn default() -> Self {
        Self {
            make: CtxFreeFunc { fn_: make }.build(),
            get_function: ConstInputFreeFunc { fn_: get_function }.build(),
            set_function: MutFunc { fn_: set_function }.build(),
            get_input: ConstInputFreeFunc { fn_: get_input }.build(),
            set_input: MutFunc { fn_: set_input }.build(),
        }
    }
}

impl CfgMod for CallLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        export_func(cfg, MAKE, self.make);
        export_func(cfg, GET_FUNCTION, self.get_function);
        export_func(cfg, SET_FUNCTION, self.set_function);
        export_func(cfg, GET_INPUT, self.get_input);
        export_func(cfg, SET_INPUT, self.set_input);
    }
}

pub fn make(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return bug!(cfg, "{MAKE}: expected input to be a pair, but got {input}");
    };
    let pair = Pair::from(pair);
    Val::Call(Call::new(pair.left, pair.right).into())
}

pub fn get_function(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{GET_FUNCTION}: expected context to be a call, but got {ctx}");
    };
    call.func.clone()
}

pub fn set_function(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{SET_FUNCTION}: expected context to be a call, but got {ctx}");
    };
    swap(&mut call.func, &mut input);
    input
}

pub fn get_input(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{GET_INPUT}: expected context to be a call, but got {ctx}");
    };
    call.input.clone()
}

pub fn set_input(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Call(call) = ctx else {
        return bug!(cfg, "{SET_INPUT}: expected context to be a call, but got {ctx}");
    };
    swap(&mut call.input, &mut input);
    input
}
