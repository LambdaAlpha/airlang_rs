use {
    crate::repl::eval::{
        ConstCtx,
        DynCtx,
        Output,
    },
    airlang::semantics::Val,
};

pub(crate) fn reset(_const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, _input: Val) -> Output {
    dyn_ctx.interpreter.reset();
    Output::Ok(Box::new("context reset"))
}
