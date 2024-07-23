use crate::{
    ask::Ask,
    bool::Bool,
    call::Call,
    ctx::{
        const1::{
            ConstCtx,
            ConstFnCtx,
        },
        free::FreeCtx,
        mut1::{
            MutCtx,
            MutFnCtx,
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
    mode::{
        basic::BasicMode,
        eval::Eval,
        Mode,
        SYMBOL_READ_PREFIX,
    },
    pair::Pair,
    prelude::{
        form_mode,
        initial_ctx,
        map_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        pair_mode,
        Named,
        Prelude,
    },
    symbol::Symbol,
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
        Val,
    },
    AskVal,
    Comment,
    CommentVal,
    ListVal,
    Map,
    MapVal,
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
    named_const_fn("read", input_mode, output_mode, true, fn_read)
}

fn fn_read(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    DefaultCtx.get_or_default(ctx, s)
}

fn move1() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_mut_fn("move", input_mode, output_mode, true, fn_move)
}

fn fn_move(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    ctx.remove(s).unwrap_or_default()
}

fn assign() -> Named<FuncVal> {
    let input_mode = pair_mode(form_mode(), Mode::default(), BasicMode::default());
    let output_mode = Mode::default();
    named_mut_fn("=", input_mode, output_mode, true, fn_assign)
}

fn fn_assign(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = pair.unwrap();
    let pattern_ctx = PatternCtx {
        extra: Extra {
            invariant: Invariant::None,
        },
        allow_extra: true,
    };
    let Some(pattern) = parse_pattern(pair.first, pattern_ctx) else {
        return Val::default();
    };
    let val = pair.second;
    assign_pattern(ctx, pattern, val)
}

const INVARIANT: &str = "invariant";

struct Binding<T> {
    name: T,
    extra: Extra,
}

#[derive(Copy, Clone)]
struct Extra {
    invariant: Invariant,
}

#[derive(Copy, Clone)]
struct PatternCtx {
    extra: Extra,
    allow_extra: bool,
}

enum Pattern {
    Any(Binding<Symbol>),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
    Ask(Box<Ask<Pattern, Pattern>>),
    Comment(Box<Comment<Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Val, Pattern>),
}

fn parse_pattern(pattern: Val, ctx: PatternCtx) -> Option<Pattern> {
    match pattern {
        Val::Symbol(name) => Some(parse_pattern_any(name, ctx)),
        Val::Pair(pair) => parse_pattern_pair(pair, ctx),
        Val::Call(call) => {
            if ctx.allow_extra && call.func.is_unit() {
                parse_pattern_extra(call, ctx)
            } else {
                parse_pattern_call(call, ctx)
            }
        }
        Val::Ask(ask) => parse_pattern_ask(ask, ctx),
        Val::Comment(comment) => parse_pattern_comment(comment, ctx),
        Val::List(list) => parse_pattern_list(list, ctx),
        Val::Map(map) => parse_pattern_map(map, ctx),
        _ => None,
    }
}

fn parse_pattern_any(name: Symbol, ctx: PatternCtx) -> Pattern {
    Pattern::Any(Binding {
        name,
        extra: ctx.extra,
    })
}

fn parse_pattern_pair(pair: PairVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let pair = Pair::from(pair);
    let first = parse_pattern(pair.first, ctx)?;
    let second = parse_pattern(pair.second, ctx)?;
    Some(Pattern::Pair(Box::new(Pair::new(first, second))))
}

fn parse_pattern_call(call: CallVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let call = Call::from(call);
    let func = parse_pattern(call.func, ctx)?;
    let input = parse_pattern(call.input, ctx)?;
    Some(Pattern::Call(Box::new(Call::new(func, input))))
}

fn parse_pattern_ask(ask: AskVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let ask = Ask::from(ask);
    let func = parse_pattern(ask.func, ctx)?;
    let output = parse_pattern(ask.output, ctx)?;
    Some(Pattern::Ask(Box::new(Ask::new(func, output))))
}

fn parse_pattern_comment(comment: CommentVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let comment = Comment::from(comment);
    let note = parse_pattern(comment.note, ctx)?;
    let value = parse_pattern(comment.value, ctx)?;
    Some(Pattern::Comment(Box::new(Comment::new(note, value))))
}

fn parse_pattern_list(list: ListVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let list = List::from(list);
    let list = list
        .into_iter()
        .map(|item| parse_pattern(item, ctx))
        .collect::<Option<List<_>>>()?;
    Some(Pattern::List(list))
}

fn parse_pattern_map(map: MapVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let map = Map::from(map);
    let map = map
        .into_iter()
        .map(|(k, v)| {
            let v = parse_pattern(v, ctx)?;
            Some((k, v))
        })
        .collect::<Option<Map<_, _>>>()?;
    Some(Pattern::Map(map))
}

fn parse_pattern_extra(call: CallVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = false;
    let call = Call::from(call);
    let Val::Pair(pair) = call.input else {
        return None;
    };
    let pair = Pair::from(pair);
    ctx.extra = parse_extra(pair.second, ctx.extra)?;
    parse_pattern(pair.first, ctx)
}

fn parse_extra(extra: Val, mut default: Extra) -> Option<Extra> {
    match extra {
        Val::Symbol(s) => {
            default.invariant = parse_invariant(&s)?;
        }
        Val::Map(mut map) => match map_remove(&mut map, INVARIANT) {
            Val::Symbol(invariant) => {
                default.invariant = parse_invariant(&invariant)?;
            }
            Val::Unit(_) => {}
            _ => return None,
        },
        _ => return None,
    }
    Some(default)
}

fn assign_pattern(ctx: MutFnCtx, pattern: Pattern, val: Val) -> Val {
    match pattern {
        Pattern::Any(binding) => assign_any(ctx, binding, val),
        Pattern::Pair(pair) => assign_pair(ctx, *pair, val),
        Pattern::Call(call) => assign_call(ctx, *call, val),
        Pattern::Ask(ask) => assign_ask(ctx, *ask, val),
        Pattern::Comment(comment) => assign_comment(ctx, *comment, val),
        Pattern::List(list) => assign_list(ctx, list, val),
        Pattern::Map(map) => assign_map(ctx, map, val),
    }
}

fn assign_any(ctx: MutFnCtx, binding: Binding<Symbol>, val: Val) -> Val {
    let ctx_value = CtxValue {
        val,
        invariant: binding.extra.invariant,
    };
    let Ok(last) = ctx.put_value(binding.name, ctx_value) else {
        return Val::default();
    };
    last.unwrap_or_default()
}

fn assign_pair(mut ctx: MutFnCtx, pattern: Pair<Pattern, Pattern>, val: Val) -> Val {
    let Val::Pair(val) = val else {
        return Val::default();
    };
    let val = Pair::from(val);
    let first = assign_pattern(ctx.reborrow(), pattern.first, val.first);
    let second = assign_pattern(ctx, pattern.second, val.second);
    Val::Pair(Pair::new(first, second).into())
}

fn assign_call(mut ctx: MutFnCtx, pattern: Call<Pattern, Pattern>, val: Val) -> Val {
    let Val::Call(val) = val else {
        return Val::default();
    };
    let val = Call::from(val);
    let func = assign_pattern(ctx.reborrow(), pattern.func, val.func);
    let input = assign_pattern(ctx, pattern.input, val.input);
    Val::Call(Call::new(func, input).into())
}

fn assign_ask(mut ctx: MutFnCtx, pattern: Ask<Pattern, Pattern>, val: Val) -> Val {
    let Val::Ask(val) = val else {
        return Val::default();
    };
    let val = Ask::from(val);
    let func = assign_pattern(ctx.reborrow(), pattern.func, val.func);
    let output = assign_pattern(ctx, pattern.output, val.output);
    Val::Ask(Ask::new(func, output).into())
}

fn assign_comment(mut ctx: MutFnCtx, pattern: Comment<Pattern, Pattern>, val: Val) -> Val {
    let Val::Comment(val) = val else {
        return Val::default();
    };
    let val = Comment::from(val);
    let note = assign_pattern(ctx.reborrow(), pattern.note, val.note);
    let value = assign_pattern(ctx, pattern.value, val.value);
    Val::Comment(Comment::new(note, value).into())
}

fn assign_list(mut ctx: MutFnCtx, pattern: List<Pattern>, val: Val) -> Val {
    let Val::List(val) = val else {
        return Val::default();
    };
    let mut list = List::from(Vec::with_capacity(pattern.len()));
    let mut val_iter = List::from(val).into_iter();
    for p in pattern {
        let val = val_iter.next().unwrap_or_default();
        let last_val = assign_pattern(ctx.reborrow(), p, val);
        list.push(last_val);
    }
    Val::List(list.into())
}

fn assign_map(mut ctx: MutFnCtx, pattern: Map<Val, Pattern>, val: Val) -> Val {
    let Val::Map(mut val) = val else {
        return Val::default();
    };
    let map: Map<Val, Val> = pattern
        .into_iter()
        .map(|(k, pattern)| {
            let val = val.remove(&k).unwrap_or_default();
            let last_val = assign_pattern(ctx.reborrow(), pattern, val);
            (k, last_val)
        })
        .collect();
    Val::Map(map.into())
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
    named_mut_fn("set_final", input_mode, output_mode, true, fn_set_final)
}

fn fn_set_final(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_final(s);
    Val::default()
}

fn set_const() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_mut_fn("set_constant", input_mode, output_mode, true, fn_set_const)
}

fn fn_set_const(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let _ = ctx.set_const(s);
    Val::default()
}

fn is_final() -> Named<FuncVal> {
    let input_mode = form_mode();
    let output_mode = Mode::default();
    named_const_fn("is_final", input_mode, output_mode, true, fn_is_final)
}

fn fn_is_final(ctx: ConstFnCtx, input: Val) -> Val {
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
    named_const_fn("is_constant", input_mode, output_mode, true, fn_is_const)
}

fn fn_is_const(ctx: ConstFnCtx, input: Val) -> Val {
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
    named_const_fn("is_null", input_mode, output_mode, true, fn_is_null)
}

fn fn_is_null(ctx: ConstFnCtx, input: Val) -> Val {
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
    named_mut_fn("access", input_mode, output_mode, true, fn_get_access)
}

const ACCESS_FREE: &str = "free";
const ACCESS_CONSTANT: &str = "constant";
const ACCESS_MUTABLE: &str = "mutable";

fn fn_get_access(ctx: MutFnCtx, _input: Val) -> Val {
    let access = match ctx {
        MutFnCtx::Free(_) => ACCESS_FREE,
        MutFnCtx::Const(_) => ACCESS_CONSTANT,
        MutFnCtx::Mut(_) => ACCESS_MUTABLE,
    };
    symbol(access)
}

fn has_meta() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("has_meta", input_mode, output_mode, true, fn_has_meta)
}

fn fn_has_meta(ctx: ConstFnCtx, _input: Val) -> Val {
    match ctx.get_meta() {
        Ok(_) => Val::Bool(Bool::t()),
        Err(CtxError::NotFound) => Val::Bool(Bool::f()),
        _ => Val::default(),
    }
}

fn set_meta() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("set_meta", input_mode, output_mode, true, fn_set_meta)
}

fn fn_set_meta(ctx: MutFnCtx, input: Val) -> Val {
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
    let input_mode = pair_mode(form_mode(), Mode::default(), BasicMode::default());
    let output_mode = Mode::default();
    named_mut_fn("|", input_mode, output_mode, false, fn_with_ctx)
}

fn fn_with_ctx(ctx: MutFnCtx, input: Val) -> Val {
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

fn with_target_ctx<F>(ctx: MutFnCtx, target_ctx: &Val, callback: F) -> Option<Val>
where
    F: FnOnce(MutFnCtx) -> Val,
{
    match target_ctx {
        Val::List(names) => get_ctx_nested(ctx, &names[..], callback),
        _ => with_target_ctx_basic(ctx, target_ctx, |ctx| Some(callback(ctx))),
    }
}

const META: &str = "meta";
const THIS: &str = "this";

fn with_target_ctx_basic<F>(ctx: MutFnCtx, target_ctx: &Val, callback: F) -> Option<Val>
where
    F: FnOnce(MutFnCtx) -> Option<Val>,
{
    match target_ctx {
        Val::Unit(_) => callback(MutFnCtx::Free(FreeCtx)),
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
                        callback(MutFnCtx::Const(ConstCtx::new(meta_ctx)))
                    } else {
                        callback(MutFnCtx::Mut(MutCtx::new(meta_ctx)))
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
                    callback(MutFnCtx::Const(ConstCtx::new(target_ctx)))
                } else {
                    callback(MutFnCtx::Mut(MutCtx::new(target_ctx)))
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
                    callback(MutFnCtx::Const(ConstCtx::new(target_ctx)))
                } else {
                    callback(MutFnCtx::Mut(MutCtx::new(target_ctx)))
                }
            }
        },
        _ => None,
    }
}

fn get_ctx_nested<F>(ctx: MutFnCtx, names: &[Val], f: F) -> Option<Val>
where
    F: for<'a> FnOnce(MutFnCtx<'a>) -> Val,
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
    named_free_fn("::", input_mode, output_mode, false, fn_ctx_in_ctx_out)
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
    let output = Eval.transform(MutCtx::new(&mut ctx), input);
    let pair = Pair::new(Val::Ctx(ctx.into()), output);
    Val::Pair(pair.into())
}

const NONE: &str = "none";
const FINAL: &str = "final";
const CONST: &str = "constant";

fn ctx_new() -> Named<FuncVal> {
    let input_mode = pair_mode(
        Mode::default(),
        map_mode(
            Map::default(),
            form_mode(),
            Mode::default(),
            BasicMode::default(),
        ),
        BasicMode::default(),
    );
    let output_mode = Mode::default();
    named_free_fn("context", input_mode, output_mode, true, fn_ctx_new)
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
                if call.func.is_unit() {
                    let call = Call::from(call);
                    let Val::Pair(pair) = call.input else {
                        return Val::default();
                    };
                    let pair = Pair::from(pair);
                    let Some(extra) = parse_extra(pair.second, Extra {
                        invariant: Invariant::None,
                    }) else {
                        return Val::default();
                    };
                    CtxValue {
                        val: pair.first,
                        invariant: extra.invariant,
                    }
                } else {
                    CtxValue::new(Val::Call(call))
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
        map_mode(
            Map::default(),
            form_mode(),
            Mode::default(),
            BasicMode::default(),
        ),
        BasicMode::default(),
    );
    named_free_fn(
        "context.represent",
        input_mode,
        output_mode,
        true,
        fn_ctx_repr,
    )
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
    named_free_fn("prelude", input_mode, output_mode, true, fn_ctx_prelude)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_this() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("this", input_mode, output_mode, true, fn_ctx_this)
}

fn fn_ctx_this(ctx: ConstFnCtx, _input: Val) -> Val {
    let ConstFnCtx::Const(ctx) = ctx else {
        return Val::default();
    };
    Val::Ctx(CtxVal::from(ctx.get_ctx_ref().clone()))
}
