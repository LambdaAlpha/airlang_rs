use crate::{
    Abstract,
    AbstractVal,
    AskVal,
    ListVal,
    Map,
    MapVal,
    PairVal,
    ask::Ask,
    bit::Bit,
    call::Call,
    ctx::{
        Ctx,
        CtxValue,
        DynRef,
        Invariant,
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
    },
    list::List,
    mode::{
        Mode,
        eval::Eval,
        primitive::PrimitiveMode,
    },
    pair::Pair,
    prelude::{
        Named,
        Prelude,
        form_mode,
        initial_ctx,
        map_mode,
        named_const_fn,
        named_free_fn,
        named_mut_fn,
        pair_mode,
    },
    symbol::Symbol,
    transformer::Transformer,
    unit::Unit,
    utils::val::{
        map_remove,
        symbol,
    },
    val::{
        Val,
        call::CallVal,
        ctx::CtxVal,
        func::FuncVal,
    },
};

#[derive(Clone)]
pub(crate) struct CtxPrelude {
    pub(crate) read: Named<FuncVal>,
    pub(crate) move1: Named<FuncVal>,
    pub(crate) assign: Named<FuncVal>,
    pub(crate) set_invariant: Named<FuncVal>,
    pub(crate) get_invariant: Named<FuncVal>,
    pub(crate) fallback: Named<FuncVal>,
    pub(crate) is_null: Named<FuncVal>,
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
            set_invariant: set_invariant(),
            get_invariant: get_invariant(),
            fallback: fallback(),
            is_null: is_null(),
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
        self.set_invariant.put(m);
        self.get_invariant.put(m);
        self.fallback.put(m);
        self.is_null.put(m);
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
    let id = "read";
    let call_mode = form_mode();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_read;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_read(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    DefaultCtx.get_or_default(ctx, s)
}

fn move1() -> Named<FuncVal> {
    let id = "move";
    let call_mode = form_mode();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_move;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let id = "=";
    let call_mode = pair_mode(form_mode(), Mode::default(), PrimitiveMode::default());
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_assign;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    Abstract(Box<Abstract<Pattern, Pattern>>),
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
        Val::Abstract(abstract1) => parse_pattern_abstract(abstract1, ctx),
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

fn parse_pattern_abstract(abstract1: AbstractVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let abstract1 = Abstract::from(abstract1);
    let func = parse_pattern(abstract1.func, ctx)?;
    let input = parse_pattern(abstract1.input, ctx)?;
    Some(Pattern::Abstract(Box::new(Abstract::new(func, input))))
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
        Pattern::Abstract(abstract1) => assign_abstract(ctx, *abstract1, val),
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

fn assign_abstract(mut ctx: MutFnCtx, pattern: Abstract<Pattern, Pattern>, val: Val) -> Val {
    let Val::Abstract(val) = val else {
        return Val::default();
    };
    let val = Abstract::from(val);
    let func = assign_pattern(ctx.reborrow(), pattern.func, val.func);
    let input = assign_pattern(ctx, pattern.input, val.input);
    Val::Abstract(Abstract::new(func, input).into())
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

fn set_invariant() -> Named<FuncVal> {
    let id = "set_invariant";
    let call_mode = form_mode();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_set_invariant;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_set_invariant(ctx: MutFnCtx, input: Val) -> Val {
    let Val::Pair(pair) = input else {
        return Val::default();
    };
    let pair = Pair::from(pair);
    let Val::Symbol(s) = pair.first else {
        return Val::default();
    };
    let Val::Symbol(invariant) = pair.second else {
        return Val::default();
    };
    let Some(invariant) = parse_invariant(&invariant) else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables_mut() else {
        return Val::default();
    };
    let _ = ctx.set_invariant(s, invariant);
    Val::default()
}

fn get_invariant() -> Named<FuncVal> {
    let id = "invariant";
    let call_mode = form_mode();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_get_invariant;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_get_invariant(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    let Ok(ctx) = ctx.get_variables() else {
        return Val::default();
    };
    let Some(invariant) = ctx.get_invariant(s) else {
        return Val::default();
    };
    Val::Symbol(generate_invariant(invariant))
}

fn fallback() -> Named<FuncVal> {
    let id = "fallback";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_fallback;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_fallback(ctx: ConstFnCtx, _input: Val) -> Val {
    let Ok(variables) = ctx.get_variables() else {
        return Val::default();
    };
    Val::Bit(Bit::new(variables.fallback()))
}

fn is_null() -> Named<FuncVal> {
    let id = "is_null";
    let call_mode = form_mode();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_is_null;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_is_null(ctx: ConstFnCtx, input: Val) -> Val {
    let Val::Symbol(s) = input else {
        return Val::default();
    };
    match DefaultCtx.is_null(ctx, s) {
        Ok(b) => Val::Bit(Bit::new(b)),
        Err(_) => Val::default(),
    }
}

fn get_access() -> Named<FuncVal> {
    let id = "access";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_get_access;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let id = "solver";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_get_solver;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_get_solver(ctx: ConstFnCtx, _input: Val) -> Val {
    match ctx.get_solver() {
        Ok(solver) => Val::Func(FuncVal::Cell(solver.clone())),
        _ => Val::default(),
    }
}

fn set_solver() -> Named<FuncVal> {
    let id = "set_solver";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_set_solver;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_set_solver(ctx: MutFnCtx, input: Val) -> Val {
    match input {
        Val::Unit(_) => {
            let _ = ctx.set_solver(None);
        }
        Val::Func(FuncVal::Cell(solver)) => {
            let _ = ctx.set_solver(Some(solver));
        }
        _ => {}
    }
    Val::default()
}

fn with_ctx() -> Named<FuncVal> {
    let id = "|";
    let call_mode = pair_mode(form_mode(), Mode::default(), PrimitiveMode::default());
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_with_ctx;
    named_mut_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let id = "|:";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = false;
    let f = fn_ctx_in_ctx_out;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
const FALLBACK: &str = "fallback";
const SOLVER: &str = "solver";

fn ctx_new() -> Named<FuncVal> {
    let id = "context";
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(
            Map::default(),
            form_mode(),
            Mode::default(),
            PrimitiveMode::default(),
        ),
    );
    map.insert(symbol(FALLBACK), Mode::default());
    map.insert(symbol(SOLVER), Mode::default());
    let call_mode = map_mode(map, form_mode(), Mode::default(), PrimitiveMode::default());
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_ctx_new;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
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
    let fallback = match map_remove(&mut map, FALLBACK) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
        _ => return Val::default(),
    };
    let variables = CtxMap::new(variables, fallback);
    let solver = match map_remove(&mut map, SOLVER) {
        Val::Unit(_) => None,
        Val::Func(FuncVal::Cell(solver)) => Some(solver),
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
    let id = "context.represent";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(
            Map::default(),
            form_mode(),
            Mode::default(),
            PrimitiveMode::default(),
        ),
    );
    map.insert(symbol(FALLBACK), Mode::default());
    map.insert(symbol(SOLVER), Mode::default());
    let ask_mode = map_mode(map, form_mode(), Mode::default(), PrimitiveMode::default());
    let cacheable = true;
    let f = fn_ctx_repr;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_ctx_repr(input: Val) -> Val {
    let Val::Ctx(ctx) = input else {
        return Val::default();
    };
    let ctx = Ctx::from(ctx).destruct();
    let mut map = Map::default();
    let fallback = ctx.variables.fallback();
    if fallback {
        map.insert(symbol(FALLBACK), Val::Bit(Bit::t()));
    }
    if let Some(variables) = generate_ctx_map(ctx.variables) {
        map.insert(symbol(VARIABLES), variables);
    }
    if let Some(solver) = ctx.solver {
        map.insert(symbol(SOLVER), Val::Func(FuncVal::Cell(solver)));
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
        !matches!(ctx_value.invariant, Invariant::None)
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
    let id = "prelude";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_ctx_prelude;
    named_free_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_ctx_prelude(_input: Val) -> Val {
    Val::Ctx(CtxVal::from(initial_ctx()))
}

fn ctx_self() -> Named<FuncVal> {
    let id = "self";
    let call_mode = Mode::default();
    let abstract_mode = call_mode.clone();
    let ask_mode = Mode::default();
    let cacheable = true;
    let f = fn_ctx_self;
    named_const_fn(id, call_mode, abstract_mode, ask_mode, cacheable, f)
}

fn fn_ctx_self(ctx: ConstFnCtx, _input: Val) -> Val {
    let ConstFnCtx::Const(ctx) = ctx else {
        return Val::default();
    };
    let ctx = ctx.get_ctx_ref().clone();
    Val::Ctx(CtxVal::from(ctx))
}
