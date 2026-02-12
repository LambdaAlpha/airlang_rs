use const_format::concatcp;
use num_traits::Signed;
use num_traits::ToPrimitive;

use crate::bug;
use crate::cfg::CfgMod;
use crate::cfg::extend_func;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::core::PREFIX_ID;
use crate::semantics::func::CtxFreeInputEvalFunc;
use crate::semantics::func::CtxFreeInputFreeFunc;
use crate::semantics::func::CtxMutInputRawFunc;
use crate::semantics::func::DynFunc;
use crate::semantics::val::PrimFuncVal;
use crate::semantics::val::Val;
use crate::type_::Int;
use crate::type_::Pair;

#[derive(Clone)]
pub struct ResourceLib {
    pub get_steps: PrimFuncVal,
    pub set_steps: PrimFuncVal,
    pub measure_steps: PrimFuncVal,
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

pub fn get_steps() -> PrimFuncVal {
    CtxFreeInputFreeFunc { fn_: fn_get_steps }.build()
}

fn fn_get_steps(cfg: &mut Cfg) -> Val {
    let steps = cfg.steps();
    Val::Int(Int::from(steps).into())
}

pub fn set_steps() -> PrimFuncVal {
    CtxFreeInputEvalFunc { fn_: fn_set_steps }.build()
}

fn fn_set_steps(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Int(steps) = input else {
        return bug!(cfg, "{SET_STEPS}: expected input to be an integer, but got {input}");
    };
    if steps.is_negative() {
        return bug!(cfg, "{SET_STEPS}: expected input to be non-negative, but got {steps}");
    }
    let steps = steps.to_u128().unwrap_or(u128::MAX);
    cfg.set_steps(steps);
    Val::default()
}

pub fn measure_steps() -> PrimFuncVal {
    CtxMutInputRawFunc { fn_: fn_measure_steps }.build()
}

fn fn_measure_steps(cfg: &mut Cfg, ctx: &mut Val, input: Val) -> Val {
    let old_steps = cfg.steps();
    let output = Eval.call(cfg, ctx, input);
    let steps = old_steps - cfg.steps();
    let steps = Val::Int(Int::from(steps).into());
    Val::Pair(Pair::new(output, steps).into())
}
