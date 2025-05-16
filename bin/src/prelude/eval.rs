use airlang::AirCell;
use airlang::Ctx;
use airlang::FuncMode;
use airlang::FuncVal;
use airlang::PreludeCtx;
use airlang::Val;

use crate::prelude::Named;
use crate::prelude::Prelude;
use crate::prelude::mut_impl;
use crate::prelude::named_mut_fn;

pub(crate) struct EvalPrelude {
    pub(crate) reset: Named<FuncVal>,
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

fn reset() -> Named<FuncVal> {
    let id = "repl.reset";
    let f = mut_impl(fn_reset);
    let mode = FuncMode::default();
    named_mut_fn(id, f, mode)
}

fn fn_reset(ctx: &mut Ctx, _input: Val) -> Val {
    *ctx = AirCell::initial_ctx();
    Val::default()
}
