use airlang::cfg::CfgMod;
use airlang::cfg::CoreCfg;
use airlang::cfg::lib::DynPrimFn;
use airlang::cfg::lib::Library;
use airlang::cfg::lib::mut_impl;
use airlang::cfg::lib::setup::default_dyn_mode;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::val::MutPrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Symbol;
use log::error;

#[derive(Clone)]
pub struct EvalLib {
    pub reset: MutPrimFuncVal,
}

impl Default for EvalLib {
    fn default() -> Self {
        Self { reset: reset() }
    }
}

impl CfgMod for EvalLib {
    fn extend(self, cfg: &Cfg) {
        self.reset.extend(cfg);
    }
}

impl Library for EvalLib {
    fn prelude(&self, ctx: &mut Ctx) {
        self.reset.prelude(ctx);
    }
}

// todo rename
pub fn reset() -> MutPrimFuncVal {
    DynPrimFn { id: "repl.reset", f: mut_impl(fn_reset), mode: default_dyn_mode() }.mut_()
}

fn fn_reset(cfg: &mut Cfg, ctx: &mut Val, _input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    if let Some(Val::Ctx(prelude)) = cfg.import(Symbol::from_str_unchecked(CoreCfg::PRELUDE)) {
        *ctx = prelude;
    }
    Val::default()
}
