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

pub(crate) fn title(_const_ctx: &ConstCtx, dyn_ctx: &mut DynCtx, _input: Val) -> Output {
    match parse(include_str!("../../air/version.air")) {
        Ok(repr) => match dyn_ctx.interpreter.interpret(repr) {
            Val::String(s) => Output::Ok(Box::new(format!("{}{}", TITLE_PREFIX, &*s))),
            repr => match generate(&repr) {
                Ok(s) => Output::Ok(Box::new(format!("{TITLE_PREFIX}{s}"))),
                Err(err) => Output::Err(Box::new(err)),
            },
        },
        Err(err) => Output::Err(Box::new(err)),
    }
}

pub(crate) fn quit(_const_ctx: &ConstCtx, _dyn_ctx: &mut DynCtx, _input: Val) -> Output {
    Output::Break
}

pub(crate) fn exit(_const_ctx: &ConstCtx, _dyn_ctx: &mut DynCtx, _input: Val) -> Output {
    Output::Break
}
