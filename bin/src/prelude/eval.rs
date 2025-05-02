use airlang::{
    AirCell,
    FuncMode,
    FuncVal,
    MutFnCtx,
    PreludeCtx,
    Val,
};

use crate::prelude::{
    Named,
    Prelude,
    named_mut_fn,
};

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
    let f = fn_reset;
    let mode = FuncMode::default();
    named_mut_fn(id, f, mode)
}

fn fn_reset(ctx: MutFnCtx, _input: Val) -> Val {
    let MutFnCtx::Mut(mut ctx) = ctx else {
        eprintln!("Unable to reset context, context is immutable.");
        return Val::default();
    };
    ctx.set(AirCell::initial_ctx());
    Val::default()
}
