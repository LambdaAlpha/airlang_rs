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
            Owner,
            Reader,
        },
    },
    std::{
        mem::swap,
        ops::DerefMut,
    },
};

pub(crate) fn assign() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN),
            eval: Reader::new(fn_assign),
        }),
    })
    .into()
}

fn fn_assign(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        let first = fn_eval_escape(ctx, pair.first);
        match first {
            Val::Symbol(s) => {
                let val = ctx.eval(pair.second);
                ctx.put(Name::from(<_ as Into<String>>::into(s)), val)
            }
            Val::String(s) => {
                let val = ctx.eval(pair.second);
                ctx.put(Name::from(<_ as Into<String>>::into(s)), val)
            }
            Val::Keeper(k) => {
                let mut val = ctx.eval(pair.second);
                if let Ok(mut o) = Keeper::owner(&k) {
                    swap(o.deref_mut(), &mut val);
                    val
                } else {
                    let _ = Keeper::reinit(&k, val);
                    Val::default()
                }
            }
            _ => Val::default(),
        }
    } else {
        Val::default()
    }
}

pub(crate) fn remove() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::MOVE),
            eval: Reader::new(fn_move),
        }),
    })
    .into()
}

fn fn_move(ctx: &mut Ctx, input: Val) -> Val {
    let input = fn_eval_escape(ctx, input);
    match input {
        Val::Symbol(s) => ctx.remove(&s),
        Val::String(s) => ctx.remove(&s),
        Val::Keeper(k) => Keeper::owner(&k).map(Owner::move_data).unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn box_new() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_NEW),
            eval: Reader::new(fn_box_new),
        }),
    })
    .into()
}

fn fn_box_new(ctx: &mut Ctx, input: Val) -> Val {
    Val::Keeper(Keeper::new(ctx.eval(input)))
}
