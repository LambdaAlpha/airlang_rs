use {
    crate::repl::eval::{
        ConstCtx,
        DynCtx,
        Output,
    },
    airlang::repr::Repr,
};

pub(crate) fn reset(_: &ConstCtx, dyn_ctx: &mut DynCtx, _: Repr) -> Output {
    dyn_ctx.interpreter.reset();
    Output::Ok(Box::new("context reset"))
}
