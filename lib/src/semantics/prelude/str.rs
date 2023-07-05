use crate::{
    semantics::{
        eval::{
            Ctx,
            EvalMode,
            Func,
            Primitive,
        },
        prelude::{
            names,
            prelude_func,
        },
        val::Val,
    },
    types::{
        Either,
        Str,
    },
};

pub(crate) fn length() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::STR_LENGTH,
        EvalMode::Inline,
        fn_length,
    )))
}

fn fn_length(ctx: &Ctx, input: Val) -> Val {
    ctx.get_ref_or_val(input, |ref_or_val| {
        let f = |val: &Val| {
            let Val::String(s) = val else {
                return Val::default();
            };
            Val::Int(s.len().into())
        };
        match ref_or_val {
            Either::Left(val) => f(val),
            Either::Right(val) => f(&val),
        }
    })
}

pub(crate) fn concat() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::STR_CONCAT,
        EvalMode::Eval,
        fn_concat,
    )))
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
