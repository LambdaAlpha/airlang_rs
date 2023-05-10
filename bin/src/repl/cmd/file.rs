use {
    crate::repl::eval::{
        ConstCtx,
        DynCtx,
        Output,
    },
    airlang::semantics::{
        parse,
        Val,
    },
};

pub(crate) fn import(const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, val: Val) -> Output {
    if let Val::String(path) = val {
        match std::fs::read_to_string(&*path) {
            Ok(input) => match parse(&input) {
                Ok(input) => const_ctx.eval(dyn_ctx, input),
                Err(err) => Output::Err(Box::new(err)),
            },
            Err(err) => Output::Err(Box::new(err)),
        }
    } else {
        Output::Err("import command only support string argument".into())
    }
}
