use airlang::{
    initial_ctx,
    CtxForMutableFn,
    EvalMode,
    FuncVal,
    IoMode,
    MutableCtx,
    Val,
};

use crate::{
    init_ctx,
    prelude::{
        named_mutable_fn,
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
    fn put(&self, mut ctx: MutableCtx) {
        self.reset.put(ctx.reborrow());
    }
}

fn reset() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = IoMode::Eval(EvalMode::Value);
    named_mutable_fn("repl.reset", input_mode, output_mode, fn_reset)
}

fn fn_reset(ctx: CtxForMutableFn, _input: Val) -> Val {
    let CtxForMutableFn::Mutable(mut ctx) = ctx else {
        eprintln!("Unable to reset context, context is immutable.");
        return Val::default();
    };
    let mut initial_ctx = initial_ctx();
    init_ctx(MutableCtx::new(&mut initial_ctx));
    ctx.set(initial_ctx);
    Val::default()
}
