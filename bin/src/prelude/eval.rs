use airlang::{
    AirCell,
    FuncVal,
    Mode,
    MutCtx,
    MutFnCtx,
    Val,
};

use crate::{
    init_ctx,
    prelude::{
        Named,
        Prelude,
        named_mut_fn,
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
    let call_mode = Mode::default();
    let ask_mode = Mode::default();
    named_mut_fn("repl.reset", call_mode, ask_mode, false, fn_reset)
}

fn fn_reset(ctx: MutFnCtx, _input: Val) -> Val {
    let MutFnCtx::Mut(mut ctx) = ctx else {
        eprintln!("Unable to reset context, context is immutable.");
        return Val::default();
    };
    let mut initial_ctx = AirCell::initial_ctx();
    init_ctx(MutCtx::new(&mut initial_ctx));
    ctx.set(initial_ctx);
    Val::default()
}
