use crate::{
    semantics::{
        eval::{
            ctx::{
                Ctx,
                TaggedRef,
            },
            strategy::{
                eval::{
                    DefaultByRefStrategy,
                    DefaultConstByRefStrategy,
                    DefaultConstStrategy,
                    DefaultFreeStrategy,
                    DefaultStrategy,
                },
                ByRefStrategy,
                EvalStrategy,
                FreeStrategy,
            },
            BasicEvalMode,
            Composed,
            ComposedCtxFn,
            EvalMode,
            Func,
            IsConst,
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable_dispatch(
        names::EVAL_TWICE,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_eval_twice::<DefaultConstStrategy, DefaultConstByRefStrategy>,
        fn_eval_twice::<DefaultStrategy, DefaultByRefStrategy>,
    )))
}

fn fn_eval_twice<Eval: EvalStrategy, EvalByRef: ByRefStrategy>(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Ref(k) => {
            let Ok(input) = Keeper::reader(&k.0) else {
                return Val::default();
            };
            EvalByRef::eval(ctx, &input.val)
        }
        i => {
            let val = Eval::eval(ctx, i);
            Eval::eval(ctx, val)
        }
    }
}

pub(crate) fn eval_thrice() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable_dispatch(
        names::EVAL_THRICE,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_eval_thrice::<DefaultConstStrategy, DefaultConstByRefStrategy>,
        fn_eval_thrice::<DefaultStrategy, DefaultByRefStrategy>,
    )))
}

fn fn_eval_thrice<Eval: EvalStrategy, EvalByRef: ByRefStrategy>(ctx: &mut Ctx, input: Val) -> Val {
    let val = Eval::eval(ctx, input);
    fn_eval_twice::<Eval, EvalByRef>(ctx, val)
}

pub(crate) fn eval_free() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::EVAL_FREE,
        EvalMode::Basic(BasicEvalMode::Interpolate),
        fn_eval_free,
    )))
}

fn fn_eval_free(input: Val) -> Val {
    DefaultFreeStrategy::eval(input)
}

pub(crate) fn eval_in_ctx() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::EVAL_IN_CTX,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Interpolate,
            non_pair: BasicEvalMode::Value,
        },
        fn_eval_in_ctx,
    )))
}

fn fn_eval_in_ctx(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name_or_val = pair.first;
    let val = pair.second;
    ctx.get_ref_or_val_or_default(is_const, name_or_val, |target_ctx| {
        let f = |target| {
            let TaggedRef {
                val_ref: Val::Ctx(CtxVal(target_ctx)),
                is_const: target_ctx_const,
            } = target
            else {
                return Val::default();
            };
            if target_ctx_const {
                DefaultConstStrategy::eval(target_ctx, val)
            } else {
                DefaultStrategy::eval(target_ctx, val)
            }
        };
        match target_ctx {
            Either::Left(r) => f(r),
            Either::Right(mut val) => f(TaggedRef::new(&mut val, false)),
        }
    })
}

pub(crate) fn eval_in_ctx_const() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::EVAL_IN_CTX_CONST,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Interpolate,
            non_pair: BasicEvalMode::Value,
        },
        fn_eval_in_ctx_const,
    )))
}

fn fn_eval_in_ctx_const(ctx: &mut Ctx, input: Val) -> Val {
    fn_eval_in_ctx(ctx, true, input)
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
            "mutable" => ComposedCtxFn::Mutable { caller_name },
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::CHAIN,
        EvalMode::Basic(BasicEvalMode::Value),
        fn_chain,
    )))
}

fn fn_chain(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    if is_const {
        DefaultConstStrategy::eval_call(ctx, pair.second, pair.first)
    } else {
        DefaultStrategy::eval_call(ctx, pair.second, pair.first)
    }
}
