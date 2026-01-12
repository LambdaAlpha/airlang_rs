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
    pub get_available_steps: FreePrimFuncVal,
    pub measure_steps: MutPrimFuncVal,
    pub limit_steps: MutPrimFuncVal,
}

impl Default for ResourceLib {
    fn default() -> Self {
        ResourceLib {
            get_available_steps: get_available_steps(),
            measure_steps: measure_steps(),
            limit_steps: limit_steps(),
        }
    }
}

impl CfgMod for ResourceLib {
    fn extend(self, cfg: &Cfg) {
        extend_func(cfg, "_resource.get_available_steps", self.get_available_steps);
        extend_func(cfg, "_resource.measure_steps", self.measure_steps);
        extend_func(cfg, "_resource.limit_steps", self.limit_steps);
    }
}

pub fn get_available_steps() -> FreePrimFuncVal {
    FreePrimFn { raw_input: false, f: free_impl(fn_get_available_steps) }.free()
}

fn fn_get_available_steps(cfg: &mut Cfg, input: Val) -> Val {
    if !input.is_unit() {
        error!("input {input:?} should be a unit");
        return illegal_input(cfg);
    }
    let steps = cfg.steps();
    Val::Int(Int::from(steps).into())
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

pub fn limit_steps() -> MutPrimFuncVal {
    DynPrimFn { raw_input: false, f: dyn_impl(fn_limit_steps) }.mut_()
}

fn fn_limit_steps(cfg: &mut Cfg, ctx: DynRef<Val>, input: Val) -> Val {
    let (steps, input) = match steps_input(cfg, input) {
        Ok((steps, input)) => (steps, input),
        Err(val) => return val,
    };
    let cur_steps = cfg.steps();
    if steps > cur_steps {
        return Eval.dyn_call(cfg, ctx, input);
    }
    cfg.set_steps_unchecked(steps);
    let output = Eval.dyn_call(cfg, ctx, input);
    cfg.set_steps_unchecked(cur_steps - (steps - cfg.steps()));
    if &*cfg.abort_reason() == Cfg::ABORT_REASON_STEPS_EXCEED {
        cfg.resume();
    }
    output
}

fn steps_input(cfg: &mut Cfg, input: Val) -> Result<(u128, Val), Val> {
    let Val::Pair(pair) = input else {
        error!("input {input:?} should be a pair");
        return Err(illegal_input(cfg));
    };
    let pair = Pair::from(pair);
    let Val::Int(steps) = pair.first else {
        error!("input.first {:?} should be an integer", pair.first);
        return Err(illegal_input(cfg));
    };
    if steps.is_negative() {
        error!("input.first {steps:?} should be a positive integer");
        return Err(illegal_input(cfg));
    }
    let steps = steps.to_u128().unwrap_or(u128::MAX);
    Ok((steps, pair.second))
}
