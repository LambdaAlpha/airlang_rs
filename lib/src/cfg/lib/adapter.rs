pub(in crate::cfg) use self::repr::EVAL_EVAL;
pub(in crate::cfg) use self::repr::EVAL_ID;
pub(in crate::cfg) use self::repr::EVAL_LITERAL;
pub(in crate::cfg) use self::repr::EVAL_REF;
pub(in crate::cfg) use self::repr::FORM_EVAL;
pub(in crate::cfg) use self::repr::FORM_ID;
pub(in crate::cfg) use self::repr::FORM_LITERAL;
pub(in crate::cfg) use self::repr::FORM_REF;

_____!();

use self::repr::parse;
use super::DynPrimFn;
use super::FreePrimFn;
use super::free_impl;
use crate::cfg::CfgMod;
use crate::cfg::CoreCfg;
use crate::cfg::adapter::ADAPTER_FUNC_ID;
use crate::cfg::adapter::CallPrimAdapter;
use crate::cfg::adapter::SymbolAdapter;
use crate::cfg::adapter::adapter_func;
use crate::cfg::adapter::id_adapter;
use crate::cfg::adapter::prim_adapter;
use crate::cfg::exception::illegal_input;
use crate::semantics::cfg::Cfg;
use crate::semantics::core::Eval;
use crate::semantics::val::FreePrimFuncVal;
use crate::semantics::val::MutPrimFuncVal;
use crate::semantics::val::Val;

#[derive(Clone)]
pub struct AdapterLib {
    pub new: FreePrimFuncVal,
    pub apply: MutPrimFuncVal,
}

impl Default for AdapterLib {
    fn default() -> Self {
        Self { new: new(), apply: apply() }
    }
}

impl CfgMod for AdapterLib {
    fn extend(self, cfg: &Cfg) {
        let new_adapter = prim_adapter(SymbolAdapter::Literal, CallPrimAdapter::Form);
        CoreCfg::extend_adapter(cfg, &self.new.id, new_adapter);
        self.new.extend(cfg);
        CoreCfg::extend_adapter(cfg, ADAPTER_FUNC_ID, id_adapter());
        let apply_adapter = prim_adapter(SymbolAdapter::Ref, CallPrimAdapter::Form);
        CoreCfg::extend_adapter(cfg, &self.apply.id, apply_adapter);
        self.apply.extend(cfg);
    }
}

pub fn new() -> FreePrimFuncVal {
    FreePrimFn { id: "adapter.new", f: free_impl(fn_new) }.free()
}

fn fn_new(_cfg: &mut Cfg, input: Val) -> Val {
    let Some(adapter) = parse(input) else {
        return illegal_input();
    };
    let func = adapter_func(adapter);
    Val::Func(func)
}

pub fn apply() -> MutPrimFuncVal {
    DynPrimFn { id: "adapter.apply", f: Eval }.mut_()
}

mod repr;
