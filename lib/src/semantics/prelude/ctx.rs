use crate::{
    semantics::{
        ctx::{
            Ctx,
            CtxError,
            CtxTrait,
            InvariantTag,
            NameMap,
            TaggedVal,
        },
        ctx_access::{
            constant::CtxForConstFn,
            mutable::CtxForMutableFn,
        },
        eval_mode::EvalMode,
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            initial_ctx,
            names,
            utils::{
                map_remove,
                symbol,
            },
            PrimitiveFunc,
        },
        val::{
            CallVal,
            CtxVal,
            MapVal,
            Val,
        },
    },
    types::{
        Bool,
        Call,
        List,
        Pair,
        Reverse,
        Symbol,
        Unit,
    },
};

pub(crate) fn read() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::READ, fn_read);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_read(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.get(&s).unwrap_or_default()
}

pub(crate) fn move1() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::MOVE, fn_move);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_move(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.remove(&s).unwrap_or_default()
}

pub(crate) fn assign() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Quote),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::ASSIGN, fn_assign);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_assign(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name = pair.first;
    assign_destruct(&mut ctx, name, pair.second, false, InvariantTag::None)
}

const LOCAL: &str = "local";
const TAG: &str = "tag";

fn assign_destruct(
    ctx: &mut CtxForMutableFn,
    name: Val,
    val: Val,
    local: bool,
    tag: InvariantTag,
) -> Val {
    match name {
        Val::Symbol(s) => {
            let tagged_val = TaggedVal { val, tag };
            if local {
                let Ok(Some(last)) = ctx.put_val_local(s, tagged_val) else {
                    return Val::default();
                };
                last
            } else {
                let Ok(Some(last)) = ctx.put_val(s, tagged_val) else {
                    return Val::default();
                };
                last
            }
        }
        Val::Pair(name_pair) => {
            let Val::Pair(val_pair) = val else {
                return Val::default();
            };
            let last_first = assign_destruct(ctx, name_pair.first, val_pair.first, local, tag);
            let last_second = assign_destruct(ctx, name_pair.second, val_pair.second, local, tag);
            Val::Pair(Box::new(Pair::new(last_first, last_second)))
        }
        Val::Call(name_call) => match parse_ctx_val_pair(name_call, true) {
            ParseCtxValPairResult::Parsed {
                val: name,
                local,
                tag,
            } => assign_destruct(ctx, name, val, local, tag),
            ParseCtxValPairResult::Fallback(name_call) => {
                let Val::Call(val_call) = val else {
                    return Val::default();
                };
                let last_func = assign_destruct(ctx, name_call.func, val_call.func, local, tag);
                let last_input = assign_destruct(ctx, name_call.input, val_call.input, local, tag);
                Val::Call(Box::new(Call::new(last_func, last_input)))
            }
            ParseCtxValPairResult::None => Val::default(),
        },
        Val::Reverse(name_reverse) => {
            let Val::Reverse(val_reverse) = val else {
                return Val::default();
            };
            let last_func = assign_destruct(ctx, name_reverse.func, val_reverse.func, local, tag);
            let last_output =
                assign_destruct(ctx, name_reverse.output, val_reverse.output, local, tag);
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
                last_list.push(assign_destruct(ctx, name, val, local, tag));
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
                    let last_val = assign_destruct(ctx, name, v, local, tag);
                    Some((k, last_val))
                })
                .collect();
            Val::Map(last_map)
        }
        _ => Val::default(),
    }
}

enum ParseCtxValPairResult {
    Parsed {
        val: Val,
        local: bool,
        tag: InvariantTag,
    },
    Fallback(Box<CallVal>),
    None,
}

fn parse_ctx_val_pair(call: Box<CallVal>, accept_local: bool) -> ParseCtxValPairResult {
    let Val::Symbol(tag) = &call.func else {
        return ParseCtxValPairResult::Fallback(call);
    };
    if &**tag != CTX_VALUE_PAIR {
        return ParseCtxValPairResult::Fallback(call);
    }
    let Val::Pair(pair) = call.input else {
        return ParseCtxValPairResult::None;
    };
    let val = pair.first;
    let (local, tag) = match pair.second {
        Val::Symbol(s) => {
            if let Some(tag) = parse_invariant_tag(&s) {
                (false, tag)
            } else if accept_local && &*s == LOCAL {
                (true, InvariantTag::None)
            } else {
                return ParseCtxValPairResult::None;
            }
        }
        Val::Map(mut map) => {
            let tag = match map_remove(&mut map, TAG) {
                Val::Symbol(tag) => {
                    if let Some(tag) = parse_invariant_tag(&tag) {
                        tag
                    } else {
                        return ParseCtxValPairResult::None;
                    }
                }
                Val::Unit(_) => InvariantTag::None,
                _ => return ParseCtxValPairResult::None,
            };
            let local = map.contains_key(&symbol(LOCAL));
            (local, tag)
        }
        _ => return ParseCtxValPairResult::None,
    };
    ParseCtxValPairResult::Parsed { val, local, tag }
}

fn parse_invariant_tag(tag: &str) -> Option<InvariantTag> {
    let tag = match tag {
        VARIABLE => InvariantTag::None,
        FINAL => InvariantTag::Final,
        CONST => InvariantTag::Const,
        _ => return None,
    };
    Some(tag)
}

fn generate_invariant_tag(tag: InvariantTag) -> Symbol {
    let tag = match tag {
        InvariantTag::None => VARIABLE,
        InvariantTag::Final => FINAL,
        InvariantTag::Const => CONST,
    };
    Symbol::from_str(tag)
}

pub(crate) fn set_final() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::SET_FINAL, fn_set_final);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_set_final(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_final(&s);
    Val::default()
}

pub(crate) fn set_const() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxMutableFn>::new(names::SET_CONST, fn_set_const);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_set_const(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_const(&s);
    Val::default()
}

pub(crate) fn is_final() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_FINAL, fn_is_final);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_final(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match ctx.is_final(&s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

pub(crate) fn is_const() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_CONST, fn_is_const);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_const(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match ctx.is_const(&s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

pub(crate) fn is_null() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_NULL, fn_is_null);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_null(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match ctx.is_null(&s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(_) => Val::default(),
    }
}

pub(crate) fn is_local() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Symbol(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::IS_LOCAL, fn_is_local);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_local(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match ctx.is_local(&s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(_) => Val::default(),
    }
}

pub(crate) fn has_meta() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::HAS_META, fn_has_meta);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_has_meta(ctx: CtxForConstFn, _input: Val) -> Val {
    match ctx.get_meta() {
        Ok(_) => Val::Bool(Bool::t()),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

pub(crate) fn set_meta() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Any(EvalMode::Eval);
    let primitive = Primitive::<CtxMutableFn>::new(names::SET_META, fn_set_meta);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_set_meta(mut ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Unit(_) => {
            let _ = ctx.set_meta(None);
        }
        Val::Ctx(meta) => {
            let _ = ctx.set_meta(Some(*meta.0));
        }
        _ => {}
    }
    Val::default()
}

const CTX_VALUE_PAIR: &str = ":";
const VARIABLE: &str = "variable";
const FINAL: &str = "final";
const CONST: &str = "constant";

pub(crate) fn ctx_new() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::Any(EvalMode::Eval),
        InputMode::MapForAll(Box::new(Pair::new(
            InputMode::Any(EvalMode::Quote),
            InputMode::Any(EvalMode::Eval),
        ))),
    )));
    let primitive = Primitive::<CtxFreeFn>::new(names::CTX_NEW, fn_ctx_new);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_new(input: Val) -> Val {
    let Val::Pair(meta_map) = input else {
        return Val::default();
    };

    let meta = match meta_map.first {
        Val::Unit(_) => None,
        Val::Ctx(meta) => Some(meta.0),
        _ => return Val::default(),
    };

    let name_map_repr = match meta_map.second {
        Val::Map(name_map) => name_map,
        Val::Unit(_) => MapVal::default(),
        _ => return Val::default(),
    };

    let mut name_map = NameMap::with_capacity(name_map_repr.len());

    for (key, val) in name_map_repr {
        let Val::Symbol(name) = key else {
            return Val::default();
        };
        let tagged_val = {
            if let Val::Call(call) = val {
                match parse_ctx_val_pair(call, false) {
                    ParseCtxValPairResult::Parsed { val, tag, .. } => TaggedVal { val, tag },
                    ParseCtxValPairResult::Fallback(call) => TaggedVal::new(Val::Call(call)),
                    ParseCtxValPairResult::None => {
                        return Val::default();
                    }
                }
            } else {
                TaggedVal::new(val)
            }
        };
        name_map.insert(name, tagged_val);
    }

    Val::Ctx(CtxVal(Box::new(Ctx { name_map, meta })))
}

pub(crate) fn ctx_repr() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Eval);
    let primitive = Primitive::<CtxFreeFn>::new(names::CTX_REPR, fn_ctx_repr);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_repr(input: Val) -> Val {
    let Val::Ctx(CtxVal(ctx)) = input else {
        return Val::default();
    };

    let meta = match ctx.meta {
        Some(meta) => Val::Ctx(CtxVal(meta)),
        None => Val::Unit(Unit),
    };

    let map = ctx
        .name_map
        .into_iter()
        .map(|(k, v)| {
            let k = Val::Symbol(k);
            let use_normal_form = if let Val::Call(call) = &v.val
                && let Val::Symbol(func) = &call.func
                && &**func == CTX_VALUE_PAIR
            {
                true
            } else {
                matches!(v.tag, InvariantTag::Final | InvariantTag::Const)
            };
            let v = if use_normal_form {
                let func = symbol(CTX_VALUE_PAIR);
                let tag = generate_invariant_tag(v.tag);
                let pair = Val::Pair(Box::new(Pair::new(v.val, Val::Symbol(tag))));
                Val::Call(Box::new(Call::new(func, pair)))
            } else {
                v.val
            };
            (k, v)
        })
        .collect();
    let map = Val::Map(map);
    Val::Pair(Box::new(Pair::new(meta, map)))
}

pub(crate) fn ctx_prelude() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::CTX_PRELUDE, fn_ctx_prelude);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal(Box::new(initial_ctx())))
}

pub(crate) fn ctx_current() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxConstFn>::new(names::CTX_CURRENT, fn_ctx_current);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_current(ctx: CtxForConstFn, _input: Val) -> Val {
    let CtxForConstFn::Const(ctx) = ctx else {
        return Val::default();
    };
    Val::Ctx(CtxVal(Box::new(ctx.0.clone())))
}
