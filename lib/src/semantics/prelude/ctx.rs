use crate::{
    semantics::{
        ctx::{
            Ctx,
            CtxTrait,
            DefaultCtx,
            InvariantTag,
            NameMap,
            TaggedRef,
            TaggedVal,
        },
        ctx_access::{
            constant::{
                ConstCtx,
                CtxForConstFn,
            },
            mutable::{
                CtxForMutableFn,
                MutableCtx,
            },
        },
        eval_mode::{
            BasicEvalMode,
            EvalMode,
        },
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            Primitive,
        },
        prelude::{
            names,
            PrimitiveFunc,
        },
        val::{
            CtxVal,
            MapVal,
            Val,
        },
    },
    types::{
        Call,
        List,
        Pair,
        Reverse,
        Symbol,
    },
};

pub(crate) fn read() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::READ, fn_read);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_read(mut ctx: CtxForConstFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.get(&s),
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
    let Some(Val::Ctx(CtxVal(ctx))) = ctx.get_const_ref(first) else {
        return Val::default();
    };
    ctx.get(second)
}

fn read_nested<Ctx: CtxTrait>(mut ctx: Ctx, names: &[Val], val_name: &str) -> Val {
    let Some(Val::Symbol(name)) = names.get(0) else {
        return ctx.get(val_name);
    };
    let rest = &names[1..];

    let Some(TaggedRef { val_ref, .. }) = ctx.get_tagged_ref(name) else {
        return Val::default();
    };
    let Val::Ctx(CtxVal(ctx)) = val_ref else {
        return Val::default();
    };
    read_nested(ConstCtx(ctx), rest, val_name)
}

pub(crate) fn is_null() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_NULL, fn_is_null);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_null(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.is_null(&s)
}

pub(crate) fn assign_local() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
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
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN, fn_assign);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_assign(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::None)
}

pub(crate) fn assign_final() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
    };
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN_FINAL, fn_assign_final);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_assign_final(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_assign_val(ctx, input, InvariantTag::Final)
}

pub(crate) fn assign_const() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Eval)),
        default: BasicEvalMode::Value,
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
    assign_recursive(&mut ctx, name, pair.second, tag)
}

fn assign_recursive(ctx: &mut CtxForMutableFn, name: Val, val: Val, tag: InvariantTag) -> Val {
    match name {
        Val::Symbol(s) => ctx.put_val(s, TaggedVal { tag, val }),
        Val::Pair(name_pair) => {
            let Val::Pair(val_pair) = val else {
                return Val::default();
            };
            let last_first = assign_recursive(ctx, name_pair.first, val_pair.first, tag);
            let last_second = assign_recursive(ctx, name_pair.second, val_pair.second, tag);
            Val::Pair(Box::new(Pair::new(last_first, last_second)))
        }
        Val::Call(name_call) => {
            let Val::Call(val_call) = val else {
                return Val::default();
            };
            let last_func = assign_recursive(ctx, name_call.func, val_call.func, tag);
            let last_input = assign_recursive(ctx, name_call.input, val_call.input, tag);
            Val::Call(Box::new(Call::new(last_func, last_input)))
        }
        Val::Reverse(name_reverse) => {
            let Val::Reverse(val_reverse) = val else {
                return Val::default();
            };
            let last_func = assign_recursive(ctx, name_reverse.func, val_reverse.func, tag);
            let last_output = assign_recursive(ctx, name_reverse.output, val_reverse.output, tag);
            Val::Reverse(Box::new(Reverse::new(last_func, last_output)))
        }
        Val::List(name_list) => {
            let Val::List(val_list) = val else {
                return Val::default();
            };
            let mut last_list = List::default();
            let mut name_iter = name_list.into_iter();
            let mut val_iter = val_list.into_iter();
            while let (Some(name), Some(val)) = (name_iter.next(), val_iter.next()) {
                if let Val::Symbol(s) = &name {
                    if &**s == "..." {
                        let name_len = name_iter.len();
                        let val_len = val_iter.len();
                        if val_len > name_len {
                            val_iter.advance_by(val_len - name_len).unwrap();
                        }
                        last_list.push(Val::default());
                        continue;
                    }
                }
                last_list.push(assign_recursive(ctx, name, val, tag));
            }
            Val::List(last_list)
        }
        Val::Map(mut name_map) => {
            let Val::Map(val_map) = val else {
                return Val::default();
            };
            let last_map = val_map
                .into_iter()
                .filter_map(|(k, v)| {
                    let name = name_map.remove(&k)?;
                    let last_val = assign_recursive(ctx, name, v, tag);
                    Some((k, last_val))
                })
                .collect();
            Val::Map(last_map)
        }
        _ => Val::default(),
    }
}

pub(crate) fn set_final() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::FINAL, fn_set_final);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set_final(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.set_final(&s);
    Val::default()
}

pub(crate) fn set_const() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::CONST, fn_set_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_set_const(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.set_const(&s);
    Val::default()
}

pub(crate) fn is_final() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_FINAL, fn_is_final);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_final(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.is_final(&s)
}

pub(crate) fn is_const() -> PrimitiveFunc<CtxConstFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_CONST, fn_is_const);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_is_const(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.is_const(&s)
}

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Inline);
    let primitive = Primitive::<CtxMutableFn>::new(names::MOVE, fn_move);
    PrimitiveFunc::new(eval_mode, primitive)
}

fn fn_move(mut ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Symbol(s) => ctx.remove(&s),
        Val::Pair(pair) => {
            let Val::Symbol(first) = pair.first else {
                return Val::default();
            };
            let Val::Symbol(second) = pair.second else {
                return Val::default();
            };
            remove_pair(&mut ctx, &first, &second)
        }
        Val::List(mut list) => {
            let Some(Val::Symbol(val_name)) = list.pop() else {
                return Val::default();
            };
            remove_nested(ctx, &list[..], &val_name)
        }
        _ => Val::default(),
    }
}

fn remove_pair(ctx: &mut CtxForMutableFn, first: &str, second: &str) -> Val {
    let Some(TaggedRef { val_ref, is_const }) = ctx.get_tagged_ref(first) else {
        return Val::default();
    };
    let Val::Ctx(CtxVal(ctx)) = val_ref else {
        return Val::default();
    };
    if is_const {
        return Val::default();
    }
    ctx.remove(second)
}

fn remove_nested<Ctx: CtxTrait>(mut ctx: Ctx, names: &[Val], val_name: &str) -> Val {
    let Some(Val::Symbol(name)) = names.get(0) else {
        return ctx.remove(val_name);
    };
    let rest = &names[1..];

    let Some(TaggedRef { val_ref, is_const }) = ctx.get_tagged_ref(name) else {
        return Val::default();
    };
    let Val::Ctx(CtxVal(ctx)) = val_ref else {
        return Val::default();
    };
    if is_const {
        remove_nested(ConstCtx(ctx), rest, val_name)
    } else {
        remove_nested(MutableCtx(ctx), rest, val_name)
    }
}

pub(crate) fn ctx_new() -> PrimitiveFunc<CtxFreeFn> {
    let eval_mode = EvalMode::basic(BasicEvalMode::Eval);
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
    let eval_mode = EvalMode {
        pair: Some((BasicEvalMode::Inline, BasicEvalMode::Inline)),
        default: BasicEvalMode::Value,
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
        Val::Symbol(name) => Some(name),
        Val::Unit(_) => None,
        _ => {
            return Val::default();
        }
    };
    if let Val::Unit(_) = &ctx_name_or_val {
        ctx.set_super(super_ctx);
        return Val::default();
    }
    DefaultCtx.get_mut_ref_no_ret(&mut ctx, ctx_name_or_val, |ctx| {
        let Val::Ctx(CtxVal(ctx)) = ctx else {
            return;
        };
        ctx.super_ctx = super_ctx;
    })
}
