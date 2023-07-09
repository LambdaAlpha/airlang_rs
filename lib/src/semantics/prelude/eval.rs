use crate::{
    semantics::{
        eval::{
            strategy::{
                eval::{
                    DefaultByRefStrategy,
                    DefaultStrategy,
                },
                ByRefStrategy,
                EvalStrategy,
            },
            BasicEvalMode,
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
        EvalMode::Basic(BasicEvalMode::Value),
        fn_value,
    )))
}

fn fn_value(input: Val) -> Val {
    input
}

pub(crate) fn eval() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_eval,
    )))
}

fn fn_eval(input: Val) -> Val {
    input
}

pub(crate) fn eval_interpolate() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL_INTERPOLATE,
        EvalMode::Basic(BasicEvalMode::Interpolate),
        fn_eval_interpolate,
    )))
}

fn fn_eval_interpolate(input: Val) -> Val {
    input
}

pub(crate) fn eval_inline() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL_INLINE,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_eval_inline,
    )))
}

fn fn_eval_inline(input: Val) -> Val {
    input
}

pub(crate) fn eval_twice() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::EVAL_TWICE,
        EvalMode::Basic(BasicEvalMode::Value),
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
        EvalMode::Basic(BasicEvalMode::Value),
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
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Interpolate,
            non_pair: BasicEvalMode::Value,
        },
        fn_eval_in_ctx,
    )))
}

fn fn_eval_in_ctx(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name_or_val = pair.first;
    let val = pair.second;
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
        EvalMode::Basic(BasicEvalMode::Eval),
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
        EvalMode::Basic(BasicEvalMode::Eval),
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
        EvalMode::Basic(BasicEvalMode::Interpolate),
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

    let eval_mode = map_remove(&mut map, "eval_mode");
    let default_eval_mode = if let Val::Unit(_) = eval_mode {
        BasicEvalMode::Eval
    } else if let Some(eval_mode) = parse_eval_mode(eval_mode) {
        eval_mode
    } else {
        return Val::default();
    };
    let pair_eval_mode = map_remove(&mut map, "pair_eval_mode");
    let eval_mode = match pair_eval_mode {
        Val::Pair(pair) => {
            let Some(first) = parse_eval_mode(pair.first) else {
                return Val::default();
            };
            let Some(second) = parse_eval_mode(pair.second) else {
                return Val::default();
            };
            EvalMode::Pair {
                first,
                second,
                non_pair: default_eval_mode,
            }
        }
        Val::Unit(_) => EvalMode::Basic(default_eval_mode),
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

fn parse_eval_mode(val: Val) -> Option<BasicEvalMode> {
    let Val::Symbol(Symbol(name)) = val else {
        return None;
    };
    let eval_mode = match &*name {
        names::VALUE => BasicEvalMode::Value,
        names::EVAL => BasicEvalMode::Eval,
        names::EVAL_INTERPOLATE => BasicEvalMode::Interpolate,
        names::EVAL_INLINE => BasicEvalMode::Inline,
        _ => return None,
    };
    Some(eval_mode)
}

pub(crate) fn chain() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::CHAIN,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_chain,
    )))
}

fn fn_chain(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    DefaultStrategy::eval_call(ctx, pair.second, pair.first)
}
