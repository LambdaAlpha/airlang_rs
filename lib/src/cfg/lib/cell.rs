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
use crate::semantics::val::CELL;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct CellLib {
    pub get_value: ConstPrimFuncVal,
    pub set_value: MutPrimFuncVal,
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

pub fn get_value() -> ConstPrimFuncVal {
    ConstImpl { free: abort_free(GET_VALUE), const_: fn_get_value }.build()
}

fn fn_get_value(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Cell(cell) = &*ctx else {
        return bug!(cfg, "{GET_VALUE}: expected context to be a cell, but got {:?}", ctx.deref());
    };
    if !input.is_unit() {
        return bug!(cfg, "{GET_VALUE}: expected input to be a unit, but got {input:?}");
    }
    cell.value.clone()
}

pub fn set_value() -> MutPrimFuncVal {
    MutImpl { free: abort_free(SET_VALUE), const_: abort_const(SET_VALUE), mut_: fn_set_value }
        .build()
}

fn fn_set_value(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Cell(cell) = ctx else {
        return bug!(cfg, "{SET_VALUE}: expected context to be a cell, but got {ctx:?}");
    };
    swap(&mut cell.value, &mut input);
    input
}
