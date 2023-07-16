use {
    crate::{
        semantics::{
            eval::{
                ctx::{
                    Ctx,
                    InvariantTag,
                    NameMap,
                    TaggedRef,
                    TaggedVal,
                },
                ctx_free::CtxFree,
                BasicEvalMode,
                EvalMode,
                Func,
                IsConst,
                Primitive,
            },
            prelude::{
                names,
                prelude_func,
            },
            val::{
                CtxVal,
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
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_read,
    )))
}

fn fn_read(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.get(&s),
        Val::Ref(r) => CtxFree::get(&r),
        _ => Val::default(),
    }
}

pub(crate) fn is_null() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::IS_NULL,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_is_null,
    )))
}

fn fn_is_null(ctx: &mut Ctx, input: Val) -> Val {
    match input {
        Val::Symbol(s) => Val::Bool(Bool::new(ctx.is_null(&s))),
        Val::Ref(k) => Val::Bool(Bool::new(CtxFree::is_null(&k))),
        _ => Val::default(),
    }
}

pub(crate) fn assign_local() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::ASSIGN_LOCAL,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Eval,
            non_pair: BasicEvalMode::Value,
        },
        fn_assign_local,
    )))
}

fn fn_assign_local(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    if is_const {
        return Val::default();
    }
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(Symbol(name)) = pair.first else {
        return Val::default();
    };
    let val = pair.second;
    ctx.put_val_local(name, TaggedVal::new(val))
}

pub(crate) fn assign() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::ASSIGN,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Eval,
            non_pair: BasicEvalMode::Value,
        },
        fn_assign,
    )))
}

fn fn_assign(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    fn_assign_val(ctx, is_const, input, InvariantTag::None)
}

pub(crate) fn assign_final() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::ASSIGN_FINAL,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Eval,
            non_pair: BasicEvalMode::Value,
        },
        fn_assign_final,
    )))
}

fn fn_assign_final(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    fn_assign_val(ctx, is_const, input, InvariantTag::Final)
}

pub(crate) fn assign_const() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::ASSIGN_CONST,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Eval,
            non_pair: BasicEvalMode::Value,
        },
        fn_assign_const,
    )))
}

fn fn_assign_const(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    fn_assign_val(ctx, is_const, input, InvariantTag::Const)
}

fn fn_assign_val(ctx: &mut Ctx, is_const: IsConst, input: Val, tag: InvariantTag) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name = pair.first;
    match name {
        Val::Symbol(s) => {
            let val = pair.second;
            ctx.put_val(is_const, s.0, TaggedVal { tag, val })
        }
        Val::Ref(k) => {
            let mut val = pair.second;
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::FINAL,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_set_final,
    )))
}

fn fn_set_final(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.set_final(is_const, &s),
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
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::CONST,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_set_const,
    )))
}

fn fn_set_const(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.set_const(is_const, &s),
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
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_is_final,
    )))
}

fn fn_is_final(ctx: &mut Ctx, input: Val) -> Val {
    let is_const = match input {
        Val::Symbol(s) => ctx.is_final(&s),
        Val::Ref(r) => CtxFree::is_final(&r),
        _ => {
            return Val::default();
        }
    };
    Val::Bool(Bool::new(is_const))
}

pub(crate) fn is_const() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_const(
        names::IS_CONST,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_is_const,
    )))
}

fn fn_is_const(ctx: &mut Ctx, input: Val) -> Val {
    let is_const = match input {
        Val::Symbol(s) => ctx.is_const(&s),
        Val::Ref(r) => CtxFree::is_const(&r),
        _ => {
            return Val::default();
        }
    };
    Val::Bool(Bool::new(is_const))
}

pub(crate) fn remove() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::MOVE,
        EvalMode::Basic(BasicEvalMode::Inline),
        fn_move,
    )))
}

fn fn_move(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.remove(is_const, &s),
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
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_new_ref,
    )))
}

fn fn_new_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new(input)).into())
}

pub(crate) fn null_ref() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::NULL_REF,
        EvalMode::Basic(BasicEvalMode::Value),
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
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_final_ref,
    )))
}

fn fn_final_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_final(input)).into())
}

pub(crate) fn const_ref() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::CONST_REF,
        EvalMode::Basic(BasicEvalMode::Eval),
        fn_const_ref,
    )))
}

fn fn_const_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_const(input)).into())
}

pub(crate) fn ctx_new() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_free(
        names::CTX_NEW,
        EvalMode::Basic(BasicEvalMode::Eval),
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
        })
        .into(),
    )
}

fn map_remove(map: &mut MapVal, name: &str) -> Val {
    let name = Val::Symbol(Symbol::from_str(name));
    map.remove(&name).unwrap_or(Val::Map(MapVal::default()))
}

pub(crate) fn ctx_set_super() -> Val {
    prelude_func(Func::new_primitive(Primitive::new_ctx_mutable(
        names::CTX_SET_SUPER,
        EvalMode::Pair {
            first: BasicEvalMode::Inline,
            second: BasicEvalMode::Inline,
            non_pair: BasicEvalMode::Value,
        },
        fn_ctx_set_super,
    )))
}

fn fn_ctx_set_super(ctx: &mut Ctx, is_const: IsConst, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_name_or_val = pair.first;
    let super_ctx = pair.second;
    let super_ctx = match super_ctx {
        Val::Symbol(Symbol(name)) => Some(Either::Left(name)),
        Val::Ref(r) => Some(Either::Right(r)),
        Val::Unit(_) => None,
        _ => {
            return Val::default();
        }
    };
    if let Val::Unit(_) = &ctx_name_or_val {
        ctx.super_ctx = super_ctx;
        return Val::default();
    }
    ctx.get_ref_or_val_or_default(is_const, ctx_name_or_val, |ctx| {
        let Either::Left(TaggedRef {
            val_ref: Val::Ctx(CtxVal(ctx)),
            is_const: false,
        }) = ctx
        else {
            return Val::default();
        };
        ctx.super_ctx = super_ctx;
        Val::default()
    })
}
