use {
    crate::{
        semantics::{
            eval::{
                Ctx,
                EvalMode,
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
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NEW_BOX),
            eval: ImRef::new(fn_new_box),
        }),
    })
}

fn fn_new_box(_: &mut Ctx, input: Val) -> Val {
    Val::BoxRef(BoxRef::new(input))
}

pub(crate) fn new_im() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NEW_IM),
            eval: ImRef::new(fn_new_im),
        }),
    })
}

fn fn_new_im(_: &mut Ctx, input: Val) -> Val {
    Val::ImRef(ImRef::new(input))
}

pub(crate) fn new_mut() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NEW_MUT),
            eval: ImRef::new(fn_new_mut),
        }),
    })
}

fn fn_new_mut(_: &mut Ctx, input: Val) -> Val {
    Val::MutRef(MutRef::new(input))
}

pub(crate) fn ref_box() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF_BOX),
            eval: ImRef::new(fn_ref_box),
        }),
    })
}

fn fn_ref_box(_: &mut Ctx, input: Val) -> Val {
    match input {
        Val::BoxRef(b) => b.ref_box().map(|b| Val::BoxRef(b)).unwrap_or_default(),
        Val::ImRef(i) => i.ref_box().map(|b| Val::BoxRef(b)).unwrap_or_default(),
        Val::MutRef(m) => m.ref_box().map(|b| Val::BoxRef(b)).unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn ref_im() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF_IM),
            eval: ImRef::new(fn_ref_im),
        }),
    })
}

fn fn_ref_im(_: &mut Ctx, input: Val) -> Val {
    match input {
        Val::BoxRef(b) => b.ref_im().map(|i| Val::ImRef(i)).unwrap_or_default(),
        Val::ImRef(i) => i.ref_im().map(|i| Val::ImRef(i)).unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn ref_mut() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF_MUT),
            eval: ImRef::new(fn_ref_mut),
        }),
    })
}

fn fn_ref_mut(_: &mut Ctx, input: Val) -> Val {
    match input {
        Val::BoxRef(b) => b.ref_mut().map(|m| Val::MutRef(m)).unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn deref_im() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Eval,
        },
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::DEREF_IM),
            eval: ImRef::new(fn_deref_im),
        }),
    })
}

fn fn_deref_im(_: &mut Ctx, input: Val) -> Val {
    match input {
        Val::ImRef(i) => i.deref().try_clone().unwrap_or_default(),
        Val::MutRef(m) => m.deref().try_clone().unwrap_or_default(),
        _ => Val::default(),
    }
}

pub(crate) fn deref_mut() -> Val {
    Val::Func(Func {
        func_trait: FuncTrait {
            input_eval_mode: EvalMode::Val,
        },
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
            swap(m.borrow_mut(), &mut val);
            return val;
        }
    }
    Val::default()
}
