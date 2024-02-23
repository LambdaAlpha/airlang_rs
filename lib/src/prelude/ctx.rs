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
        input::ByVal,
        Evaluator,
    },
    eval_mode::{
        eager::Eager,
        EvalMode,
    },
    io_mode::IoMode,
    list::List,
    pair::Pair,
    prelude::{
        default_mode,
        initial_ctx,
        map_mode_for_all,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        symbol_value_mode,
        utils::{
            map_remove,
            symbol,
        },
        Named,
        Prelude,
    },
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
    CallMode,
    ListMode,
    ListVal,
    MatchMode,
    PairVal,
    ReverseMode,
    ReverseVal,
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
    pub(crate) get_access: Named<FuncVal>,
    pub(crate) has_meta: Named<FuncVal>,
    pub(crate) set_meta: Named<FuncVal>,
    pub(crate) with_ctx: Named<FuncVal>,
    pub(crate) with_ctx_func: Named<FuncVal>,
    pub(crate) with_ctx_input: Named<FuncVal>,
    pub(crate) with_ctx_func_input: Named<FuncVal>,
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
            get_access: get_access(),
            has_meta: has_meta(),
            set_meta: set_meta(),
            with_ctx: with_ctx(),
            with_ctx_func: with_ctx_func(),
            with_ctx_input: with_ctx_input(),
            with_ctx_func_input: with_ctx_func_input(),
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
        self.get_access.put(m);
        self.has_meta.put(m);
        self.set_meta.put(m);
        self.with_ctx.put(m);
        self.with_ctx_func.put(m);
        self.with_ctx_input.put(m);
        self.with_ctx_func_input.put(m);
        self.ctx_new.put(m);
        self.ctx_repr.put(m);
        self.ctx_prelude.put(m);
        self.ctx_this.put(m);
    }
}

fn read() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_const_fn("read", input_mode, output_mode, fn_read)
}

fn fn_read(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.get(&s).unwrap_or_default()
}

fn move1() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
    named_mutable_fn("move", input_mode, output_mode, fn_move)
}

fn fn_move(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.remove(&s).unwrap_or_default()
}

fn assign() -> Named<FuncVal> {
    let input_mode = pair_mode(IoMode::Eval(EvalMode::Value), default_mode());
    let output_mode = default_mode();
    named_mutable_fn("=", input_mode, output_mode, fn_assign)
}

fn fn_assign(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let name = pair.first;
    let val = pair.second;
    let options = AssignOptions::default();
    assign_allow_options(&mut ctx, name, val, options)
}

const INVARIANT: &str = "invariant";

fn assign_destruct(
    ctx: &mut CtxForMutableFn,
    name: Val,
    val: Val,
    options: AssignOptions,
    allow_options: bool,
) -> Val {
    match name {
        Val::Symbol(s) => assign_symbol(ctx, s, val, options),
        Val::Pair(name) => assign_pair(ctx, *name, val, options),
        Val::Call(name) => {
            if allow_options {
                match parse_ctx_val_pair(name) {
                    ParseCtxValPairResult::Parsed { val: name, tag } => {
                        let options = AssignOptions { tag };
                        assign_destruct(ctx, name, val, options, false)
                    }
                    ParseCtxValPairResult::Fallback(name) => assign_call(ctx, *name, val, options),
                    ParseCtxValPairResult::None => Val::default(),
                }
            } else {
                assign_call(ctx, *name, val, options)
            }
        }
        Val::Reverse(name) => assign_reverse(ctx, *name, val, options),
        Val::List(name) => assign_list(ctx, name, val, options),
        Val::Map(name) => assign_map(ctx, name, val, options),
        _ => Val::default(),
    }
}

fn assign_allow_options(
    ctx: &mut CtxForMutableFn,
    name: Val,
    val: Val,
    options: AssignOptions,
) -> Val {
    assign_destruct(ctx, name, val, options, true)
}

fn assign_symbol(ctx: &mut CtxForMutableFn, name: Symbol, val: Val, options: AssignOptions) -> Val {
    let tagged_val = TaggedVal {
        val,
        tag: options.tag,
    };
    let Ok(last) = ctx.put_val(name, tagged_val) else {
        return Val::default();
    };
    last.unwrap_or_default()
}

fn assign_pair(ctx: &mut CtxForMutableFn, name: PairVal, val: Val, options: AssignOptions) -> Val {
    let Val::Pair(val) = val else {
        return Val::default();
    };
    let first = assign_allow_options(ctx, name.first, val.first, options);
    let second = assign_allow_options(ctx, name.second, val.second, options);
    Val::Pair(Box::new(Pair::new(first, second)))
}

fn assign_call(ctx: &mut CtxForMutableFn, name: CallVal, val: Val, options: AssignOptions) -> Val {
    let Val::Call(val) = val else {
        return Val::default();
    };
    let func = assign_allow_options(ctx, name.func, val.func, options);
    let input = assign_allow_options(ctx, name.input, val.input, options);
    Val::Call(Box::new(Call::new(func, input)))
}

fn assign_reverse(
    ctx: &mut CtxForMutableFn,
    name: ReverseVal,
    val: Val,
    options: AssignOptions,
) -> Val {
    let Val::Reverse(val) = val else {
        return Val::default();
    };
    let func = assign_allow_options(ctx, name.func, val.func, options);
    let output = assign_allow_options(ctx, name.output, val.output, options);
    Val::Reverse(Box::new(Reverse::new(func, output)))
}

fn assign_list(ctx: &mut CtxForMutableFn, name: ListVal, val: Val, options: AssignOptions) -> Val {
    let Val::List(val) = val else {
        return Val::default();
    };
    let mut list = List::default();
    let mut name_iter = name.into_iter();
    let mut val_iter: Box<dyn ExactSizeIterator<Item = Val>> = Box::new(val.into_iter());
    while let (Some(name), val) = (name_iter.next(), val_iter.next()) {
        if let Val::Symbol(s) = &name {
            if &**s == "..." {
                let name_len = name_iter.len();
                let val_len = val_iter.len();
                if val_len > name_len {
                    val_iter = Box::new(val_iter.skip(val_len - name_len));
                }
                list.push(Val::default());
                continue;
            }
        }
        let val = val.unwrap_or_default();
        list.push(assign_allow_options(ctx, name, val, options));
    }
    Val::List(list)
}

fn assign_map(ctx: &mut CtxForMutableFn, name: MapVal, val: Val, options: AssignOptions) -> Val {
    let Val::Map(mut val) = val else {
        return Val::default();
    };
    let map = name
        .into_iter()
        .map(|(k, name)| {
            let val = val.remove(&k).unwrap_or_default();
            let last_val = assign_allow_options(ctx, name, val, options);
            (k, last_val)
        })
        .collect();
    Val::Map(map)
}

#[derive(Copy, Clone)]
struct AssignOptions {
    tag: InvariantTag,
}

impl Default for AssignOptions {
    fn default() -> Self {
        AssignOptions {
            tag: InvariantTag::None,
        }
    }
}

enum ParseCtxValPairResult {
    Parsed { val: Val, tag: InvariantTag },
    Fallback(Box<CallVal>),
    None,
}

fn parse_ctx_val_pair(call: Box<CallVal>) -> ParseCtxValPairResult {
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
    let tag = match pair.second {
        Val::Symbol(s) => {
            if let Some(tag) = parse_invariant_tag(&s) {
                tag
            } else {
                return ParseCtxValPairResult::None;
            }
        }
        Val::Map(mut map) => match map_remove(&mut map, INVARIANT) {
            Val::Symbol(tag) => {
                if let Some(tag) = parse_invariant_tag(&tag) {
                    tag
                } else {
                    return ParseCtxValPairResult::None;
                }
            }
            Val::Unit(_) => InvariantTag::None,
            _ => return ParseCtxValPairResult::None,
        },
        _ => return ParseCtxValPairResult::None,
    };
    ParseCtxValPairResult::Parsed { val, tag }
}

fn parse_invariant_tag(tag: &str) -> Option<InvariantTag> {
    let tag = match tag {
        NONE => InvariantTag::None,
        FINAL => InvariantTag::Final,
        CONST => InvariantTag::Const,
        _ => return None,
    };
    Some(tag)
}

fn generate_invariant_tag(tag: InvariantTag) -> Symbol {
    let tag = match tag {
        InvariantTag::None => NONE,
        InvariantTag::Final => FINAL,
        InvariantTag::Const => CONST,
    };
    Symbol::from_str(tag)
}

fn set_final() -> Named<FuncVal> {
    let input_mode = symbol_value_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
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
    let input_mode = symbol_value_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
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
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
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
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
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
    let input_mode = symbol_value_mode();
    let output_mode = default_mode();
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

fn get_access() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = symbol_value_mode();
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
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = default_mode();
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
    let input_mode = default_mode();
    let output_mode = IoMode::Eval(EvalMode::Value);
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
    let input_mode = pair_mode(
        IoMode::Match(MatchMode {
            symbol: EvalMode::Value,
            list: Box::new(ListMode::ForAll(symbol_value_mode())),
            ..Default::default()
        }),
        IoMode::Match(MatchMode {
            call: Box::new(CallMode::Call(Call::new(
                IoMode::Eval(EvalMode::Eager),
                IoMode::Eval(EvalMode::Value),
            ))),
            reverse: Box::new(ReverseMode::Reverse(Reverse::new(
                IoMode::Eval(EvalMode::Eager),
                IoMode::Eval(EvalMode::Value),
            ))),
            ..Default::default()
        }),
    );
    let output_mode = default_mode();
    named_mutable_fn("..", input_mode, output_mode, fn_with_ctx)
}

fn fn_with_ctx(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    match pair.second {
        Val::Call(call) => {
            let func = call.func;
            let input = Eager.eval_input(&mut ctx, &func, call.input);
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager::call(&mut target_ctx, func, input)
            });
            result.unwrap_or_default()
        }
        Val::Reverse(reverse) => {
            let func = reverse.func;
            let output = Eager.eval_output(&mut ctx, &func, reverse.output);
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager::solve(&mut target_ctx, func, output)
            });
            result.unwrap_or_default()
        }
        _ => Val::default(),
    }
}

fn with_ctx_func() -> Named<FuncVal> {
    let input_mode = pair_mode(
        IoMode::Match(MatchMode {
            symbol: EvalMode::Value,
            list: Box::new(ListMode::ForAll(symbol_value_mode())),
            ..Default::default()
        }),
        IoMode::Match(MatchMode {
            call: Box::new(CallMode::Call(Call::new(
                IoMode::Eval(EvalMode::Value),
                IoMode::Eval(EvalMode::Value),
            ))),
            reverse: Box::new(ReverseMode::Reverse(Reverse::new(
                IoMode::Eval(EvalMode::Value),
                IoMode::Eval(EvalMode::Value),
            ))),
            ..Default::default()
        }),
    );
    let output_mode = default_mode();
    named_mutable_fn(":.", input_mode, output_mode, fn_with_ctx_func)
}

fn fn_with_ctx_func(mut ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    match pair.second {
        Val::Call(call) => {
            let func = call.func;
            let input = call.input;
            let Some(func) = with_target_ctx(ctx.reborrow(), &pair.first, |mut target_ctx| {
                Eager.eval(&mut target_ctx, func)
            }) else {
                return Val::default();
            };
            let input = Eager.eval_input(&mut ctx, &func, input);
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager::call(&mut target_ctx, func, input)
            });
            result.unwrap_or_default()
        }
        Val::Reverse(reverse) => {
            let func = reverse.func;
            let output = reverse.output;
            let Some(func) = with_target_ctx(ctx.reborrow(), &pair.first, |mut target_ctx| {
                Eager.eval(&mut target_ctx, func)
            }) else {
                return Val::default();
            };
            let output = Eager.eval_output(&mut ctx, &func, output);
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager::solve(&mut target_ctx, func, output)
            });
            result.unwrap_or_default()
        }
        _ => Val::default(),
    }
}

fn with_ctx_input() -> Named<FuncVal> {
    let input_mode = pair_mode(
        IoMode::Match(MatchMode {
            symbol: EvalMode::Value,
            list: Box::new(ListMode::ForAll(symbol_value_mode())),
            ..Default::default()
        }),
        IoMode::Match(MatchMode {
            call: Box::new(CallMode::Call(Call::new(
                IoMode::Eval(EvalMode::Eager),
                IoMode::Eval(EvalMode::Value),
            ))),
            reverse: Box::new(ReverseMode::Reverse(Reverse::new(
                IoMode::Eval(EvalMode::Eager),
                IoMode::Eval(EvalMode::Value),
            ))),
            ..Default::default()
        }),
    );
    let output_mode = default_mode();
    named_mutable_fn(".:", input_mode, output_mode, fn_with_ctx_input)
}

fn fn_with_ctx_input(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    match pair.second {
        Val::Call(call) => {
            let func = call.func;
            let input = call.input;
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager.eval_input_then_call(&mut target_ctx, func, input)
            });
            result.unwrap_or_default()
        }
        Val::Reverse(reverse) => {
            let func = reverse.func;
            let output = reverse.output;
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager.eval_output_then_solve(&mut target_ctx, func, output)
            });
            result.unwrap_or_default()
        }
        _ => Val::default(),
    }
}

fn with_ctx_func_input() -> Named<FuncVal> {
    let input_mode = pair_mode(
        IoMode::Match(MatchMode {
            symbol: EvalMode::Value,
            list: Box::new(ListMode::ForAll(symbol_value_mode())),
            ..Default::default()
        }),
        IoMode::Match(MatchMode {
            call: Box::new(CallMode::Call(Call::new(
                IoMode::Eval(EvalMode::Value),
                IoMode::Eval(EvalMode::Value),
            ))),
            reverse: Box::new(ReverseMode::Reverse(Reverse::new(
                IoMode::Eval(EvalMode::Value),
                IoMode::Eval(EvalMode::Value),
            ))),
            ..Default::default()
        }),
    );
    let output_mode = default_mode();
    named_mutable_fn("::", input_mode, output_mode, fn_with_ctx_func_input)
}

fn fn_with_ctx_func_input(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    match pair.second {
        Val::Call(call) => {
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager.eval_call(&mut target_ctx, call.func, call.input)
            });
            result.unwrap_or_default()
        }
        Val::Reverse(reverse) => {
            let result = with_target_ctx(ctx, &pair.first, |mut target_ctx| {
                Eager.eval_reverse(&mut target_ctx, reverse.func, reverse.output)
            });
            result.unwrap_or_default()
        }
        _ => Val::default(),
    }
}

fn with_target_ctx<F>(ctx: CtxForMutableFn, target_ctx: &Val, callback: F) -> Option<Val>
where
    F: FnOnce(CtxForMutableFn) -> Val,
{
    match target_ctx {
        Val::List(names) => get_ctx_nested(ctx, &names[..], callback),
        _ => with_target_ctx_basic(ctx, target_ctx, |ctx| Some(callback(ctx))),
    }
}

fn with_target_ctx_basic<F>(mut ctx: CtxForMutableFn, target_ctx: &Val, callback: F) -> Option<Val>
where
    F: FnOnce(CtxForMutableFn) -> Option<Val>,
{
    match target_ctx {
        Val::Unit(_) => callback(CtxForMutableFn::Free(FreeCtx)),
        Val::Bool(is_meta) => {
            if is_meta.bool() {
                let Ok(TaggedRef {
                    is_const,
                    val_ref: meta_ctx,
                }) = ctx.get_tagged_meta()
                else {
                    return None;
                };
                if is_const {
                    callback(CtxForMutableFn::Const(ConstCtx::new(meta_ctx)))
                } else {
                    callback(CtxForMutableFn::Mutable(MutableCtx::new(meta_ctx)))
                }
            } else {
                callback(ctx)
            }
        }
        Val::Symbol(name) => {
            let Ok(TaggedRef { val_ref, is_const }) = ctx.get_tagged_ref(name) else {
                return None;
            };
            let Val::Ctx(CtxVal(target_ctx)) = val_ref else {
                return None;
            };
            if is_const {
                callback(CtxForMutableFn::Const(ConstCtx::new(target_ctx)))
            } else {
                callback(CtxForMutableFn::Mutable(MutableCtx::new(target_ctx)))
            }
        }
        _ => None,
    }
}

fn get_ctx_nested<F>(ctx: CtxForMutableFn, names: &[Val], f: F) -> Option<Val>
where
    F: for<'a> FnOnce(CtxForMutableFn<'a>) -> Val,
{
    let Some(target_ctx) = names.first() else {
        return Some(f(ctx));
    };
    let rest = &names[1..];

    with_target_ctx_basic(ctx, target_ctx, |ctx| get_ctx_nested(ctx, rest, f))
}

const CTX_VALUE_PAIR: &str = ":";
const NONE: &str = "none";
const FINAL: &str = "final";
const CONST: &str = "constant";

fn ctx_new() -> Named<FuncVal> {
    let input_mode = pair_mode(
        default_mode(),
        map_mode_for_all(symbol_value_mode(), default_mode()),
    );
    let output_mode = default_mode();
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
                match parse_ctx_val_pair(call) {
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
    let input_mode = default_mode();
    let output_mode = pair_mode(
        default_mode(),
        map_mode_for_all(symbol_value_mode(), default_mode()),
    );
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
            let use_normal_form = 'a: {
                if let Val::Call(call) = &v.val {
                    if let Val::Symbol(func) = &call.func {
                        if &**func == CTX_VALUE_PAIR {
                            break 'a true;
                        }
                    }
                }
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
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = default_mode();
    named_free_fn("prelude", input_mode, output_mode, fn_ctx_prelude)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal(Box::new(initial_ctx())))
}

fn ctx_this() -> Named<FuncVal> {
    let input_mode = IoMode::Eval(EvalMode::Value);
    let output_mode = default_mode();
    named_const_fn("this", input_mode, output_mode, fn_ctx_this)
}

fn fn_ctx_this(ctx: CtxForConstFn, _input: Val) -> Val {
    let CtxForConstFn::Const(ctx) = ctx else {
        return Val::default();
    };
    Val::Ctx(CtxVal(Box::new(ctx.get_ctx_ref().clone())))
}
