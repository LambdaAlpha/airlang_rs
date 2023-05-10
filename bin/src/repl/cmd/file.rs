use {
    crate::repl::eval::{
        ConstCtx,
        DynCtx,
        Output,
    },
    airlang::{
        repr::Repr,
        syntax::parse,
    },
};

pub(crate) fn import(const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, repr: Repr) -> Output {
    if let Repr::String(path) = repr {
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
