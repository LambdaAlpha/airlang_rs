use crate::{
    semantics::{
        eval_mode::EvalMode,
        func::{
            CtxFreeFn,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            names,
            PrimitiveFunc,
        },
        val::Val,
    },
    types::Str,
};

pub(crate) fn parse() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::PARSE, fn_parse);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_parse(input: Val) -> Val {
    let Val::String(input) = input else {
        return Val::default();
    };
    crate::semantics::parse(&input).unwrap_or_default()
}

pub(crate) fn stringify() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::STRINGIFY, fn_stringify);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_stringify(input: Val) -> Val {
    let Ok(str) = crate::semantics::generate(&input) else {
        return Val::default();
    };
    Val::String(Str::from(str))
}
