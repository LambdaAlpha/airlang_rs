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

pub(crate) fn into_keeper() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INTO_KEEPER),
            eval: Reader::new(fn_into_keeper),
        }),
    })
}

fn fn_into_keeper(ctx: &mut Ctx, input: Val) -> Val {
    Val::Keeper(Keeper::new(ctx.eval(&input)))
}

pub(crate) fn into_reader() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INTO_READER),
            eval: Reader::new(fn_into_reader),
        }),
    })
}

fn fn_into_reader(ctx: &mut Ctx, input: Val) -> Val {
    Val::Reader(Reader::new(ctx.eval(&input)))
}

pub(crate) fn into_owner() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::INTO_OWNER),
            eval: Reader::new(fn_into_owner),
        }),
    })
}

fn fn_into_owner(ctx: &mut Ctx, input: Val) -> Val {
    Val::Owner(Owner::new(ctx.eval(&input)))
}

pub(crate) fn share_keeper() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::SHARE_KEEPER),
            eval: Reader::new(fn_share_keeper),
        }),
    })
}

fn fn_share_keeper(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::Keeper(b) => Keeper::keeper(&b)
            .map(|b| Val::Keeper(b))
            .unwrap_or_default(),
        Val::Reader(i) => Reader::keeper(&i)
            .map(|b| Val::Keeper(b))
            .unwrap_or_default(),
        Val::Owner(m) => Owner::keeper(&m)
            .map(|b| Val::Keeper(b))
            .unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn share_reader() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::SHARE_READER),
            eval: Reader::new(fn_share_reader),
        }),
    })
}

fn fn_share_reader(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::Keeper(b) => Keeper::reader(&b)
            .map(|i| Val::Reader(i))
            .unwrap_or_default(),
        Val::Reader(i) => Reader::reader(&i)
            .map(|i| Val::Reader(i))
            .unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn share_owner() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::SHARE_OWNER),
            eval: Reader::new(fn_share_owner),
        }),
    })
}

fn fn_share_owner(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::Keeper(b) => Keeper::owner(&b).map(|m| Val::Owner(m)).unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn from_reader() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FROM_READER),
            eval: Reader::new(fn_from_reader),
        }),
    })
}

fn fn_from_reader(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::Reader(i) => i.deref().try_clone().unwrap_or_default(),
        Val::Owner(m) => m.deref().try_clone().unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn from_owner() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FROM_OWNER),
            eval: Reader::new(fn_from_owner),
        }),
    })
}

fn fn_from_owner(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Owner(m) = ctx.eval(&input) {
        return Owner::move_data(m);
    }
    Val::default()
}

pub(crate) fn assign_owner() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN_OWNER),
            eval: Reader::new(fn_assign_owner),
        }),
    })
}

fn fn_assign_owner(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::Owner(m) = ctx.eval(&pair.first) {
            let mut val = ctx.eval(&pair.second);
            swap(Owner::borrow_mut(&m), &mut val);
            return val;
        }
    }
    Val::default()
}
