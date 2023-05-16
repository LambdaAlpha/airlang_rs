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
            Keeper,
            Reader,
        },
    },
    std::{
        mem::swap,
        ops::DerefMut,
    },
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
    match fn_eval_escape(ctx, input) {
        Val::Symbol(s) => {
            if let Some(val) = ctx.get_ref(&s) {
                fn_first_ref(val)
            } else {
                Val::default()
            }
        }
        Val::String(s) => {
            if let Some(val) = ctx.get_ref(&s) {
                fn_first_ref(val)
            } else {
                Val::default()
            }
        }
        Val::Keeper(k) => {
            if let Ok(i) = Keeper::reader(&k) {
                fn_first_ref(&i)
            } else {
                Val::default()
            }
        }
        Val::Pair(pair) => pair.first,
        _ => Val::default(),
    }
}

fn fn_first_ref(input: &Val) -> Val {
    match input {
        Val::Pair(p) => p.first.clone(),
        _ => Val::default(),
    }
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
    if let Val::Pair(name_val) = input {
        let name = fn_eval_escape(ctx, name_val.first);
        match name {
            Val::Symbol(s) => {
                let mut val = ctx.eval(name_val.second);
                let pair = ctx.get_mut(&s);
                if let Some(Val::Pair(pair)) = pair {
                    swap(&mut pair.first, &mut val);
                    return val;
                }
            }
            Val::String(s) => {
                let mut val = ctx.eval(name_val.second);
                let pair = ctx.get_mut(&s);
                if let Some(Val::Pair(pair)) = pair {
                    swap(&mut pair.first, &mut val);
                    return val;
                }
            }
            Val::Keeper(k) => {
                let mut val = ctx.eval(name_val.second);
                if let Ok(mut o) = Keeper::owner(&k) {
                    if let Val::Pair(pair) = o.deref_mut() {
                        swap(&mut pair.first, &mut val);
                        return val;
                    }
                }
            }
            _ => {}
        }
    }
    Val::default()
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
    match fn_eval_escape(ctx, input) {
        Val::Symbol(s) => {
            if let Some(val) = ctx.get_ref(&s) {
                fn_second_ref(val)
            } else {
                Val::default()
            }
        }
        Val::String(s) => {
            if let Some(val) = ctx.get_ref(&s) {
                fn_second_ref(val)
            } else {
                Val::default()
            }
        }
        Val::Keeper(k) => {
            if let Ok(i) = Keeper::reader(&k) {
                fn_second_ref(&i)
            } else {
                Val::default()
            }
        }
        Val::Pair(pair) => pair.second,
        _ => Val::default(),
    }
}

fn fn_second_ref(input: &Val) -> Val {
    match input {
        Val::Pair(p) => p.second.clone(),
        _ => Val::default(),
    }
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
    if let Val::Pair(name_val) = input {
        let name = fn_eval_escape(ctx, name_val.first);
        match name {
            Val::Symbol(s) => {
                let mut val = ctx.eval(name_val.second);
                let pair = ctx.get_mut(&s);
                if let Some(Val::Pair(pair)) = pair {
                    swap(&mut pair.second, &mut val);
                    return val;
                }
            }
            Val::String(s) => {
                let mut val = ctx.eval(name_val.second);
                let pair = ctx.get_mut(&s);
                if let Some(Val::Pair(pair)) = pair {
                    swap(&mut pair.second, &mut val);
                    return val;
                }
            }
            Val::Keeper(k) => {
                let mut val = ctx.eval(name_val.second);
                if let Ok(mut o) = Keeper::owner(&k) {
                    if let Val::Pair(pair) = o.deref_mut() {
                        swap(&mut pair.second, &mut val);
                        return val;
                    }
                }
            }
            _ => {}
        }
    }
    Val::default()
}
