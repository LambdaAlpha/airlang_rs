use {
    crate::{
        semantics::{
            eval::{
                Ctx,
                Func,
                FuncImpl,
                FuncTrait,
                InvariantTag,
                Name,
                Primitive,
                TaggedVal,
            },
            prelude::{
                eval::fn_eval_escape,
                names,
            },
            val::Val,
        },
        types::{
            Either,
            Keeper,
            Owner,
            Reader,
        },
    },
    std::{
        convert::identity,
        mem::swap,
    },
};

pub(crate) fn read() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::READ),
            eval: Reader::new(fn_read),
        }),
    })
    .into()
}

fn fn_read(ctx: &mut Ctx, input: Val) -> Val {
    let name = fn_eval_escape(ctx, input);
    ctx.eval_ref(name, |is_ref| {
        if is_ref {
            Either::Left(Clone::clone)
        } else {
            Either::Right(identity)
        }
    })
}

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
    fn_assign_val(ctx, input, InvariantTag::None)
}

pub(crate) fn assign_final() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN_FINAL),
            eval: Reader::new(fn_assign_final),
        }),
    })
    .into()
}

fn fn_assign_final(ctx: &mut Ctx, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Final)
}

pub(crate) fn assign_const() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN_CONST),
            eval: Reader::new(fn_assign_const),
        }),
    })
    .into()
}

fn fn_assign_const(ctx: &mut Ctx, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Const)
}

fn fn_assign_val(ctx: &mut Ctx, input: Val, tag: InvariantTag) -> Val {
    if let Val::Pair(pair) = input {
        let first = fn_eval_escape(ctx, pair.first);
        match first {
            Val::Symbol(s) => {
                let val = ctx.eval(pair.second);
                return ctx.put_val(
                    Name::from(<_ as Into<String>>::into(s)),
                    TaggedVal { tag, val },
                );
            }
            Val::String(s) => {
                let val = ctx.eval(pair.second);
                return ctx.put_val(
                    Name::from(<_ as Into<String>>::into(s)),
                    TaggedVal { tag, val },
                );
            }
            Val::Box(k) => {
                let mut val = ctx.eval(pair.second);
                if let Ok(mut o) = Keeper::owner(&k.0) {
                    if matches!(o.tag, InvariantTag::None) {
                        swap(&mut o.val, &mut val);
                        o.tag = tag;
                        return val;
                    }
                } else {
                    let _ = Keeper::reinit(&k.0, TaggedVal::new(val));
                }
            }
            _ => {}
        }
    }
    Val::default()
}

pub(crate) fn set_final() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FINAL),
            eval: Reader::new(fn_set_final),
        }),
    })
    .into()
}

fn fn_set_final(ctx: &mut Ctx, input: Val) -> Val {
    let input = fn_eval_escape(ctx, input);
    match input {
        Val::Symbol(s) => ctx.set_final(&s),
        Val::String(s) => ctx.set_final(&s),
        Val::Box(k) => {
            if let Ok(mut o) = Keeper::owner(&k.0) {
                if matches!(o.tag, InvariantTag::None) {
                    o.tag = InvariantTag::Final;
                }
            }
        }
        _ => {}
    }
    Val::default()
}

pub(crate) fn set_const() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::CONST),
            eval: Reader::new(fn_set_const),
        }),
    })
    .into()
}

fn fn_set_const(ctx: &mut Ctx, input: Val) -> Val {
    let input = fn_eval_escape(ctx, input);
    match input {
        Val::Symbol(s) => ctx.set_const(&s),
        Val::String(s) => ctx.set_const(&s),
        Val::Box(k) => {
            if let Ok(mut o) = Keeper::owner(&k.0) {
                o.tag = InvariantTag::Const;
            }
        }
        _ => {}
    }
    Val::default()
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
        Val::Symbol(s) => return ctx.remove(&s),
        Val::String(s) => return ctx.remove(&s),
        Val::Box(k) => {
            if let Ok(o) = Keeper::owner(&k.0) {
                if matches!(o.tag, InvariantTag::None) {
                    return Owner::move_data(o).val;
                }
            }
        }
        _ => {}
    }
    Val::default()
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
    Val::Box(Keeper::new(TaggedVal::new(ctx.eval(input))).into())
}

pub(crate) fn final_box_new() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FINAL_BOX_NEW),
            eval: Reader::new(fn_final_box_new),
        }),
    })
    .into()
}

fn fn_final_box_new(ctx: &mut Ctx, input: Val) -> Val {
    Val::Box(Keeper::new(TaggedVal::new_final(ctx.eval(input))).into())
}

pub(crate) fn const_box_new() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::CONST_BOX_NEW),
            eval: Reader::new(fn_const_box_new),
        }),
    })
    .into()
}

fn fn_const_box_new(ctx: &mut Ctx, input: Val) -> Val {
    Val::Box(Keeper::new(TaggedVal::new_const(ctx.eval(input))).into())
}
