use {
    crate::repl::eval::{
        ConstCtx,
        DynCtx,
        Output,
    },
    airlang::semantics::{
        generate,
        parse,
        Val,
    },
};

const TITLE_PREFIX: &str = "🜁 Air ";

pub(crate) fn title(_: &ConstCtx, dyn_ctx: &mut DynCtx, _: Val) -> Output {
    match parse(include_str!("../../air/version.air")) {
        Ok(repr) => match dyn_ctx.interpreter.interpret(repr) {
            Val::String(s) => Output::Ok(Box::new(format!("{}{}", TITLE_PREFIX, &*s))),
            repr => match generate(&repr) {
                Ok(s) => Output::Ok(Box::new(format!("{}{}", TITLE_PREFIX, s))),
                Err(err) => Output::Err(Box::new(err)),
            },
        },
        Err(err) => Output::Err(Box::new(err)),
    }
}

pub(crate) fn quit(_: &ConstCtx, _: &mut DynCtx, _: Val) -> Output {
    Output::Break
}

pub(crate) fn exit(_: &ConstCtx, _: &mut DynCtx, _: Val) -> Output {
    Output::Break
}
