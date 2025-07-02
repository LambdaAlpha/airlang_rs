use airlang::initial_ctx;
use airlang::prelude::DynFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::mode::FuncMode;
use airlang::prelude::mut_impl;
use airlang::semantics::val::MutStaticPrimFuncVal;
use airlang::semantics::val::Val;
use log::error;

pub struct EvalPrelude {
    pub reset: MutStaticPrimFuncVal,
}

impl Default for EvalPrelude {
    fn default() -> Self {
        Self { reset: reset() }
    }
}

impl Prelude for EvalPrelude {
    fn put(&self, ctx: &mut dyn PreludeCtx) {
        self.reset.put(ctx);
    }
}

// todo rename
pub fn reset() -> MutStaticPrimFuncVal {
    DynFn {
        id: "repl.reset",
        f: mut_impl(fn_reset),
        mode: FuncMode::default(),
        ctx_explicit: false,
    }
    .mut_static()
}

fn fn_reset(ctx: &mut Val, _input: Val) -> Val {
    let Val::Ctx(ctx) = ctx else {
        error!("ctx {ctx:?} should be a ctx");
        return Val::default();
    };
    **ctx = initial_ctx();
    Val::default()
}
