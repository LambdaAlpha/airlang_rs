use crate::{
    bool::Bool,
    call::Call,
    ctx::{
        Ctx,
        CtxError,
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
        free::FreeCtx,
        mutable::{
            CtxForMutableFn,
            MutableCtx,
        },
    },
    eval::{
        output::OutputBuilder,
        Evaluator,
        ValBuilder,
    },
    eval_mode::EvalMode,
    io_mode::IoMode,
    list::List,
    pair::Pair,
    prelude::{
        initial_ctx,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        utils::{
            map_remove,
            symbol,
        },
        Named,
        Prelude,
    },
    problem::solve,
    reverse::Reverse,
    symbol::Symbol,
    unit::Unit,
    val::{
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
        map::MapVal,
        Val,
    },
};

#[derive(Clone)]
pub(crate) struct CtxPrelude {
    pub(crate) read: Named<FuncVal>,
    pub(crate) move1: Named<FuncVal>,
    pub(crate) assign: Named<FuncVal>,
    pub(crate) set_final: Named<FuncVal>,
    pub(crate) set_const: Named<FuncVal>,
    pub(crate) is_final: Named<FuncVal>,
    pub(crate) is_const: Named<FuncVal>,
    pub(crate) is_null: Named<FuncVal>,
    pub(crate) is_local: Named<FuncVal>,
    pub(crate) get_access: Named<FuncVal>,
    pub(crate) has_meta: Named<FuncVal>,
    pub(crate) set_meta: Named<FuncVal>,
    pub(crate) with_ctx: Named<FuncVal>,
    pub(crate) ctx_new: Named<FuncVal>,
    pub(crate) ctx_repr: Named<FuncVal>,
    pub(crate) ctx_prelude: Named<FuncVal>,
    pub(crate) ctx_this: Named<FuncVal>,
}

impl Default for CtxPrelude {
    fn default() -> Self {
        CtxPrelude {
            read: read(),
            move1: move1(),
            assign: assign(),
            set_final: set_final(),
            set_const: set_const(),
            is_final: is_final(),
            is_const: is_const(),
            is_null: is_null(),
            is_local: is_local(),
            get_access: get_access(),
            has_meta: has_meta(),
            set_meta: set_meta(),
            with_ctx: with_ctx(),
            ctx_new: ctx_new(),
            ctx_repr: ctx_repr(),
            ctx_prelude: ctx_prelude(),
            ctx_this: ctx_this(),
        }
    }
}

impl Prelude for CtxPrelude {
    fn put(&self, m: &mut NameMap) {
        self.read.put(m);
        self.move1.put(m);
        self.assign.put(m);
        self.set_final.put(m);
        self.set_const.put(m);
        self.is_final.put(m);
        self.is_const.put(m);
        self.is_null.put(m);
        self.is_local.put(m);
        self.get_access.put(m);
        self.has_meta.put(m);
        self.set_meta.put(m);
        self.with_ctx.put(m);
        self.ctx_new.put(m);
        self.ctx_repr.put(m);
        self.ctx_prelude.put(m);
        self.ctx_this.put(m);
    }
}

fn read() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("read", input_mode, output_mode, fn_read)
}

fn fn_read(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.get(&s).unwrap_or_default()
}

fn move1() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("move", input_mode, output_mode, fn_move)
}

fn fn_move(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.remove(&s).unwrap_or_default()
}

fn assign() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::Less),
        IoMode::Any(EvalMode::More),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("=", input_mode, output_mode, fn_assign)
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
            while let (Some(name), val) = (name_iter.next(), val_iter.next()) {
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
                let val = val.unwrap_or_default();
                last_list.push(assign_destruct(ctx, name, val, local, tag));
            }
            Val::List(last_list)
        }
        Val::Map(name_map) => {
            let Val::Map(mut val_map) = val else {
                return Val::default();
            };
            let last_map = name_map
                .into_iter()
                .map(|(k, name)| {
                    let val = val_map.remove(&k).unwrap_or_default();
                    let last_val = assign_destruct(ctx, name, val, local, tag);
                    (k, last_val)
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

fn set_final() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    named_mutable_fn("set_final", input_mode, output_mode, fn_set_final)
}

fn fn_set_final(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_final(&s);
    Val::default()
}

fn set_const() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::Value);
    named_mutable_fn("set_constant", input_mode, output_mode, fn_set_const)
}

fn fn_set_const(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_const(&s);
    Val::default()
}

fn is_final() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("is_final", input_mode, output_mode, fn_is_final)
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

fn is_const() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("is_constant", input_mode, output_mode, fn_is_const)
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

fn is_null() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("is_null", input_mode, output_mode, fn_is_null)
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

fn is_local() -> Named<FuncVal> {
    let input_mode = IoMode::Symbol(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("is_local", input_mode, output_mode, fn_is_local)
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

fn get_access() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Symbol(EvalMode::Value);
    named_mutable_fn("access", input_mode, output_mode, fn_get_access)
}

const ACCESS_FREE: &str = "free";
const ACCESS_CONSTANT: &str = "constant";
const ACCESS_MUTABLE: &str = "mutable";

fn fn_get_access(ctx: CtxForMutableFn, _input: Val) -> Val {
    let access = match ctx {
        CtxForMutableFn::Free(_) => ACCESS_FREE,
        CtxForMutableFn::Const(_) => ACCESS_CONSTANT,
        CtxForMutableFn::Mutable(_) => ACCESS_MUTABLE,
    };
    symbol(access)
}

fn has_meta() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("has_meta", input_mode, output_mode, fn_has_meta)
}

fn fn_has_meta(ctx: CtxForConstFn, _input: Val) -> Val {
    match ctx.get_meta() {
        Ok(_) => Val::Bool(Bool::t()),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

fn set_meta() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Any(EvalMode::Value);
    named_mutable_fn("set_meta", input_mode, output_mode, fn_set_meta)
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

fn with_ctx() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::ListForAll(Box::new(IoMode::Symbol(EvalMode::Value))),
        IoMode::Call(Box::new(Call::new(
            IoMode::Any(EvalMode::More),
            IoMode::Any(EvalMode::Value),
        ))),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_mutable_fn("do", input_mode, output_mode, fn_with_ctx)
}

fn fn_with_ctx(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    match pair.second {
        Val::Call(call) => {
            let Val::Func(FuncVal(func)) = call.func else {
                return Val::default();
            };
            let target_ctx = pair.first;
            let input = func.input_mode.eval(&mut ctx, call.input);

            match target_ctx {
                Val::Unit(_) => func.eval(&mut FreeCtx, input),
                Val::Symbol(name) if &*name == "meta" => {
                    let Ok(meta) = ctx.get_tagged_meta() else {
                        return Val::default();
                    };
                    if meta.is_const {
                        func.eval(&mut ConstCtx::new(meta.val_ref), input)
                    } else {
                        func.eval(&mut MutableCtx::new(meta.val_ref), input)
                    }
                }
                Val::List(names) => {
                    get_ctx_nested(ctx, &names[..], |mut ctx| func.eval(&mut ctx, input))
                }
                _ => Val::default(),
            }
        }
        Val::Reverse(reverse) => {
            let Val::Func(FuncVal(func)) = &reverse.func else {
                return Val::default();
            };
            let target_ctx = pair.first;
            let output = func.output_mode.eval(&mut ctx, reverse.output);
            let reverse = ValBuilder.from_reverse(reverse.func, output);
            match target_ctx {
                Val::Unit(_) => solve(&mut FreeCtx, reverse),
                Val::Symbol(name) if &*name == "meta" => {
                    let Ok(meta) = ctx.get_tagged_meta() else {
                        return Val::default();
                    };
                    if meta.is_const {
                        solve(&mut ConstCtx::new(meta.val_ref), reverse)
                    } else {
                        solve(&mut MutableCtx::new(meta.val_ref), reverse)
                    }
                }
                Val::List(names) => {
                    get_ctx_nested(ctx, &names[..], |mut ctx| solve(&mut ctx, reverse))
                }
                _ => Val::default(),
            }
        }
        _ => Val::default(),
    }
}

fn get_ctx_nested<F>(mut ctx: CtxForMutableFn, names: &[Val], f: F) -> Val
where
    F: for<'a> FnOnce(CtxForMutableFn<'a>) -> Val,
{
    let Some(Val::Symbol(name)) = names.first() else {
        return f(ctx);
    };
    let rest = &names[1..];

    let Ok(TaggedRef { val_ref, is_const }) = ctx.get_tagged_ref(name) else {
        return Val::default();
    };
    let Val::Ctx(CtxVal(ctx)) = val_ref else {
        return Val::default();
    };
    if is_const {
        get_ctx_nested(CtxForMutableFn::Const(ConstCtx::new(ctx)), rest, f)
    } else {
        get_ctx_nested(CtxForMutableFn::Mutable(MutableCtx::new(ctx)), rest, f)
    }
}

const CTX_VALUE_PAIR: &str = ":";
const VARIABLE: &str = "variable";
const FINAL: &str = "final";
const CONST: &str = "constant";

fn ctx_new() -> Named<FuncVal> {
    let input_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::MapForAll(Box::new(Pair::new(
            IoMode::Any(EvalMode::Less),
            IoMode::Any(EvalMode::More),
        ))),
    )));
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn("context", input_mode, output_mode, fn_ctx_new)
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

fn ctx_repr() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::More);
    let output_mode = IoMode::Pair(Box::new(Pair::new(
        IoMode::Any(EvalMode::More),
        IoMode::MapForAll(Box::new(Pair::new(
            IoMode::Any(EvalMode::Less),
            IoMode::Any(EvalMode::More),
        ))),
    )));
    named_free_fn("context.represent", input_mode, output_mode, fn_ctx_repr)
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

fn ctx_prelude() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_free_fn("prelude", input_mode, output_mode, fn_ctx_prelude)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal(Box::new(initial_ctx())))
}

fn ctx_this() -> Named<FuncVal> {
    let input_mode = IoMode::Any(EvalMode::Value);
    let output_mode = IoMode::Any(EvalMode::More);
    named_const_fn("this", input_mode, output_mode, fn_ctx_this)
}

fn fn_ctx_this(ctx: CtxForConstFn, _input: Val) -> Val {
    let CtxForConstFn::Const(ctx) = ctx else {
        return Val::default();
    };
    Val::Ctx(CtxVal(Box::new(ctx.get_ctx_ref().clone())))
}
