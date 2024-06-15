use crate::{
    ask::Ask,
    bool::Bool,
    call::Call,
    ctx::{
        constant::{
            ConstCtx,
            CtxForConstFn,
        },
        free::FreeCtx,
        mutable::{
            CtxForMutableFn,
            MutableCtx,
        },
        ref1::CtxRef,
        Ctx,
        CtxError,
        CtxMap,
        CtxValue,
        DefaultCtx,
        DynRef,
        Invariant,
    },
    list::List,
    mode::Mode,
    pair::Pair,
    prelude::{
        form_mode,
        initial_ctx,
        map_all_mode,
        named_const_fn,
        named_free_fn,
        named_mutable_fn,
        pair_mode,
        Named,
        Prelude,
    },
    symbol::Symbol,
    transform::{
        eval::Eval,
        Transform,
        SYMBOL_READ_PREFIX,
    },
    transformer::Transformer,
    unit::Unit,
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
        map::MapVal,
        Val,
    },
    AskVal,
    ListVal,
    Map,
    PairVal,
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
    pub(crate) ctx_in_ctx_out: Named<FuncVal>,
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
            ctx_in_ctx_out: ctx_in_ctx_out(),
            ctx_new: ctx_new(),
            ctx_repr: ctx_repr(),
            ctx_prelude: ctx_prelude(),
            ctx_this: ctx_this(),
        }
    }
}

impl Prelude for CtxPrelude {
    fn put(&self, m: &mut CtxMap) {
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
        self.ctx_in_ctx_out.put(m);
        self.ctx_new.put(m);
        self.ctx_repr.put(m);
        self.ctx_prelude.put(m);
        self.ctx_this.put(m);
    }
}

fn read() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_const_fn("read", input_mode, output_mode, fn_read)
}

fn fn_read(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    DefaultCtx.get_or_default(ctx, s)
}

fn move1() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_mutable_fn("move", input_mode, output_mode, fn_move)
}

fn fn_move(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.remove(s).unwrap_or_default()
}

fn assign() -> Named<FuncVal> {
    let input_mode = pair_mode(form_mode(), Mode::default(), Transform::default());
    let output_mode = Mode::default();
    named_mutable_fn("=", input_mode, output_mode, fn_assign)
}

fn fn_assign(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = pair.unwrap();
    let name = pair.first;
    let val = pair.second;
    let options = AssignOptions::default();
    assign_allow_options(ctx, name, val, options)
}

const INVARIANT: &str = "invariant";

fn assign_destruct(
    mut ctx: CtxForMutableFn,
    name: Val,
    val: Val,
    options: AssignOptions,
    allow_options: bool,
) -> Val {
    match name {
        Val::Symbol(s) => assign_symbol(ctx.reborrow(), s, val, options),
        Val::Pair(name) => assign_pair(ctx, name, val, options),
        Val::Call(name) => {
            if allow_options {
                match parse_ctx_val_pair(name) {
                    ParseCtxValPairResult::Parsed {
                        val: name,
                        invariant,
                    } => {
                        let options = AssignOptions { invariant };
                        assign_destruct(ctx, name, val, options, false)
                    }
                    ParseCtxValPairResult::Fallback(name) => assign_call(ctx, name, val, options),
                    ParseCtxValPairResult::None => Val::default(),
                }
            } else {
                assign_call(ctx, name, val, options)
            }
        }
        Val::Ask(name) => assign_ask(ctx, name, val, options),
        Val::List(name) => assign_list(ctx, name, val, options),
        Val::Map(name) => assign_map(ctx, name, val, options),
        _ => Val::default(),
    }
}

fn assign_allow_options(ctx: CtxForMutableFn, name: Val, val: Val, options: AssignOptions) -> Val {
    assign_destruct(ctx, name, val, options, true)
}

fn assign_symbol(ctx: CtxForMutableFn, name: Symbol, val: Val, options: AssignOptions) -> Val {
    let ctx_value = CtxValue {
        val,
        invariant: options.invariant,
    };
    let Ok(last) = ctx.put_value(name, ctx_value) else {
        return Val::default();
    };
    last.unwrap_or_default()
}

fn assign_pair(mut ctx: CtxForMutableFn, name: PairVal, val: Val, options: AssignOptions) -> Val {
    let Val::Pair(val) = val else {
        return Val::default();
    };
    let val = Pair::from(val);
    let name = Pair::from(name);
    let first = assign_allow_options(ctx.reborrow(), name.first, val.first, options);
    let second = assign_allow_options(ctx, name.second, val.second, options);
    Val::Pair(Pair::new(first, second).into())
}

fn assign_call(mut ctx: CtxForMutableFn, name: CallVal, val: Val, options: AssignOptions) -> Val {
    let Val::Call(val) = val else {
        return Val::default();
    };
    let name = Call::from(name);
    let val = Call::from(val);
    let func = assign_allow_options(ctx.reborrow(), name.func, val.func, options);
    let input = assign_allow_options(ctx, name.input, val.input, options);
    Val::Call(Call::new(func, input).into())
}

fn assign_ask(mut ctx: CtxForMutableFn, name: AskVal, val: Val, options: AssignOptions) -> Val {
    let Val::Ask(val) = val else {
        return Val::default();
    };
    let name = Ask::from(name);
    let val = Ask::from(val);
    let func = assign_allow_options(ctx.reborrow(), name.func, val.func, options);
    let output = assign_allow_options(ctx, name.output, val.output, options);
    Val::Ask(Ask::new(func, output).into())
}

fn assign_list(mut ctx: CtxForMutableFn, name: ListVal, val: Val, options: AssignOptions) -> Val {
    let Val::List(val) = val else {
        return Val::default();
    };
    let name = List::from(name);
    let val = List::from(val);
    let mut list = List::default();
    let mut name_iter = name.into_iter();
    let mut val_iter: Box<dyn ExactSizeIterator<Item = Val>> = Box::new(val.into_iter());
    while let (Some(name), val) = (name_iter.next(), val_iter.next()) {
        if let Val::Symbol(s) = &name {
            if &**s == ".." {
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
        list.push(assign_allow_options(ctx.reborrow(), name, val, options));
    }
    Val::List(list.into())
}

fn assign_map(mut ctx: CtxForMutableFn, name: MapVal, val: Val, options: AssignOptions) -> Val {
    let Val::Map(mut val) = val else {
        return Val::default();
    };
    let name = Map::from(name);
    let map: Map<Val, Val> = name
        .into_iter()
        .map(|(k, name)| {
            let val = val.remove(&k).unwrap_or_default();
            let last_val = assign_allow_options(ctx.reborrow(), name, val, options);
            (k, last_val)
        })
        .collect();
    Val::Map(map.into())
}

#[derive(Copy, Clone)]
struct AssignOptions {
    invariant: Invariant,
}

impl Default for AssignOptions {
    fn default() -> Self {
        AssignOptions {
            invariant: Invariant::None,
        }
    }
}

enum ParseCtxValPairResult {
    Parsed { val: Val, invariant: Invariant },
    Fallback(CallVal),
    None,
}

fn parse_ctx_val_pair(call: CallVal) -> ParseCtxValPairResult {
    let Val::Unit(_) = &call.func else {
        return ParseCtxValPairResult::Fallback(call);
    };
    let call = Call::from(call);
    let Val::Pair(pair) = call.input else {
        return ParseCtxValPairResult::None;
    };
    let pair = Pair::from(pair);
    let val = pair.first;
    let invariant = match pair.second {
        Val::Symbol(s) => {
            if let Some(invariant) = parse_invariant(&s) {
                invariant
            } else {
                return ParseCtxValPairResult::None;
            }
        }
        Val::Map(mut map) => match map_remove(&mut map, INVARIANT) {
            Val::Symbol(invariant) => {
                if let Some(invariant) = parse_invariant(&invariant) {
                    invariant
                } else {
                    return ParseCtxValPairResult::None;
                }
            }
            Val::Unit(_) => Invariant::None,
            _ => return ParseCtxValPairResult::None,
        },
        _ => return ParseCtxValPairResult::None,
    };
    ParseCtxValPairResult::Parsed { val, invariant }
}

fn parse_invariant(invariant: &str) -> Option<Invariant> {
    let invariant = match invariant {
        NONE => Invariant::None,
        FINAL => Invariant::Final,
        CONST => Invariant::Const,
        _ => return None,
    };
    Some(invariant)
}

fn generate_invariant(invariant: Invariant) -> Symbol {
    let invariant = match invariant {
        Invariant::None => NONE,
        Invariant::Final => FINAL,
        Invariant::Const => CONST,
    };
    Symbol::from_str(invariant)
}

fn set_final() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_mutable_fn("set_final", input_mode, output_mode, fn_set_final)
}

fn fn_set_final(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_final(s);
    Val::default()
}

fn set_const() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_mutable_fn("set_constant", input_mode, output_mode, fn_set_const)
}

fn fn_set_const(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_const(s);
    Val::default()
}

fn is_final() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_const_fn("is_final", input_mode, output_mode, fn_is_final)
}

fn fn_is_final(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match ctx.is_final(s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

fn is_const() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_const_fn("is_constant", input_mode, output_mode, fn_is_const)
}

fn fn_is_const(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match ctx.is_const(s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

fn is_null() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_const_fn("is_null", input_mode, output_mode, fn_is_null)
}

fn fn_is_null(ctx: CtxForConstFn, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match DefaultCtx.is_null(ctx, s) {
        Ok(b) => Val::Bool(Bool::new(b)),
        Err(_) => Val::default(),
    }
}

fn get_access() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
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
    let input_mode = Mode::default();
    let output_mode = Mode::default();
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
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mutable_fn("set_meta", input_mode, output_mode, fn_set_meta)
}

fn fn_set_meta(ctx: CtxForMutableFn, input: Val) -> Val {
    match input {
        Val::Unit(_) => {
            let _ = ctx.set_meta(None);
        }
        Val::Ctx(meta) => {
            let meta = Ctx::from(meta);
            let _ = ctx.set_meta(Some(meta));
        }
        _ => {}
    }
    Val::default()
}

fn with_ctx() -> Named<FuncVal> {
    let input_mode = pair_mode(form_mode(), Mode::default(), Transform::default());
    let output_mode = Mode::default();
    named_mutable_fn("|", input_mode, output_mode, fn_with_ctx)
}

fn fn_with_ctx(ctx: CtxForMutableFn, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let val = pair.second;
    let result = with_target_ctx(ctx, &pair.first, |target_ctx| {
        Eval.transform(target_ctx, val)
    });
    result.unwrap_or_default()
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

const META: &str = "meta";
const THIS: &str = "this";

fn with_target_ctx_basic<F>(ctx: CtxForMutableFn, target_ctx: &Val, callback: F) -> Option<Val>
where
    F: FnOnce(CtxForMutableFn) -> Option<Val>,
{
    match target_ctx {
        Val::Unit(_) => callback(CtxForMutableFn::Free(FreeCtx)),
        Val::Symbol(name) => match name.chars().next() {
            Some(Symbol::ID_PREFIX) => match &name[1..] {
                META => {
                    let Ok(DynRef {
                        is_const,
                        ref1: meta_ctx,
                    }) = ctx.get_meta_dyn()
                    else {
                        return None;
                    };
                    if is_const {
                        callback(CtxForMutableFn::Const(ConstCtx::new(meta_ctx)))
                    } else {
                        callback(CtxForMutableFn::Mutable(MutableCtx::new(meta_ctx)))
                    }
                }
                THIS => callback(ctx),
                _ => None,
            },
            Some(SYMBOL_READ_PREFIX) => {
                let name = Symbol::from_str(&name[1..]);
                let Ok(DynRef { ref1, is_const }) = ctx.get_ref_dyn(name) else {
                    return None;
                };
                let Val::Ctx(target_ctx) = ref1 else {
                    return None;
                };
                if is_const {
                    callback(CtxForMutableFn::Const(ConstCtx::new(target_ctx)))
                } else {
                    callback(CtxForMutableFn::Mutable(MutableCtx::new(target_ctx)))
                }
            }
            _ => {
                let Ok(DynRef { ref1, is_const }) = ctx.get_ref_dyn(name.clone()) else {
                    return None;
                };
                let Val::Ctx(target_ctx) = ref1 else {
                    return None;
                };
                if is_const {
                    callback(CtxForMutableFn::Const(ConstCtx::new(target_ctx)))
                } else {
                    callback(CtxForMutableFn::Mutable(MutableCtx::new(target_ctx)))
                }
            }
        },
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

fn ctx_in_ctx_out() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("::", input_mode, output_mode, fn_ctx_in_ctx_out)
}

fn fn_ctx_in_ctx_out(input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let ctx_input = Pair::from(pair);
    let Val::Ctx(ctx) = ctx_input.first else {
        return Val::default();
    };
    let mut ctx = Ctx::from(ctx);
    let input = ctx_input.second;
    let output = Eval.transform(MutableCtx::new(&mut ctx), input);
    let pair = Pair::new(Val::Ctx(ctx.into()), output);
    Val::Pair(pair.into())
}

const NONE: &str = "none";
const FINAL: &str = "final";
const CONST: &str = "constant";

fn ctx_new() -> Named<FuncVal> {
    let input_mode = pair_mode(
        Mode::default(),
        map_all_mode(form_mode(), Mode::default(), Transform::default()),
        Transform::default(),
    );
    let output_mode = Mode::default();
    named_free_fn("context", input_mode, output_mode, fn_ctx_new)
}

fn fn_ctx_new(input: Val) -> Val {
    let Val::Pair(meta_map) = input else {
        return Val::default();
    };
    let meta_map = Pair::from(meta_map);
    let meta = match meta_map.first {
        Val::Unit(_) => None,
        Val::Ctx(meta) => Some(meta.unwrap()),
        _ => return Val::default(),
    };

    let ctx_map_repr = match meta_map.second {
        Val::Map(ctx_map) => Map::from(ctx_map),
        Val::Unit(_) => Map::default(),
        _ => return Val::default(),
    };

    let mut ctx_map = CtxMap::with_capacity(ctx_map_repr.len());

    for (key, val) in ctx_map_repr {
        let Val::Symbol(name) = key else {
            return Val::default();
        };
        let ctx_value = {
            if let Val::Call(call) = val {
                match parse_ctx_val_pair(call) {
                    ParseCtxValPairResult::Parsed { val, invariant, .. } => {
                        CtxValue { val, invariant }
                    }
                    ParseCtxValPairResult::Fallback(call) => CtxValue::new(Val::Call(call)),
                    ParseCtxValPairResult::None => {
                        return Val::default();
                    }
                }
            } else {
                CtxValue::new(val)
            }
        };
        ctx_map.insert(name, ctx_value);
    }

    Val::Ctx(Ctx::new(ctx_map, meta).into())
}

fn ctx_repr() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = pair_mode(
        Mode::default(),
        map_all_mode(form_mode(), Mode::default(), Transform::default()),
        Transform::default(),
    );
    named_free_fn("context.represent", input_mode, output_mode, fn_ctx_repr)
}

fn fn_ctx_repr(input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        return Val::default();
    };
    let ctx = Ctx::from(ctx);

    let meta = match ctx.meta {
        Some(meta) => Val::Ctx(CtxVal::new(meta)),
        None => Val::Unit(Unit),
    };

    let map: Map<Val, Val> = ctx
        .map
        .into_iter()
        .map(|(k, v)| {
            let k = Val::Symbol(k);
            let use_normal_form = 'a: {
                if let Val::Call(call) = &v.val {
                    if let Val::Unit(_) = &call.func {
                        break 'a true;
                    }
                }
                matches!(v.invariant, Invariant::Final | Invariant::Const)
            };
            let v = if use_normal_form {
                let func = Val::Unit(Unit);
                let invariant = generate_invariant(v.invariant);
                let pair = Val::Pair(Pair::new(v.val, Val::Symbol(invariant)).into());
                Val::Call(Call::new(func, pair).into())
            } else {
                v.val
            };
            (k, v)
        })
        .collect();
    let map = Val::Map(map.into());
    Val::Pair(Pair::new(meta, map).into())
}

fn ctx_prelude() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("prelude", input_mode, output_mode, fn_ctx_prelude)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_this() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("this", input_mode, output_mode, fn_ctx_this)
}

fn fn_ctx_this(ctx: CtxForConstFn, _input: Val) -> Val {
    let CtxForConstFn::Const(ctx) = ctx else {
        return Val::default();
    };
    Val::Ctx(CtxVal::from(ctx.get_ctx_ref().clone()))
}
