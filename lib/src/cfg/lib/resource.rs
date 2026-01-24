use const_format::concatcp;
use log::error;
use num_traits::Signed;
use num_traits::ToPrimitive;

use super::DynImpl;
use super::FreeImpl;
use super::abort_free;
use crate::cfg::CfgMod;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::DynRef;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct ResourceLib {
    pub get_steps: FreePrimFuncVal,
    pub set_steps: FreePrimFuncVal,
    pub measure_steps: MutPrimFuncVal,
}

const RESOURCE: &str = "resource";

pub const GET_STEPS: &str = concatcp!(PREFIX_ID, RESOURCE, ".get_steps");
pub const SET_STEPS: &str = concatcp!(PREFIX_ID, RESOURCE, ".set_steps");
pub const MEASURE_STEPS: &str = concatcp!(PREFIX_ID, RESOURCE, ".measure_steps");

impl Default for ResourceLib {
    fn default() -> Self {
        ResourceLib {
            get_steps: get_steps(),
            set_steps: set_steps(),
            measure_steps: measure_steps(),
        }
    }
}

impl CfgMod for ResourceLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, GET_STEPS, self.get_steps);
        extend_func(cfg, SET_STEPS, self.set_steps);
        extend_func(cfg, MEASURE_STEPS, self.measure_steps);
    }
}

pub fn get_steps() -> FreePrimFuncVal {
    FreeImpl { free: fn_get_steps }.build()
}

fn fn_get_steps(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let steps = cfg.steps();
    Val::Int(Int::from(steps).into())
}

pub fn set_steps() -> FreePrimFuncVal {
    FreeImpl { free: fn_set_steps }.build()
}

fn fn_set_steps(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Int(steps) = input else {
        error!("input {input:?} should be an integer");
        return illegal_input(cfg);
    };
    if steps.is_negative() {
        error!("input {steps:?} should be a natural integer");
        return illegal_input(cfg);
    }
    let steps = steps.to_u128().unwrap_or(u128::MAX);
    cfg.set_steps(steps);
    Val::default()
}

pub fn measure_steps() -> MutPrimFuncVal {
    DynImpl { free: abort_free(MEASURE_STEPS), dyn_: fn_measure_steps }.build_with(true)
}

fn fn_measure_steps(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let old_steps = cfg.steps();
    let output = Eval.dyn_call(cfg, ctx, input);
    let steps = old_steps - cfg.steps();
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}
