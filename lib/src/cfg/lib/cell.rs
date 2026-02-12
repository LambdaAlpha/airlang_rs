use std::mem::swap;

use const_format::concatcp;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxConstInputFreeFunc;
use crate::semantics::func::CtxMutInputEvalFunc;
use crate::semantics::val::CELL;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct CellLib {
    pub get_value: PrimFuncVal,
    pub set_value: PrimFuncVal,
}

pub const GET_VALUE: &str = concatcp!(PREFIX_ID, CELL, ".get_value");
pub const SET_VALUE: &str = concatcp!(PREFIX_ID, CELL, ".set_value");

impl Default for CellLib {
    fn default() -> Self {
        CellLib { get_value: get_value(), set_value: set_value() }
    }
}

impl CfgMod for CellLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, GET_VALUE, self.get_value);
        extend_func(cfg, SET_VALUE, self.set_value);
    }
}

pub fn get_value() -> PrimFuncVal {
    CtxConstInputFreeFunc { fn_: fn_get_value }.build()
}

fn fn_get_value(cfg: &mut Cfg, ctx: &Val) -> Val {
    let Val::Cell(cell) = ctx else {
        return bug!(cfg, "{GET_VALUE}: expected context to be a cell, but got {ctx}");
    };
    cell.value.clone()
}

pub fn set_value() -> PrimFuncVal {
    CtxMutInputEvalFunc { fn_: fn_set_value }.build()
}

fn fn_set_value(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Cell(cell) = ctx else {
        return bug!(cfg, "{SET_VALUE}: expected context to be a cell, but got {ctx}");
    };
    swap(&mut cell.value, &mut input);
    input
}
