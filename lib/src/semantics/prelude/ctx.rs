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
                NameMap,
                Primitive,
                TaggedVal,
            },
            prelude::{
                map::fn_map_new,
                names,
            },
            val::{
                MapVal,
                Val,
            },
        },
        types::{
            Bool,
            Either,
            Keeper,
            Owner,
            Reader,
            Symbol,
        },
    },
    std::mem::swap,
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
    let name = ctx.eval_escape(input);
    ctx.get_ref_or_val(name, |ref_or_val| match ref_or_val {
        Either::Left(r) => r.clone(),
        Either::Right(_) => Val::default(),
    })
}

pub(crate) fn is_null() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::IS_NULL),
            eval: Reader::new(fn_is_null),
        }),
    })
    .into()
}

fn fn_is_null(ctx: &mut Ctx, input: Val) -> Val {
    let name = ctx.eval_escape(input);
    match name {
        Val::Symbol(s) => Val::Bool(Bool::new(ctx.get_ref(&s).is_none())),
        Val::Ref(k) => Val::Bool(Bool::new(Keeper::reader(&k.0).is_err())),
        _ => Val::default(),
    }
}

pub(crate) fn assign_local() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::ASSIGN_LOCAL),
            eval: Reader::new(fn_assign_local),
        }),
    })
    .into()
}

fn fn_assign_local(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(Symbol(name)) = ctx.eval_escape(pair.first) else {
        return Val::default();
    };
    let val = ctx.eval(pair.second);
    ctx.put_val_local(name, TaggedVal::new(val))
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
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let first = ctx.eval_escape(pair.first);
    match first {
        Val::Symbol(s) => {
            let val = ctx.eval(pair.second);
            ctx.put_val(
                Name::from(<_ as Into<String>>::into(s)),
                TaggedVal { tag, val },
            )
        }
        Val::Ref(k) => {
            let mut val = ctx.eval(pair.second);
            if let Ok(mut o) = Keeper::owner(&k.0) {
                if !matches!(o.tag, InvariantTag::None) {
                    return Val::default();
                }
                swap(&mut o.val, &mut val);
                o.tag = tag;
                val
            } else {
                let _ = Keeper::reinit(&k.0, TaggedVal::new(val));
                Val::default()
            }
        }
        _ => Val::default(),
    }
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
    let input = ctx.eval_escape(input);
    match input {
        Val::Symbol(s) => ctx.set_final(&s),
        Val::Ref(k) => {
            let Ok(mut o) = Keeper::owner(&k.0) else {
                return Val::default();
            };
            if !matches!(o.tag, InvariantTag::None) {
                return Val::default();
            }
            o.tag = InvariantTag::Final;
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
    let input = ctx.eval_escape(input);
    match input {
        Val::Symbol(s) => ctx.set_const(&s),
        Val::Ref(k) => {
            let Ok(mut o) = Keeper::owner(&k.0) else {
                return Val::default();
            };
            o.tag = InvariantTag::Const;
        }
        _ => {}
    }
    Val::default()
}

pub(crate) fn is_final() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::IS_FINAL),
            eval: Reader::new(fn_is_final),
        }),
    })
    .into()
}

fn fn_is_final(ctx: &mut Ctx, input: Val) -> Val {
    let input = ctx.eval_escape(input);
    let is_const = match input {
        Val::Symbol(s) => ctx.is_final(&s),
        Val::Ref(k) => {
            if let Ok(r) = Keeper::reader(&k.0) {
                matches!(&r.tag, InvariantTag::Final | InvariantTag::Const)
            } else {
                false
            }
        }
        _ => {
            return Val::default();
        }
    };
    Val::Bool(Bool::new(is_const))
}

pub(crate) fn is_const() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::IS_CONST),
            eval: Reader::new(fn_is_const),
        }),
    })
    .into()
}

fn fn_is_const(ctx: &mut Ctx, input: Val) -> Val {
    let input = ctx.eval_escape(input);
    let is_const = match input {
        Val::Symbol(s) => ctx.is_const(&s),
        Val::Ref(k) => {
            if let Ok(r) = Keeper::reader(&k.0) {
                matches!(&r.tag, InvariantTag::Const)
            } else {
                false
            }
        }
        _ => {
            return Val::default();
        }
    };
    Val::Bool(Bool::new(is_const))
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
    let input = ctx.eval_escape(input);
    match input {
        Val::Symbol(s) => ctx.remove(&s),
        Val::Ref(k) => {
            let Ok(o) = Keeper::owner(&k.0) else {
                return Val::default();
            };
            if !matches!(o.tag, InvariantTag::None) {
                return Val::default();
            }
            Owner::move_data(o).val
        }
        _ => Val::default(),
    }
}

pub(crate) fn new_ref() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::REF),
            eval: Reader::new(fn_new_ref),
        }),
    })
    .into()
}

fn fn_new_ref(ctx: &mut Ctx, input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new(ctx.eval(input))).into())
}

pub(crate) fn null_ref() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::NULL_REF),
            eval: Reader::new(fn_null_ref),
        }),
    })
    .into()
}

fn fn_null_ref(_: &mut Ctx, _: Val) -> Val {
    let k = Keeper::new(TaggedVal::new(Val::default()));
    let Ok(o) = Keeper::owner(&k) else {
        return Val::default();
    };
    Owner::drop_data(o);
    Val::Ref(k.into())
}

pub(crate) fn final_ref() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::FINAL_REF),
            eval: Reader::new(fn_final_ref),
        }),
    })
    .into()
}

fn fn_final_ref(ctx: &mut Ctx, input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_final(ctx.eval(input))).into())
}

pub(crate) fn const_ref() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::CONST_REF),
            eval: Reader::new(fn_const_ref),
        }),
    })
    .into()
}

fn fn_const_ref(ctx: &mut Ctx, input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_const(ctx.eval(input))).into())
}

pub(crate) fn ctx_new() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::CTX_NEW),
            eval: Reader::new(fn_ctx_new),
        }),
    })
    .into()
}

fn fn_ctx_new(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Map(mut map) = fn_map_new(ctx, input) else {
        return Val::default();
    };

    let Val::Map(constants) = map_remove(&mut map, "const") else {
        return Val::default();
    };
    let Val::Map(finals) = map_remove(&mut map, "final") else {
        return Val::default();
    };
    let Val::Map(variables) = map_remove(&mut map, "var") else {
        return Val::default();
    };

    let mut name_map = NameMap::with_capacity(constants.len() + finals.len() + variables.len());

    for (key, val) in constants {
        let Val::Symbol(Symbol(name)) = key else {
            return Val::default();
        };
        name_map.insert(name, TaggedVal::new_const(val));
    }
    for (key, val) in finals {
        let Val::Symbol(Symbol(name)) = key else {
            return Val::default();
        };
        name_map.insert(name, TaggedVal::new_final(val));
    }
    for (key, val) in variables {
        let Val::Symbol(Symbol(name)) = key else {
            return Val::default();
        };
        name_map.insert(name, TaggedVal::new(val));
    }

    Val::Ctx(Box::new(Ctx {
        name_map,
        super_ctx_name: None,
        reverse_interpreter: None,
    }))
}

fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or(Val::Map(MapVal::default()))
}

pub(crate) fn ctx_set_super() -> Val {
    Box::new(Func {
        func_trait: FuncTrait {},
        func_impl: FuncImpl::Primitive(Primitive {
            id: Name::from(names::CTX_SET_SUPER),
            eval: Reader::new(fn_ctx_set_super),
        }),
    })
    .into()
}

fn fn_ctx_set_super(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_name_or_val = ctx.eval_escape(pair.first);
    let super_ctx_name = ctx.eval_escape(pair.second);
    let f = |ctx: &mut Ctx| {
        match super_ctx_name {
            Val::Symbol(Symbol(name)) => {
                ctx.super_ctx_name = Some(name);
            }
            Val::Unit(_) => {
                ctx.super_ctx_name = None;
            }
            _ => {}
        }
        Val::default()
    };
    if let Val::Unit(_) = &ctx_name_or_val {
        return f(ctx);
    }
    ctx.get_mut_or_val(ctx_name_or_val, |ref_or_val| match ref_or_val {
        Either::Left(r) => {
            let Val::Ctx(ctx) = r else {
                return Val::default();
            };
            f(ctx)
        }
        Either::Right(_) => Val::default(),
    })
}
