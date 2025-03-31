use crate::{
    FuncMode,
    FuncVal,
    MutFnCtx,
    Val,
    ctx::default::DefaultCtx,
    prelude::mut_fn,
};

thread_local!(pub(crate) static MODE_PRELUDE: ModePrelude = ModePrelude::default());

#[derive(Clone)]
pub(crate) struct ModePrelude {
    pub(crate) ref_mode: FuncVal,
}

impl Default for ModePrelude {
    fn default() -> Self {
        Self { ref_mode: ref_mode() }
    }
}

fn ref_mode() -> FuncVal {
    let id = "mode.reference";
    let f = fn_ref_mode;
    let mode = FuncMode::id_func_mode();
    mut_fn(id, f, mode)
}

fn fn_ref_mode(ctx: MutFnCtx, input: Val) -> Val {
    DefaultCtx::eval_escape_symbol(ctx, input)
}
