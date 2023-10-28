use crate::{
    semantics::{
        ctx::DefaultCtx,
        ctx_access::constant::CtxForConstFn,
        eval_mode::EvalMode,
        func::{
            CtxConstFn,
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

pub(crate) fn length() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::STR_LENGTH, fn_length);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_length(ctx: CtxForConstFn, input: Val) -> Val {
    DefaultCtx.get_const_ref(&ctx, input, |val| {
        let Val::String(s) = val else {
            return Val::default();
        };
        Val::Int(s.len().into())
    })
}

pub(crate) fn concat() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::List(EvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::STR_CONCAT, fn_concat);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_concat(input: Val) -> Val {
    let Val::List(strings) = input else {
        return Val::default();
    };
    let mut ret = String::new();
    for str in strings {
        let Val::String(str) = str else {
            return Val::default();
        };
        ret.push_str(&str);
    }
    Val::String(Str::from(ret))
}
