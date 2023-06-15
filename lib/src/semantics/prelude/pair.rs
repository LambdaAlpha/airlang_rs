use {
    crate::{
        semantics::{
            eval::{
                Ctx,
                Func,
                Primitive,
            },
            prelude::names,
            val::Val,
        },
        types::Either,
    },
    std::mem::swap,
};

pub(crate) fn first() -> Val {
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::PAIR_FIRST,
        fn_first,
    )))
    .into()
}

fn fn_first(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_pair = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_pair, |ref_or_val| match ref_or_val {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::PAIR_FIRST_ASSIGN,
        fn_first_assign,
    )))
    .into()
}

fn fn_first_assign(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_val.first);
    let mut val = ctx.eval(name_val.second);
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::PAIR_SECOND,
        fn_second,
    )))
    .into()
}

fn fn_second(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_pair = ctx.eval_escape(input);
    ctx.get_ref_or_val(name_or_pair, |ref_or_val| match ref_or_val {
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
    Box::new(Func::new_primitive(Primitive::new_ctx_aware(
        names::PAIR_SECOND_ASSIGN,
        fn_second_assign,
    )))
    .into()
}

fn fn_second_assign(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = ctx.eval_escape(name_val.first);
    let mut val = ctx.eval(name_val.second);
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
