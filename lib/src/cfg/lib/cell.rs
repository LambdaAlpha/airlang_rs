use std::mem::swap;

use log::error;

use crate::cfg::CfgMod;
use crate::cfg::exception::illegal_ctx;
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
    pub value: ConstPrimFuncVal,
    pub set_value: MutPrimFuncVal,
}

impl Default for CellLib {
    fn default() -> Self {
        CellLib { value: value(), set_value: set_value() }
    }
}

impl CfgMod for CellLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_cell.value", self.value);
        extend_func(cfg, "_cell.set_value", self.set_value);
    }
}

pub fn value() -> ConstPrimFuncVal {
    DynPrimFn { raw_input: false, f: const_impl(fn_value) }.const_()
}

fn fn_value(cfg: &mut Cfg, ctx: ConstRef<Val>, _input: Val) -> Val {
    let Val::Cell(cell) = &*ctx else {
        error!("ctx {ctx:?} should be a cell");
        return illegal_ctx(cfg);
    };
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
