use crate::{
    semantics::{
        eval::{
            ctx::{
                constant::CtxForConstFn,
                CtxTrait,
            },
            BasicEvalMode,
            CtxConstFn,
            CtxFreeFn,
            EvalMode,
            Primitive,
        },
        prelude::{
            names,
            PrimitiveFunc,
        },
        val::Val,
    },
    types::{
        Either,
        Str,
    },
};

pub(crate) fn length() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::STR_LENGTH, fn_length);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_length(mut ctx: CtxForConstFn, input: Val) -> Val {
    ctx.get_ref_or_val_or_default(input, |ref_or_val| {
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

pub(crate) fn concat() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::STR_CONCAT, fn_concat);
    PrimitiveFunc::new(eval_mode, primitive)
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
