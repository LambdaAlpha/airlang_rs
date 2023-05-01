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
            BoxRef,
            ImRef,
            MutRef,
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
            id: Name::from(names::NEW_BOX),
            eval: ImRef::new(fn_new_box),
        }),
    })
}

fn fn_new_box(ctx: &mut Ctx, input: Val) -> Val {
    Val::BoxRef(BoxRef::new(ctx.eval(&input)))
}

pub(crate) fn new_im() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NEW_IM),
            eval: ImRef::new(fn_new_im),
        }),
    })
}

fn fn_new_im(ctx: &mut Ctx, input: Val) -> Val {
    Val::ImRef(ImRef::new(ctx.eval(&input)))
}

pub(crate) fn new_mut() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NEW_MUT),
            eval: ImRef::new(fn_new_mut),
        }),
    })
}

fn fn_new_mut(ctx: &mut Ctx, input: Val) -> Val {
    Val::MutRef(MutRef::new(ctx.eval(&input)))
}

pub(crate) fn ref_box() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF_BOX),
            eval: ImRef::new(fn_ref_box),
        }),
    })
}

fn fn_ref_box(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::BoxRef(b) => BoxRef::ref_box(&b)
            .map(|b| Val::BoxRef(b))
            .unwrap_or_default(),
        Val::ImRef(i) => ImRef::ref_box(&i)
            .map(|b| Val::BoxRef(b))
            .unwrap_or_default(),
        Val::MutRef(m) => MutRef::ref_box(&m)
            .map(|b| Val::BoxRef(b))
            .unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn ref_im() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF_IM),
            eval: ImRef::new(fn_ref_im),
        }),
    })
}

fn fn_ref_im(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::BoxRef(b) => BoxRef::ref_im(&b)
            .map(|i| Val::ImRef(i))
            .unwrap_or_default(),
        Val::ImRef(i) => ImRef::ref_im(&i).map(|i| Val::ImRef(i)).unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn ref_mut() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF_MUT),
            eval: ImRef::new(fn_ref_mut),
        }),
    })
}

fn fn_ref_mut(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::BoxRef(b) => BoxRef::ref_mut(&b)
            .map(|m| Val::MutRef(m))
            .unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn deref_im() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::DEREF_IM),
            eval: ImRef::new(fn_deref_im),
        }),
    })
}

fn fn_deref_im(ctx: &mut Ctx, input: Val) -> Val {
    match ctx.eval(&input) {
        Val::ImRef(i) => i.deref().try_clone().unwrap_or_default(),
        Val::MutRef(m) => m.deref().try_clone().unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn deref_mut() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::DEREF_MUT),
            eval: ImRef::new(fn_deref_mut),
        }),
    })
}

fn fn_deref_mut(ctx: &mut Ctx, input: Val) -> Val {
    if let Val::Pair(pair) = input {
        if let Val::MutRef(m) = ctx.eval(&pair.first) {
            let mut val = ctx.eval(&pair.second);
            swap(MutRef::borrow_mut(&m), &mut val);
            return val;
        }
    }
    Val::default()
}
