use std::mem::swap;

use log::error;

use crate::cfg::CfgMod;
use crate::cfg::error::illegal_ctx;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::const_impl;
use crate::cfg::lib::mut_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::val::ConstPrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;

#[derive(Clone)]
pub struct CellLib {
    pub get_value: ConstPrimFuncVal,
    pub set_value: MutPrimFuncVal,
}

impl Default for CellLib {
    fn default() -> Self {
        CellLib { get_value: get_value(), set_value: set_value() }
    }
}

impl CfgMod for CellLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_cell.get_value", self.get_value);
        extend_func(cfg, "_cell.set_value", self.set_value);
    }
}

pub fn get_value() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_get_value) }.const_()
}

fn fn_get_value(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let Val::Cell(cell) = &*ctx else {
        error!("ctx {ctx:?} should be a cell");
        return illegal_ctx(cfg);
    };
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    cell.value.clone()
}

pub fn set_value() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: mut_impl(fn_set_value) }.mut_()
}

fn fn_set_value(cfg: &mut Cfg, ctx: &mut Val, mut input: Val) -> Val {
    let Val::Cell(cell) = ctx else {
        error!("ctx {ctx:?} should be a cell");
        return illegal_ctx(cfg);
    };
    swap(&mut cell.value, &mut input);
    input
}
