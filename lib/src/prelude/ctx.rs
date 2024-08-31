use crate::{
    ask::Ask,
    bool::Bool,
    call::Call,
    ctx::{
        const1::{
            ConstCtx,
            ConstFnCtx,
        },
        default::DefaultCtx,
        free::FreeCtx,
        map::{
            CtxMap,
            CtxMapRef,
        },
        mut1::{
            MutCtx,
            MutFnCtx,
        },
        ref1::CtxRef,
        Ctx,
        CtxError,
        CtxValue,
        DynRef,
        Invariant,
    },
    list::List,
    mode::{
        basic::BasicMode,
        eval::Eval,
        Mode,
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
    pub(crate) is_unchecked: Named<FuncVal>,
    pub(crate) get_access: Named<FuncVal>,
    pub(crate) get_solver: Named<FuncVal>,
    pub(crate) set_solver: Named<FuncVal>,
    pub(crate) with_ctx: Named<FuncVal>,
    pub(crate) ctx_in_ctx_out: Named<FuncVal>,
    pub(crate) ctx_new: Named<FuncVal>,
    pub(crate) ctx_repr: Named<FuncVal>,
    pub(crate) ctx_prelude: Named<FuncVal>,
    pub(crate) ctx_self: Named<FuncVal>,
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
            is_unchecked: is_unchecked(),
            get_access: get_access(),
            get_solver: get_solver(),
            set_solver: set_solver(),
            with_ctx: with_ctx(),
            ctx_in_ctx_out: ctx_in_ctx_out(),
            ctx_new: ctx_new(),
            ctx_repr: ctx_repr(),
            ctx_prelude: ctx_prelude(),
            ctx_self: ctx_self(),
        }
    }
}

impl Prelude for CtxPrelude {
    fn put(&self, m: &mut Map<Symbol, CtxValue>) {
        self.read.put(m);
        self.move1.put(m);
        self.assign.put(m);
        self.set_final.put(m);
        self.set_const.put(m);
        self.is_final.put(m);
        self.is_const.put(m);
        self.is_null.put(m);
        self.is_unchecked.put(m);
        self.get_access.put(m);
        self.get_solver.put(m);
        self.set_solver.put(m);
        self.with_ctx.put(m);
        self.ctx_in_ctx_out.put(m);
        self.ctx_new.put(m);
        self.ctx_repr.put(m);
        self.ctx_prelude.put(m);
        self.ctx_self.put(m);
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
    let Ok(ctx) = ctx.get_variables_mut() else {
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
    let Ok(ctx) = ctx.get_variables_mut() else {
        return Val::default();
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
    let Ok(ctx) = ctx.get_variables_mut() else {
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
    let Ok(ctx) = ctx.get_variables_mut() else {
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
    let Ok(ctx) = ctx.get_variables() else {
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
    let Ok(ctx) = ctx.get_variables() else {
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

fn is_unchecked() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn(
        "is_unchecked",
        input_mode,
        output_mode,
        true,
        fn_is_unchecked,
    )
}

fn fn_is_unchecked(ctx: ConstFnCtx, _input: Val) -> Val {
    Val::Bool(Bool::new(ctx.is_unchecked()))
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

fn get_solver() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("get_solver", input_mode, output_mode, true, fn_get_solver)
}

fn fn_get_solver(ctx: ConstFnCtx, _input: Val) -> Val {
    match ctx.get_solver() {
        Ok(solver) => Val::Func(solver.clone()),
        _ => Val::default(),
    }
}

fn set_solver() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_mut_fn("set_solver", input_mode, output_mode, true, fn_set_solver)
}

fn fn_set_solver(ctx: MutFnCtx, input: Val) -> Val {
    match input {
        Val::Unit(_) => {
            let _ = ctx.set_solver(None);
        }
        Val::Func(solver) => {
            let _ = ctx.set_solver(Some(solver));
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
    let target_ctx = match pair.first {
        Val::Unit(_) => MutFnCtx::Free(FreeCtx),
        Val::Symbol(name) => {
            let Ok(ctx) = ctx.get_variables_dyn() else {
                return Val::default();
            };
            let Ok(DynRef { ref1, is_const }) = ctx.ref1.get_ref_dyn(name) else {
                return Val::default();
            };
            let Val::Ctx(target_ctx) = ref1 else {
                return Val::default();
            };
            if ctx.is_const || is_const {
                MutFnCtx::Const(ConstCtx::new(target_ctx))
            } else {
                MutFnCtx::Mut(MutCtx::new(target_ctx))
            }
        }
        _ => return Val::default(),
    };
    Eval.transform(target_ctx, val)
}

fn ctx_in_ctx_out() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("|:", input_mode, output_mode, false, fn_ctx_in_ctx_out)
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

const VARIABLES: &str = "variables";
const UNCHECKED: &str = "unchecked";
const SOLVER: &str = "solver";

fn ctx_new() -> Named<FuncVal> {
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(
            Map::default(),
            form_mode(),
            Mode::default(),
            BasicMode::default(),
        ),
    );
    map.insert(symbol(UNCHECKED), Mode::default());
    map.insert(symbol(SOLVER), Mode::default());
    let input_mode = map_mode(map, form_mode(), Mode::default(), BasicMode::default());
    let output_mode = Mode::default();
    named_free_fn("context", input_mode, output_mode, true, fn_ctx_new)
}

fn fn_ctx_new(input: Val) -> Val {
    let Val::Map(mut map) = input else {
        return Val::default();
    };
    let variables = match map_remove(&mut map, VARIABLES) {
        Val::Unit(_) => Map::default(),
        Val::Map(map) => Map::from(map),
        _ => return Val::default(),
    };
    let Some(variables) = parse_ctx_map(variables) else {
        return Val::default();
    };
    let unchecked = match map_remove(&mut map, UNCHECKED) {
        Val::Unit(_) => false,
        Val::Bool(b) => b.bool(),
        _ => return Val::default(),
    };
    let variables = CtxMap::new(variables, unchecked);
    let solver = match map_remove(&mut map, SOLVER) {
        Val::Unit(_) => None,
        Val::Func(solver) => Some(solver),
        _ => return Val::default(),
    };
    let ctx = Ctx::new(variables, solver);
    Val::Ctx(ctx.into())
}

fn parse_ctx_map(map: Map<Val, Val>) -> Option<Map<Symbol, CtxValue>> {
    map.into_iter()
        .map(|(key, val)| {
            let Val::Symbol(name) = key else {
                return None;
            };
            let ctx_value = parse_ctx_value(val)?;
            Some((name, ctx_value))
        })
        .collect()
}

fn parse_ctx_value(val: Val) -> Option<CtxValue> {
    let Val::Call(call) = val else {
        return Some(CtxValue::new(val));
    };
    if !call.func.is_unit() {
        return Some(CtxValue::new(Val::Call(call)));
    }
    let call = Call::from(call);
    let Val::Pair(pair) = call.input else {
        return None;
    };
    let pair = Pair::from(pair);
    let extra = parse_extra(pair.second, Extra {
        invariant: Invariant::None,
    })?;
    Some(CtxValue {
        val: pair.first,
        invariant: extra.invariant,
    })
}

fn ctx_repr() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(
            Map::default(),
            form_mode(),
            Mode::default(),
            BasicMode::default(),
        ),
    );
    map.insert(symbol(UNCHECKED), Mode::default());
    map.insert(symbol(SOLVER), Mode::default());
    let output_mode = map_mode(map, form_mode(), Mode::default(), BasicMode::default());
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
    let ctx = Ctx::from(ctx).destruct();
    let mut map = Map::default();
    let unchecked = ctx.variables.is_unchecked();
    if unchecked {
        map.insert(symbol(UNCHECKED), Val::Bool(Bool::t()));
    }
    if let Some(variables) = generate_ctx_map(ctx.variables) {
        map.insert(symbol(VARIABLES), variables);
    }
    if let Some(solver) = ctx.solver {
        map.insert(symbol(SOLVER), Val::Func(solver));
    };
    Val::Map(map.into())
}

fn generate_ctx_map(ctx_map: CtxMap) -> Option<Val> {
    if ctx_map.is_empty() {
        return None;
    }
    let map: Map<Val, Val> = ctx_map
        .unwrap()
        .into_iter()
        .map(|(k, v)| {
            let k = Val::Symbol(k);
            let v = generate_ctx_value(v);
            (k, v)
        })
        .collect();
    Some(Val::Map(map.into()))
}

fn generate_ctx_value(ctx_value: CtxValue) -> Val {
    let use_normal_form = 'a: {
        if let Val::Call(call) = &ctx_value.val {
            if let Val::Unit(_) = &call.func {
                break 'a true;
            }
        }
        matches!(ctx_value.invariant, Invariant::Final | Invariant::Const)
    };
    if use_normal_form {
        let func = Val::Unit(Unit);
        let invariant = generate_invariant(ctx_value.invariant);
        let pair = Val::Pair(Pair::new(ctx_value.val, Val::Symbol(invariant)).into());
        Val::Call(Call::new(func, pair).into())
    } else {
        ctx_value.val
    }
}

fn ctx_prelude() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_free_fn("prelude", input_mode, output_mode, true, fn_ctx_prelude)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_self() -> Named<FuncVal> {
    let input_mode = Mode::default();
    let output_mode = Mode::default();
    named_const_fn("self", input_mode, output_mode, true, fn_ctx_self)
}

fn fn_ctx_self(ctx: ConstFnCtx, _input: Val) -> Val {
    let ConstFnCtx::Const(ctx) = ctx else {
        return Val::default();
    };
    let ctx = ctx.get_ctx_ref().clone();
    Val::Ctx(CtxVal::from(ctx))
}
