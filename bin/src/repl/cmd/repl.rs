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

const TITLE_PREFIX: &str = "🜁 Air ";

pub(crate) fn title(_: &ConstCtx, dyn_ctx: &mut DynCtx, _: Repr) -> Output {
    match parse(include_str!("../../air/version.air")) {
        Ok(repr) => match dyn_ctx.interpreter.interpret(repr) {
            Ok(repr) => match repr {
                Repr::String(s) => Output::Ok(Box::new(format!("{}{}", TITLE_PREFIX, &*s))),
                repr => Output::Ok(Box::new(format!("{}{}", TITLE_PREFIX, repr))),
            },
            Err(err) => Output::Err(Box::new(err)),
        },
        Err(err) => Output::Err(Box::new(err)),
    }
}

pub(crate) fn quit(_: &ConstCtx, _: &mut DynCtx, _: Repr) -> Output {
    Output::Break
}

pub(crate) fn exit(_: &ConstCtx, _: &mut DynCtx, _: Repr) -> Output {
    Output::Break
}
