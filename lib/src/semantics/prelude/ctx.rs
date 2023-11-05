use crate::{
    semantics::{
        ctx::{
            Ctx,
            CtxTrait,
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
        eval_mode::EvalMode,
        func::{
            CtxConstFn,
            CtxFreeFn,
            CtxMutableFn,
            Primitive,
        },
        input_mode::InputMode,
        prelude::{
            names,
            prelude,
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
        Call,
        List,
        Map,
        Pair,
        Reverse,
        Symbol,
    },
};

pub(crate) fn load() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxConstFn>::new(names::LOAD, fn_load);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_load(ctx: CtxForConstFn, input: Val) -> Val {
    fn_nested(ctx, input, |ctx, name| ctx.get(&name))
}

pub(crate) fn remove() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxMutableFn>::new(names::MOVE, fn_move);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_move(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_nested(ctx, input, |mut ctx, name| ctx.remove(&name))
}

pub(crate) fn save() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value))),
        InputMode::Any(EvalMode::Eval),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::SAVE, fn_save);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_save(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let path = pair.first;
    let (local, tagged_val) = if let Val::Call(call) = pair.second {
        match parse_ctx_val_pair(call, true) {
            ParseCtxValPairResult::Parsed { val, local, tag } => (local, TaggedVal { val, tag }),
            ParseCtxValPairResult::Fallback(call) => (false, TaggedVal::new(Val::Call(call))),
            ParseCtxValPairResult::None => {
                return Val::default();
            }
        }
    } else {
        (false, TaggedVal::new(pair.second))
    };
    fn_nested(ctx, path, |mut ctx, name| {
        if local {
            ctx.put_val_local(name, tagged_val)
        } else {
            ctx.put_val(name, tagged_val)
        }
    })
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
                ctx.put_val_local(s, tagged_val)
            } else {
                ctx.put_val(s, tagged_val)
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
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxMutableFn>::new(names::SET_FINAL, fn_set_final);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_set_final(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_nested(ctx, input, |mut ctx, name| {
        ctx.set_final(&name);
        Val::default()
    })
}

pub(crate) fn set_const() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxMutableFn>::new(names::SET_CONST, fn_set_const);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_set_const(ctx: CtxForMutableFn, input: Val) -> Val {
    fn_nested(ctx, input, |mut ctx, name| {
        ctx.set_const(&name);
        Val::default()
    })
}

pub(crate) fn is_final() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxConstFn>::new(names::IS_FINAL, fn_is_final);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_final(ctx: CtxForConstFn, input: Val) -> Val {
    fn_nested(ctx, input, |ctx, name| ctx.is_final(&name))
}

pub(crate) fn is_const() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxConstFn>::new(names::IS_CONST, fn_is_const);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_const(ctx: CtxForConstFn, input: Val) -> Val {
    fn_nested(ctx, input, |ctx, name| ctx.is_const(&name))
}

pub(crate) fn is_null() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxConstFn>::new(names::IS_NULL, fn_is_null);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_null(ctx: CtxForConstFn, input: Val) -> Val {
    fn_nested(ctx, input, |ctx, name| ctx.is_null(&name))
}

pub(crate) fn is_local() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxConstFn>::new(names::IS_LOCAL, fn_is_local);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_is_local(ctx: CtxForConstFn, input: Val) -> Val {
    fn_nested(ctx, input, |ctx, name| ctx.is_local(&name))
}

fn fn_nested<Ctx, F>(ctx: Ctx, names: Val, f: F) -> Val
where
    Ctx: CtxTrait,
    F: for<'a> FnOnce(Box<dyn CtxTrait + 'a>, Symbol) -> Val,
{
    match names {
        Val::Symbol(s) => f(Box::new(ctx), s),
        Val::List(mut list) => {
            let Some(Val::Symbol(val_name)) = list.pop() else {
                return Val::default();
            };
            nested(ctx, &list[..], val_name, f)
        }
        _ => Val::default(),
    }
}

fn nested<Ctx, F>(mut ctx: Ctx, names: &[Val], val_name: Symbol, f: F) -> Val
where
    Ctx: CtxTrait,
    F: for<'a> FnOnce(Box<dyn CtxTrait + 'a>, Symbol) -> Val,
{
    let Some(Val::Symbol(name)) = names.first() else {
        return f(Box::new(ctx), val_name);
    };
    let rest = &names[1..];

    let Some(TaggedRef { val_ref, is_const }) = ctx.get_tagged_ref(name) else {
        return Val::default();
    };
    let Val::Ctx(CtxVal(ctx)) = val_ref else {
        return Val::default();
    };
    if is_const {
        nested(ConstCtx(ctx), rest, val_name, f)
    } else {
        nested(MutableCtx(ctx), rest, val_name, f)
    }
}

const MAP: &str = "map";
const SUPER: &str = "super";

const CTX_VALUE_PAIR: &str = ":";
const VARIABLE: &str = "variable";
const FINAL: &str = "final";
const CONST: &str = "constant";

pub(crate) fn ctx_new() -> PrimitiveFunc<CtxFreeFn> {
    let mut map = Map::default();
    map.insert(
        symbol(MAP),
        InputMode::MapForAll(Box::new(Pair::new(
            InputMode::Any(EvalMode::Quote),
            InputMode::Any(EvalMode::Eval),
        ))),
    );
    map.insert(symbol(SUPER), InputMode::Symbol(EvalMode::Value));
    let input_mode = InputMode::MapForSome(map);
    let primitive = Primitive::<CtxFreeFn>::new(names::CTX_NEW, fn_ctx_new);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };

    let name_map_repr = match map_remove(&mut map, MAP) {
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

    let super_ctx = match map_remove(&mut map, SUPER) {
        Val::Symbol(s) => Some(s),
        _ => None,
    };

    Val::Ctx(CtxVal(Box::new(Ctx {
        name_map,
        super_ctx,
    })))
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
    let mut repr = MapVal::default();

    if let Some(name) = ctx.super_ctx {
        repr.insert(symbol(SUPER), Val::Symbol(name));
    }

    if !ctx.name_map.is_empty() {
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
        repr.insert(symbol(MAP), Val::Map(map));
    }

    Val::Map(repr)
}

pub(crate) fn ctx_set_super() -> PrimitiveFunc<CtxMutableFn> {
    let input_mode = InputMode::Pair(Box::new(Pair::new(
        InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value))),
        InputMode::Symbol(EvalMode::Value),
    )));
    let primitive = Primitive::<CtxMutableFn>::new(names::CTX_SET_SUPER, fn_ctx_set_super);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_set_super(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let mut path = pair.first;
    let super_ctx = pair.second;
    let super_ctx = match super_ctx {
        Val::Symbol(name) => Some(name),
        Val::Unit(_) => None,
        _ => {
            return Val::default();
        }
    };
    match path {
        Val::Symbol(name) => {
            let Some(TaggedRef {
                val_ref: Val::Ctx(CtxVal(ctx)),
                is_const: false,
            }) = ctx.get_tagged_ref(&name)
            else {
                return Val::default();
            };
            ctx.super_ctx = super_ctx;
            Val::default()
        }
        Val::List(mut list) => {
            if list.is_empty() {
                ctx.set_super(super_ctx);
                return Val::default();
            }
            let Val::Symbol(val_name) = list.pop().unwrap() else {
                return Val::default();
            };
            nested(ctx, &list[..], val_name, |mut ctx, name| {
                let Some(TaggedRef {
                    val_ref: Val::Ctx(CtxVal(ctx)),
                    is_const: false,
                }) = ctx.get_tagged_ref(&name)
                else {
                    return Val::default();
                };
                ctx.super_ctx = super_ctx;
                Val::default()
            })
        }
        Val::Ctx(CtxVal(ref mut ctx)) => {
            ctx.super_ctx = super_ctx;
            path
        }
        _ => Val::default(),
    }
}

pub(crate) fn ctx_get_super() -> PrimitiveFunc<CtxConstFn> {
    let input_mode = InputMode::ListForAll(Box::new(InputMode::Symbol(EvalMode::Value)));
    let primitive = Primitive::<CtxConstFn>::new(names::CTX_GET_SUPER, fn_ctx_get_super);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_get_super(ctx: CtxForConstFn, input: Val) -> Val {
    match input {
        Val::Symbol(name) => {
            let Some(Val::Ctx(CtxVal(ctx))) = ctx.get_const_ref(&name) else {
                return Val::default();
            };
            let Some(super_name) = &ctx.super_ctx else {
                return Val::default();
            };
            Val::Symbol(super_name.clone())
        }
        Val::List(mut list) => {
            if list.is_empty() {
                let Some(super_name) = ctx.get_super() else {
                    return Val::default();
                };
                return Val::Symbol(super_name);
            }
            let Val::Symbol(val_name) = list.pop().unwrap() else {
                return Val::default();
            };
            nested(ctx, &list[..], val_name, |ctx, name| {
                let Some(Val::Ctx(CtxVal(ctx))) = ctx.get_const_ref(&name) else {
                    return Val::default();
                };
                let Some(super_name) = &ctx.super_ctx else {
                    return Val::default();
                };
                Val::Symbol(super_name.clone())
            })
        }
        Val::Ctx(CtxVal(ref ctx)) => {
            let super_ctx = match &ctx.super_ctx {
                Some(name) => Val::Symbol(name.clone()),
                None => Val::default(),
            };
            Val::Pair(Box::new(Pair::new(input, super_ctx)))
        }
        _ => Val::default(),
    }
}

pub(crate) fn ctx_prelude() -> PrimitiveFunc<CtxFreeFn> {
    let input_mode = InputMode::Any(EvalMode::Value);
    let primitive = Primitive::<CtxFreeFn>::new(names::CTX_PRELUDE, fn_ctx_prelude);
    PrimitiveFunc::new(input_mode, primitive)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    let name_map = prelude();
    Val::Ctx(CtxVal(Box::new(Ctx {
        name_map,
        super_ctx: None,
    })))
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
