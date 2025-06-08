use airlang::Air;
use airlang::prelude::DynFn;
use airlang::prelude::Prelude;
use airlang::prelude::PreludeCtx;
use airlang::prelude::mut_impl;
use airlang::semantics::func::FuncMode;
use airlang::semantics::val::MutStaticPrimFuncVal;
use airlang::semantics::val::Val;

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
        return Val::default();
    };
    **ctx = Air::initial_ctx();
    Val::default()
}
