use {
    crate::{
        semantics::{
            eval::{
                strategy::{
                    eval::DefaultStrategy,
                    inline::InlineStrategy,
                    EvalStrategy,
                },
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
        types::Either,
    },
    std::mem::swap,
};

pub(crate) fn first() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::PAIR_FIRST,
        EvalMode::Inline,
        fn_first,
    )))
}

fn fn_first(ctx: &Ctx, input: Val) -> Val {
    ctx.get_ref_or_val(input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val {
            Val::Pair(pair) => pair.first.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Pair(pair) => pair.first,
            _ => Val::default(),
        },
    })
}

pub(crate) fn first_assign() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::PAIR_FIRST_ASSIGN,
        EvalMode::Value,
        fn_first_assign,
    )))
}

fn fn_first_assign(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_val.first);
    let mut val = DefaultStrategy::eval(ctx, name_val.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(pair) => {
            let Val::Pair(pair) = pair else {
                return Val::default();
            };
            swap(&mut pair.first, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}

pub(crate) fn second() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::PAIR_SECOND,
        EvalMode::Inline,
        fn_second,
    )))
}

fn fn_second(ctx: &Ctx, input: Val) -> Val {
    ctx.get_ref_or_val(input, |ref_or_val| match ref_or_val {
        Either::Left(val) => match val {
            Val::Pair(pair) => pair.second.clone(),
            _ => Val::default(),
        },
        Either::Right(val) => match val {
            Val::Pair(pair) => pair.second,
            _ => Val::default(),
        },
    })
}

pub(crate) fn second_assign() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::PAIR_SECOND_ASSIGN,
        EvalMode::Value,
        fn_second_assign,
    )))
}

fn fn_second_assign(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = InlineStrategy::eval(ctx, name_val.first);
    let mut val = DefaultStrategy::eval(ctx, name_val.second);
    ctx.get_mut_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(pair) => {
            let Val::Pair(pair) = pair else {
                return Val::default();
            };
            swap(&mut pair.second, &mut val);
            val
        }
        Either::Right(_) => Val::default(),
    })
}
