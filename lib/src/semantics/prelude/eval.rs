use crate::{
    semantics::{
        eval::{
            strategy::{
                eval::{
                    DefaultByRefStrategy,
                    DefaultStrategy,
                },
                inline::InlineStrategy,
                interpolate::InterpolateStrategy,
                ByRefStrategy,
                EvalStrategy,
            },
            Composed,
            ComposedCtxFn,
            Ctx,
            EvalMode,
            Func,
            Name,
            Primitive,
        },
        prelude::{
            names,
            prelude_func,
        },
        val::{
            CtxVal,
            MapVal,
            Val,
        },
    },
    types::{
        Either,
        Keeper,
        Reader,
        Str,
        Symbol,
    },
};

pub(crate) fn value() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::VALUE,
        EvalMode::Value,
        fn_value,
    )))
}

fn fn_value(input: Val) -> Val {
    input
}

pub(crate) fn eval() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL,
        EvalMode::Eval,
        fn_eval,
    )))
}

fn fn_eval(input: Val) -> Val {
    input
}

pub(crate) fn eval_interpolate() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL_INTERPOLATE,
        EvalMode::Interpolate,
        fn_eval_interpolate,
    )))
}

fn fn_eval_interpolate(input: Val) -> Val {
    input
}

pub(crate) fn eval_inline() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL_INLINE,
        EvalMode::Inline,
        fn_eval_inline,
    )))
}

fn fn_eval_inline(input: Val) -> Val {
    input
}

pub(crate) fn eval_twice() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::EVAL_TWICE,
        EvalMode::Value,
        fn_eval_twice,
    )))
}

fn fn_eval_twice(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Ref(k) => {
            let Ok(input) = Keeper::reader(&k.0) else {
                return Val::default();
            };
            DefaultByRefStrategy::eval(ctx, &input.val)
        }
        i => {
            let val = DefaultStrategy::eval(ctx, i);
            DefaultStrategy::eval(ctx, val)
        }
    }
}

pub(crate) fn eval_thrice() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::EVAL_THRICE,
        EvalMode::Value,
        fn_eval_thrice,
    )))
}

fn fn_eval_thrice(ctx: &mut Ctx, input: Val) -> Val {
    let val = DefaultStrategy::eval(ctx, input);
    fn_eval_twice(ctx, val)
}

pub(crate) fn eval_in_ctx() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::EVAL_IN_CTX,
        EvalMode::Value,
        fn_eval_in_ctx,
    )))
}

fn fn_eval_in_ctx(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name_or_val = InlineStrategy::eval(ctx, pair.first);
    let val = InterpolateStrategy::eval(ctx, pair.second);
    ctx.get_mut_or_val(name_or_val, |ref_or_val| {
        let f = |target: &mut Val| {
            let Val::Ctx(CtxVal(target_ctx)) = target else {
                return Val::default();
            };
            DefaultStrategy::eval(target_ctx, val)
        };
        match ref_or_val {
            Either::Left(r) => f(r),
            Either::Right(mut val) => f(&mut val),
        }
    })
}

pub(crate) fn parse() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::PARSE,
        EvalMode::Eval,
        fn_parse,
    )))
}

fn fn_parse(input: Val) -> Val {
    let Val::String(input) = input else {
        return Val::default();
    };
    crate::semantics::parse(&input).unwrap_or_default()
}

pub(crate) fn stringify() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::STRINGIFY,
        EvalMode::Eval,
        fn_stringify,
    )))
}

fn fn_stringify(input: Val) -> Val {
    let Ok(str) = crate::semantics::generate(&input) else {
        return Val::default();
    };
    Val::String(Str::from(str))
}

pub(crate) fn func() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::FUNC,
        EvalMode::Interpolate,
        fn_func,
    )))
}

fn fn_func(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let body = map_remove(&mut map, "body");
    let func_ctx = match map_remove(&mut map, "context") {
        Val::Ctx(func_ctx) => *func_ctx.0,
        Val::Unit(_) => Ctx::default(),
        _ => return Val::default(),
    };
    let input_name = match map_remove(&mut map, "input") {
        Val::Symbol(Symbol(name)) => name,
        Val::Unit(_) => Name::from("input"),
        _ => return Val::default(),
    };
    let eval_mode = match map_remove(&mut map, "eval_mode") {
        Val::Symbol(Symbol(name)) => match &*name {
            names::VALUE => EvalMode::Value,
            names::EVAL => EvalMode::Eval,
            names::EVAL_INTERPOLATE => EvalMode::Interpolate,
            names::EVAL_INLINE => EvalMode::Inline,
            _ => return Val::default(),
        },
        Val::Unit(_) => EvalMode::Eval,
        _ => return Val::default(),
    };
    let caller_name = match map_remove(&mut map, "caller_name") {
        Val::Symbol(Symbol(name)) => name,
        Val::Unit(_) => Name::from("caller"),
        _ => return Val::default(),
    };
    let ctx_fn = match map_remove(&mut map, "caller_context") {
        Val::Symbol(s) => match &*s {
            "free" => ComposedCtxFn::Free,
            "const" => ComposedCtxFn::Const { caller_name },
            "aware" => ComposedCtxFn::Aware { caller_name },
            _ => return Val::default(),
        },
        Val::Unit(_) => ComposedCtxFn::Free,
        _ => return Val::default(),
    };

    let func = Func::new_composed(Composed {
        body,
        ctx: func_ctx,
        input_name,
        eval_mode,
        ctx_fn,
    });

    Val::Func(Reader::new(func).into())
}

fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or_default()
}

pub(crate) fn chain() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::CHAIN,
        EvalMode::Value,
        fn_chain,
    )))
}

fn fn_chain(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultStrategy::eval_call(ctx, pair.second, pair.first)
}
