use {
    crate::repl::eval::{
        ConstCtx,
        DynCtx,
        Output,
    },
    airlang::semantics::Val,
};

pub(crate) fn reset(_: &ConstCtx, dyn_ctx: &mut DynCtx, _: Val) -> Output {
    dyn_ctx.interpreter.reset();
    Output::Ok(Box::new("context reset"))
}
