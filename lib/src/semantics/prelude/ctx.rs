use {
    crate::{
        semantics::{
            eval::{
                strategy::{
                    eval::{
                        DefaultStrategy,
                        Eval,
                    },
                    inline::InlineStrategy,
                    EvalStrategy,
                },
                Ctx,
                EvalMode,
                Func,
                InvariantTag,
                Name,
                NameMap,
                Primitive,
                TaggedVal,
            },
            prelude::{
                names,
                prelude_func,
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
            Symbol,
        },
    },
    std::mem::swap,
};

pub(crate) fn read() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::READ,
        EvalMode::Inline,
        fn_read,
    )))
}

fn fn_read(ctx: &Ctx, input: Val) -> Val {
    match input {
        Val::Symbol(s) => Eval::eval_symbol(ctx, &s),
        Val::Ref(r) => Eval::eval_ref(&r),
        _ => Val::default(),
    }
}

pub(crate) fn is_null() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::IS_NULL,
        EvalMode::Inline,
        fn_is_null,
    )))
}

fn fn_is_null(ctx: &Ctx, input: Val) -> Val {
    match input {
        Val::Symbol(s) => Val::Bool(Bool::new(ctx.get_ref(&s, |op| op.is_none()))),
        Val::Ref(k) => Val::Bool(Bool::new(Keeper::reader(&k.0).is_err())),
        _ => Val::default(),
    }
}

pub(crate) fn assign_local() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::ASSIGN_LOCAL,
        EvalMode::Value,
        fn_assign_local,
    )))
}

fn fn_assign_local(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(Symbol(name)) = InlineStrategy::eval(ctx, pair.first) else {
        return Val::default();
    };
    let val = DefaultStrategy::eval(ctx, pair.second);
    ctx.put_val_local(name, TaggedVal::new(val))
}

pub(crate) fn assign() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::ASSIGN,
        EvalMode::Value,
        fn_assign,
    )))
}

fn fn_assign(ctx: &mut Ctx, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::None)
}

pub(crate) fn assign_final() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::ASSIGN_FINAL,
        EvalMode::Value,
        fn_assign_final,
    )))
}

fn fn_assign_final(ctx: &mut Ctx, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Final)
}

pub(crate) fn assign_const() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::ASSIGN_CONST,
        EvalMode::Value,
        fn_assign_const,
    )))
}

fn fn_assign_const(ctx: &mut Ctx, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Const)
}

fn fn_assign_val(ctx: &mut Ctx, input: Val, tag: InvariantTag) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let first = InlineStrategy::eval(ctx, pair.first);
    match first {
        Val::Symbol(s) => {
            let val = DefaultStrategy::eval(ctx, pair.second);
            ctx.put_val(
                Name::from(<_ as Into<String>>::into(s)),
                TaggedVal { tag, val },
            )
        }
        Val::Ref(k) => {
            let mut val = DefaultStrategy::eval(ctx, pair.second);
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::FINAL,
        EvalMode::Inline,
        fn_set_final,
    )))
}

fn fn_set_final(ctx: &mut Ctx, input: Val) -> Val {
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::CONST,
        EvalMode::Inline,
        fn_set_const,
    )))
}

fn fn_set_const(ctx: &mut Ctx, input: Val) -> Val {
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::IS_FINAL,
        EvalMode::Inline,
        fn_is_final,
    )))
}

fn fn_is_final(ctx: &Ctx, input: Val) -> Val {
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::IS_CONST,
        EvalMode::Inline,
        fn_is_const,
    )))
}

fn fn_is_const(ctx: &Ctx, input: Val) -> Val {
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::MOVE,
        EvalMode::Inline,
        fn_move,
    )))
}

fn fn_move(ctx: &mut Ctx, input: Val) -> Val {
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::REF,
        EvalMode::Eval,
        fn_new_ref,
    )))
}

fn fn_new_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new(input)).into())
}

pub(crate) fn null_ref() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::NULL_REF,
        EvalMode::Value,
        fn_null_ref,
    )))
}

fn fn_null_ref(_: Val) -> Val {
    let k = Keeper::new(TaggedVal::new(Val::default()));
    let Ok(o) = Keeper::owner(&k) else {
        return Val::default();
    };
    Owner::drop_data(o);
    Val::Ref(k.into())
}

pub(crate) fn final_ref() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::FINAL_REF,
        EvalMode::Eval,
        fn_final_ref,
    )))
}

fn fn_final_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_final(input)).into())
}

pub(crate) fn const_ref() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::CONST_REF,
        EvalMode::Eval,
        fn_const_ref,
    )))
}

fn fn_const_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_const(input)).into())
}

pub(crate) fn ctx_new() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::CTX_NEW,
        EvalMode::Eval,
        fn_ctx_new,
    )))
}

fn fn_ctx_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
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

    Val::Ctx(
        Box::new(Ctx {
            name_map,
            super_ctx: None,
            reverse_interpreter: None,
        })
        .into(),
    )
}

fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or(Val::Map(MapVal::default()))
}

pub(crate) fn ctx_set_super() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_aware(
        names::CTX_SET_SUPER,
        EvalMode::Inline,
        fn_ctx_set_super,
    )))
}

fn fn_ctx_set_super(ctx: &mut Ctx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_name_or_val = pair.first;
    let super_ctx = pair.second;
    let f = |ctx: &mut Ctx| {
        match super_ctx {
            Val::Symbol(Symbol(name)) => {
                ctx.super_ctx = Some(Either::Left(name));
            }
            Val::Ref(r) => {
                ctx.super_ctx = Some(Either::Right(r));
            }
            Val::Unit(_) => {
                ctx.super_ctx = None;
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
            f(&mut ctx.0)
        }
        Either::Right(_) => Val::default(),
    })
}
