use {
    crate::{
        semantics::{
            eval::{
                Ctx,
                Func,
                FuncImpl,
                FuncTrait,
                Name,
                Primitive,
            },
            prelude::{
                eval::fn_eval_escape,
                names,
            },
            val::Val,
        },
        types::{
            Either,
            Reader,
        },
    },
    std::mem::swap,
};

pub(crate) fn first() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::PAIR_FIRST),
            eval: Reader::new(fn_first),
        }),
    })
    .into()
}

fn fn_first(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_pair = fn_eval_escape(ctx, input);
    ctx.eval_ref(name_or_pair, |is_ref| {
        if is_ref {
            Either::Left(|val: &Val| match val {
                Val::Pair(pair) => pair.first.clone(),
                _ => Val::default(),
            })
        } else {
            Either::Right(|val| match val {
                Val::Pair(pair) => pair.first,
                _ => Val::default(),
            })
        }
    })
}

pub(crate) fn first_assign() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::PAIR_FIRST_ASSIGN),
            eval: Reader::new(fn_first_assign),
        }),
    })
    .into()
}

fn fn_first_assign(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_val.first);
    let mut val = ctx.eval(name_val.second);
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|pair: &mut Val| {
                let Val::Pair(pair) = pair else {
                    return Val::default();
                };
                swap(&mut pair.first, &mut val);
                val
            })
        } else {
            Either::Right(|_| Val::default())
        }
    })
}

pub(crate) fn second() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::PAIR_SECOND),
            eval: Reader::new(fn_second),
        }),
    })
    .into()
}

fn fn_second(ctx: &mut Ctx, input: Val) -> Val {
    let name_or_pair = fn_eval_escape(ctx, input);
    ctx.eval_ref(name_or_pair, |is_ref| {
        if is_ref {
            Either::Left(|val: &Val| match val {
                Val::Pair(pair) => pair.second.clone(),
                _ => Val::default(),
            })
        } else {
            Either::Right(|val| match val {
                Val::Pair(pair) => pair.second,
                _ => Val::default(),
            })
        }
    })
}

pub(crate) fn second_assign() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::PAIR_SECOND_ASSIGN),
            eval: Reader::new(fn_second_assign),
        }),
    })
    .into()
}

fn fn_second_assign(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(name_val) = input else {
        return Val::default();
    };
    let name = fn_eval_escape(ctx, name_val.first);
    let mut val = ctx.eval(name_val.second);
    ctx.eval_mut(name, |is_ref| {
        if is_ref {
            Either::Left(|pair: &mut Val| {
                let Val::Pair(pair) = pair else {
                    return Val::default();
                };
                swap(&mut pair.second, &mut val);
                val
            })
        } else {
            Either::Right(|_| Val::default())
        }
    })
}
