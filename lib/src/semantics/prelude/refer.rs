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
        traits::TryClone,
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

pub(crate) fn new_box() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX),
            eval: Reader::new(fn_box),
        }),
    })
}

fn fn_box(ctx: &mut Ctx, input: Val) -> Val {
    Val::Keeper(Keeper::new(ctx.eval(&input)))
}

pub(crate) fn box_copy() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_COPY),
            eval: Reader::new(fn_box_copy),
        }),
    })
}

fn fn_box_copy(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::Keeper(k) => Keeper::reader(&k)
            .ok()
            .and_then(|i| i.deref().try_clone())
            .unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn box_move_out() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_MOVE_OUT),
            eval: Reader::new(fn_box_move_out),
        }),
    })
}

fn fn_box_move_out(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Keeper(k) = ctx.eval(&input) {
        return Keeper::owner(&k).map(Owner::move_data).unwrap_or_default();
    }
    Val::default()
}

pub(crate) fn box_assign() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::BOX_ASSIGN),
            eval: Reader::new(fn_box_assign),
        }),
    })
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
