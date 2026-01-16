use log::error;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::cfg::CfgMod;
use crate::cfg::error::illegal_input;
use crate::cfg::extend_func;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::dyn_impl;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
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
        extend_func(cfg, "_resource.get_steps", self.get_steps);
        extend_func(cfg, "_resource.set_steps", self.set_steps);
        extend_func(cfg, "_resource.measure_steps", self.measure_steps);
    }
}

pub fn get_steps() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_get_steps) }.free()
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
    FreePrimFn { raw_input: false, f: free_impl(fn_set_steps) }.free()
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
    DynPrimFn { raw_input: true, f: dyn_impl(fn_measure_steps) }.mut_()
}

fn fn_measure_steps(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let old_steps = cfg.steps();
    let output = Eval.dyn_call(cfg, ctx, input);
    let steps = old_steps - cfg.steps();
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}
