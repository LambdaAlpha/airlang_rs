use airlang::{
    EvalMode,
    FuncVal,
    IoMode,
    MutableCtx,
    Val,
};

use crate::prelude::{
    named_free_fn,
    Named,
    Prelude,
};

pub(crate) struct ReplPrelude {
    pub(crate) exit: Named<FuncVal>,
}

impl Default for ReplPrelude {
    fn default() -> Self {
        Self { exit: exit() }
    }
}

impl Prelude for ReplPrelude {
    fn put(&self, mut ctx: MutableCtx) {
        self.exit.put(ctx.reborrow());
    }
}

fn exit() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = IoMode::Eval(EvalMode::Value);
    named_free_fn("repl.exit", input_mode, output_mode, fn_exit)
}

fn fn_exit(_input: Val) -> Val {
    std::process::exit(0)
}
