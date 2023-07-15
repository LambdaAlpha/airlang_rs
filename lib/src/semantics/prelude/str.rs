use crate::{
    semantics::{
        eval::{
            ctx::Ctx,
            BasicEvalMode,
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
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_length,
    )))
}

fn fn_length(ctx: &mut Ctx, input: Val) -> Val {
    ctx.get_ref_or_val_or_default(true, input, |ref_or_val| {
        let f = |val: &Val| {
            let Val::String(s) = val else {
                return Val::default();
            };
            Val::Int(s.len().into())
        };
        match ref_or_val {
            Either::Left(val) => f(val.as_const()),
            Either::Right(val) => f(&val),
        }
    })
}

pub(crate) fn concat() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::STR_CONCAT,
        EvalMode::Basic(BasicEvalMode::Eval),
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
