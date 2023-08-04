use {
    crate::{
        semantics::{
            eval::{
                ctx::{
                    constant::{
                        ConstCtx,
                        CtxForConstFn,
                    },
                    free::FreeCtx,
                    mutable::CtxForMutableFn,
                    Ctx,
                    CtxTrait,
                    InvariantTag,
                    NameMap,
                    TaggedRef,
                    TaggedVal,
                },
                BasicEvalMode,
                CtxConstFn,
                CtxFreeFn,
                CtxMutableFn,
                EvalMode,
                Primitive,
            },
            prelude::{
                names,
                PrimitiveFunc,
            },
            val::{
                CtxVal,
                MapVal,
                RefVal,
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

pub(crate) fn read() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::READ, fn_read);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_read(mut ctx: CtxForConstFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.get(&s),
        Val::Ref(r) => FreeCtx::get_val_ref(&r),
        Val::Pair(pair) => {
            let Val::Symbol(first) = pair.first else {
                return Val::default();
            };
            let Val::Symbol(second) = pair.second else {
                return Val::default();
            };
            read_pair(&mut ctx, &first, &second)
        }
        Val::List(mut list) => {
            let Some(Val::Symbol(val_name)) = list.pop() else {
                return Val::default();
            };
            read_nested(ctx, &list[..], &val_name)
        }
        _ => Val::default(),
    }
}

fn read_pair(ctx: &mut CtxForConstFn, first: &str, second: &str) -> Val {
    ctx.get_ref(first, |val| {
        let Some(TaggedRef { val_ref, .. }) = val else {
            return Val::default();
        };
        match val_ref {
            Val::Ctx(CtxVal(ctx)) => ctx.get(second),
            Val::Ref(RefVal(k)) => {
                let Ok(mut o) = Keeper::owner(k) else {
                    return Val::default();
                };
                let TaggedVal {
                    val: Val::Ctx(CtxVal(ctx)),
                    ..
                } = &mut *o
                else {
                    return Val::default();
                };
                ctx.get(second)
            }
            _ => Val::default(),
        }
    })
}

fn read_nested<Ctx: CtxTrait>(mut ctx: Ctx, names: &[Val], val_name: &str) -> Val {
    let Some(Val::Symbol(name)) = names.get(0) else {
        return ctx.get(val_name);
    };
    let rest = &names[1..];

    ctx.get_ref(name, |val| {
        let Some(TaggedRef { val_ref, .. }) = val else {
            return Val::default();
        };
        match val_ref {
            Val::Ctx(CtxVal(ctx)) => read_nested(ConstCtx(ctx), rest, val_name),
            Val::Ref(RefVal(k)) => {
                let Ok(mut o) = Keeper::owner(k) else {
                    return Val::default();
                };
                let TaggedVal {
                    val: Val::Ctx(CtxVal(ctx)),
                    ..
                } = &mut *o
                else {
                    return Val::default();
                };
                read_nested(ConstCtx(ctx), rest, val_name)
            }
            _ => Val::default(),
        }
    })
}

pub(crate) fn is_null() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_NULL, fn_is_null);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_null(mut ctx: CtxForConstFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.is_null(&s),
        Val::Ref(k) => Val::Bool(Bool::new(FreeCtx::is_null_ref(&k))),
        _ => Val::default(),
    }
}

pub(crate) fn assign_local() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Pair {
        first: BasicEvalMode::Inline,
        second: BasicEvalMode::Eval,
        non_pair: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN_LOCAL, fn_assign_local);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_assign_local(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let Val::Symbol(name) = pair.first else {
        return Val::default();
    };
    let val = pair.second;
    ctx.put_val_local(name, TaggedVal::new(val))
}

pub(crate) fn assign() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Pair {
        first: BasicEvalMode::Inline,
        second: BasicEvalMode::Eval,
        non_pair: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN, fn_assign);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_assign(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::None)
}

pub(crate) fn assign_final() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Pair {
        first: BasicEvalMode::Inline,
        second: BasicEvalMode::Eval,
        non_pair: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN_FINAL, fn_assign_final);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_assign_final(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Final)
}

pub(crate) fn assign_const() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Pair {
        first: BasicEvalMode::Inline,
        second: BasicEvalMode::Eval,
        non_pair: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN_CONST, fn_assign_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_assign_const(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Const)
}

fn fn_assign_val(mut ctx: CtxForMutableFn, input: Val, tag: InvariantTag) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name = pair.first;
    match name {
        Val::Symbol(s) => {
            let val = pair.second;
            ctx.put_val(s, TaggedVal { tag, val })
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

pub(crate) fn set_final() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::FINAL, fn_set_final);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set_final(mut ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.set_final(&s),
        Val::Ref(k) => FreeCtx::set_final_ref(&k),
        _ => {}
    }
    Val::default()
}

pub(crate) fn set_const() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::CONST, fn_set_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set_const(mut ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.set_const(&s),
        Val::Ref(k) => FreeCtx::set_const_ref(&k),
        _ => {}
    }
    Val::default()
}

pub(crate) fn is_final() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_FINAL, fn_is_final);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_final(mut ctx: CtxForConstFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.is_final(&s),
        Val::Ref(r) => {
            let is_final = FreeCtx::is_final_ref(&r);
            Val::Bool(Bool::new(is_final))
        }
        _ => Val::default(),
    }
}

pub(crate) fn is_const() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_CONST, fn_is_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_const(mut ctx: CtxForConstFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.is_const(&s),
        Val::Ref(r) => {
            let is_const = FreeCtx::is_const_ref(&r);
            Val::Bool(Bool::new(is_const))
        }
        _ => Val::default(),
    }
}

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::MOVE, fn_move);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_move(mut ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.remove(&s),
        Val::Ref(k) => FreeCtx::remove_ref(&k),
        _ => Val::default(),
    }
}

pub(crate) fn new_ref() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::REF, fn_new_ref);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_new_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new(input)).into())
}

pub(crate) fn null_ref() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::NULL_REF, fn_null_ref);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_null_ref(_: Val) -> Val {
    let k = Keeper::new(TaggedVal::new(Val::default()));
    let Ok(o) = Keeper::owner(&k) else {
        return Val::default();
    };
    Owner::drop_data(o);
    Val::Ref(k.into())
}

pub(crate) fn final_ref() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::FINAL_REF, fn_final_ref);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_final_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_final(input)).into())
}

pub(crate) fn const_ref() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::CONST_REF, fn_const_ref);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_const_ref(input: Val) -> Val {
    Val::Ref(Keeper::new(TaggedVal::new_const(input)).into())
}

pub(crate) fn ctx_new() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::Basic(BasicEvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::CTX_NEW, fn_ctx_new);
    PrimitiveFunc::new(eval_mode, primitive)
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
        let Val::Symbol(name) = key else {
            return Val::default();
        };
        name_map.insert(name, TaggedVal::new_const(val));
    }
    for (key, val) in finals {
        let Val::Symbol(name) = key else {
            return Val::default();
        };
        name_map.insert(name, TaggedVal::new_final(val));
    }
    for (key, val) in variables {
        let Val::Symbol(name) = key else {
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

pub(crate) fn ctx_set_super() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::Pair {
        first: BasicEvalMode::Inline,
        second: BasicEvalMode::Inline,
        non_pair: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::CTX_SET_SUPER, fn_ctx_set_super);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_ctx_set_super(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_name_or_val = pair.first;
    let super_ctx = pair.second;
    let super_ctx = match super_ctx {
        Val::Symbol(name) => Some(Either::Left(name)),
        Val::Ref(r) => Some(Either::Right(r)),
        Val::Unit(_) => None,
        _ => {
            return Val::default();
        }
    };
    if let Val::Unit(_) = &ctx_name_or_val {
        ctx.set_super(super_ctx);
        return Val::default();
    }
    ctx.get_ref_or_val_or_default(ctx_name_or_val, |ctx| {
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
