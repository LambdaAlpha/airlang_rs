use log::error;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::exception::illegal_input;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::MutImpl;
use crate::cfg::lib::adapter::default_adapter;
use crate::cfg::lib::adapter::id_adapter;
use crate::cfg::lib::adapter::pair_adapter;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::cfg::StepsExceed;
use crate::semantics::core::Eval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Map;
use crate::type_::Pair;

#[derive(Clone)]
pub struct ResourceLib {
    pub available_steps: FreePrimFuncVal,
    pub measure_steps: MutPrimFuncVal,
    pub limit_steps: MutPrimFuncVal,
}

impl Default for ResourceLib {
    fn default() -> Self {
        ResourceLib {
            available_steps: available_steps(),
            measure_steps: measure_steps(),
            limit_steps: limit_steps(),
        }
    }
}

impl CfgMod for ResourceLib {
    fn extend(self, cfg: &Cfg) {
        self.available_steps.extend(cfg);
        let measure_steps_adapter = id_adapter();
        CoreCfg::extend_adapter(cfg, &self.measure_steps.id, measure_steps_adapter);
        self.measure_steps.extend(cfg);
        let limit_steps_adapter = pair_adapter(Map::default(), default_adapter(), id_adapter());
        CoreCfg::extend_adapter(cfg, &self.limit_steps.id, limit_steps_adapter);
        self.limit_steps.extend(cfg);
    }
}

pub fn available_steps() -> FreePrimFuncVal {
    FreePrimFn { id: "resource.available_steps", f: free_impl(fn_available_steps) }.free()
}

fn fn_available_steps(cfg: &mut Cfg, _input: Val) -> Val {
    let steps = cfg.steps();
    Val::Int(Int::from(steps).into())
}

pub fn measure_steps() -> MutPrimFuncVal {
    DynPrimFn {
        id: "resource.measure_steps",
        f: MutImpl::new(fn_measure_steps_free, fn_measure_steps_const, fn_measure_steps_mut),
    }
    .mut_()
}

fn fn_measure_steps_free(cfg: &mut Cfg, input: Val) -> Val {
    let old_steps = cfg.steps();
    let output = Eval.free_call(cfg, input);
    let steps = cfg.steps() - old_steps;
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}

fn fn_measure_steps_const(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let old_steps = cfg.steps();
    let output = Eval.const_call(cfg, ctx, input);
    let steps = cfg.steps() - old_steps;
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}

fn fn_measure_steps_mut(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let old_steps = cfg.steps();
    let output = Eval.mut_call(cfg, ctx, input);
    let steps = old_steps - cfg.steps();
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}

pub fn limit_steps() -> MutPrimFuncVal {
    DynPrimFn {
        id: "resource.limit_steps",
        f: MutImpl::new(fn_limit_steps_free, fn_limit_steps_const, fn_limit_steps_mut),
    }
    .mut_()
}

fn fn_limit_steps_free(cfg: &mut Cfg, input: Val) -> Val {
    let (steps, input) = match steps_input(cfg, input) {
        Ok((steps, input)) => (steps, input),
        Err(val) => return val,
    };
    let cur_steps = cfg.steps();
    if steps > cur_steps {
        return Eval.free_call(cfg, input);
    }
    cfg.set_steps_unchecked(steps);
    let output = StepsExceed::catch(|| Eval.free_call(cfg, input));
    cfg.set_steps_unchecked(cur_steps - (steps - cfg.steps()));
    output.unwrap_or_default()
}

fn fn_limit_steps_const(cfg: &mut Cfg, ctx: ConstRef<Val>, input: Val) -> Val {
    let (steps, input) = match steps_input(cfg, input) {
        Ok((steps, input)) => (steps, input),
        Err(val) => return val,
    };
    let cur_steps = cfg.steps();
    if steps > cur_steps {
        return Eval.const_call(cfg, ctx, input);
    }
    cfg.set_steps_unchecked(steps);
    let output = StepsExceed::catch(|| Eval.const_call(cfg, ctx, input));
    cfg.set_steps_unchecked(cur_steps - (steps - cfg.steps()));
    output.unwrap_or_default()
}

fn fn_limit_steps_mut(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let (steps, input) = match steps_input(cfg, input) {
        Ok((steps, input)) => (steps, input),
        Err(val) => return val,
    };
    let cur_steps = cfg.steps();
    if steps > cur_steps {
        return Eval.mut_call(cfg, ctx, input);
    }
    cfg.set_steps_unchecked(steps);
    let output = StepsExceed::catch(|| Eval.mut_call(cfg, ctx, input));
    cfg.set_steps_unchecked(cur_steps - (steps - cfg.steps()));
    output.unwrap_or_default()
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
