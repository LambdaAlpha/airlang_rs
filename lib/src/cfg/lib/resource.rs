use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::adapter::id_adapter;
use crate::cfg::lib::DynPrimFn;
use crate::cfg::lib::FreePrimFn;
use crate::cfg::lib::MutImpl;
use crate::cfg::lib::free_impl;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::func::ConstFn;
use crate::semantics::func::FreeFn;
use crate::semantics::func::MutFn;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::ConstRef;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct ResourceLib {
    pub steps: FreePrimFuncVal,
    pub measure_steps: MutPrimFuncVal,
}

impl Default for ResourceLib {
    fn default() -> Self {
        ResourceLib { steps: steps(), measure_steps: measure_steps() }
    }
}

impl CfgMod for ResourceLib {
    fn extend(self, cfg: &Cfg) {
        self.steps.extend(cfg);
        let steps_adapter = id_adapter();
        CoreCfg::extend_adapter(cfg, &self.measure_steps.id, steps_adapter);
        self.measure_steps.extend(cfg);
    }
}

pub fn steps() -> FreePrimFuncVal {
    FreePrimFn { id: "resource.steps", f: free_impl(fn_steps) }.free()
}

fn fn_steps(cfg: &mut Cfg, _input: Val) -> Val {
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
    let steps = cfg.steps() - old_steps;
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}
