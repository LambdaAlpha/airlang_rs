use airlang::AirCell;
use airlang::FuncMode;
use airlang::MutStaticPrimFuncVal;
use airlang::PreludeCtx;
use airlang::Val;

use crate::prelude::DynFn;
use crate::prelude::Prelude;
use crate::prelude::mut_impl;

pub(crate) struct EvalPrelude {
    pub(crate) reset: MutStaticPrimFuncVal,
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

fn reset() -> MutStaticPrimFuncVal {
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
    **ctx = AirCell::initial_ctx();
    Val::default()
}
