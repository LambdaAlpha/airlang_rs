use airlang::{
    initial_ctx,
    FuncVal,
    Mode,
    MutCtx,
    MutFnCtx,
    Val,
};

use crate::{
    init_ctx,
    prelude::{
        named_mut_fn,
        Named,
        Prelude,
    },
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
    fn put(&self, mut ctx: MutCtx) {
        self.reset.put(ctx.reborrow());
    }
}

fn reset() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("repl.reset", input_mode, output_mode, false, fn_reset)
}

fn fn_reset(ctx: MutFnCtx, _input: Val) -> Val {
    let MutFnCtx::Mut(mut ctx) = ctx else {
        eprintln!("Unable to reset context, context is immutable.");
        return Val::default();
    };
    let mut initial_ctx = initial_ctx();
    init_ctx(MutCtx::new(&mut initial_ctx));
    ctx.set(initial_ctx);
    Val::default()
}
