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
            prelude::names,
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
        ops::Deref,
    },
};

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
    Val::Keeper(Keeper::new(ctx.eval(&input)))
}

pub(crate) fn box_read() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_READ),
            eval: Reader::new(fn_box_read),
        }),
    })
    .into()
}

fn fn_box_read(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::Keeper(k) => Keeper::reader(&k)
            .map(|i| i.deref().clone())
            .unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn box_move() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_MOVE),
            eval: Reader::new(fn_box_move),
        }),
    })
    .into()
}

fn fn_box_move(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Keeper(k) = ctx.eval(&input) {
        return Keeper::owner(&k).map(Owner::move_data).unwrap_or_default();
    }
    Val::default()
}

pub(crate) fn box_assign() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_ASSIGN),
            eval: Reader::new(fn_box_assign),
        }),
    })
    .into()
}

fn fn_box_assign(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Keeper(k) = ctx.eval(&pair.first) {
            let mut val = ctx.eval(&pair.second);
            if let Ok(owner) = Keeper::owner(&k) {
                swap(Owner::borrow_mut(&owner), &mut val);
                return val;
            } else {
                let _ = Keeper::reinit(&k, val);
            }
        }
    }
    Val::default()
}
