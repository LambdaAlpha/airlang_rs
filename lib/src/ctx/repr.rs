use crate::{
    Abstract,
    AbstractVal,
    Ask,
    AskVal,
    Bit,
    Call,
    CallVal,
    Change,
    ChangeVal,
    Ctx,
    CtxVal,
    List,
    ListVal,
    Map,
    MapVal,
    Mode,
    MutFnCtx,
    Pair,
    PairVal,
    Symbol,
    Unit,
    Val,
    ctx::{
        map::{
            CtxMap,
            CtxMapRef,
            CtxValue,
            VarAccess,
        },
        ref1::CtxRef,
    },
    prelude::{
        map_mode,
        symbol_literal_mode,
    },
    utils::val::{
        map_remove,
        symbol,
    },
};

const ACCESS: &str = "access";
const STATIC: &str = "static";

pub(crate) const ASSIGNABLE: &str = "assignable";
pub(crate) const MUTABLE: &str = "mutable";
pub(crate) const CONST: &str = "constant";

pub(crate) const VARIABLES: &str = "variables";
pub(crate) const REVERSE: &str = "reverse";
pub(crate) const SOLVER: &str = "solver";

pub(crate) fn parse_mode() -> Mode {
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(Map::default(), symbol_literal_mode(), Mode::default()),
    );
    map_mode(map, symbol_literal_mode(), Mode::default())
}

pub(crate) fn parse_ctx(input: Val) -> Option<CtxVal> {
    let Val::Map(mut map) = input else {
        return None;
    };
    let variables = match map_remove(&mut map, VARIABLES) {
        Val::Unit(_) => Map::default(),
        Val::Map(map) => Map::from(map),
        _ => return None,
    };
    let variables = parse_ctx_map(variables)?;
    let reverse = match map_remove(&mut map, REVERSE) {
        Val::Unit(_) => false,
        Val::Bit(b) => b.bool(),
        _ => return None,
    };
    let variables = CtxMap::new(variables, reverse);
    let solver = match map_remove(&mut map, SOLVER) {
        Val::Unit(_) => None,
        Val::Func(solver) => Some(solver),
        _ => return None,
    };
    let ctx = Ctx::new(variables, solver);
    Some(ctx.into())
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
    let extra = parse_extra(pair.second, Extra::default())?;
    Some(CtxValue {
        val: pair.first,
        access: extra.access,
        static1: extra.static1,
    })
}

fn parse_extra(extra: Val, mut default: Extra) -> Option<Extra> {
    match extra {
        Val::Symbol(s) => match &*s {
            ASSIGNABLE => default.access = VarAccess::Assign,
            MUTABLE => default.access = VarAccess::Mut,
            CONST => default.access = VarAccess::Const,
            STATIC => default.static1 = true,
            _ => return None,
        },
        Val::Map(mut map) => {
            match map_remove(&mut map, ACCESS) {
                Val::Symbol(access) => {
                    default.access = parse_var_access(&access)?;
                }
                Val::Unit(_) => {}
                _ => return None,
            }
            let static1 = match map.remove(&symbol(STATIC)) {
                Some(Val::Unit(_)) => true,
                Some(Val::Bit(bit)) => bit.bool(),
                None => false,
                _ => return None,
            };
            default.static1 = static1;
        }
        _ => return None,
    }
    Some(default)
}

pub(crate) fn parse_var_access(access: &str) -> Option<VarAccess> {
    let access = match access {
        ASSIGNABLE => VarAccess::Assign,
        MUTABLE => VarAccess::Mut,
        CONST => VarAccess::Const,
        _ => return None,
    };
    Some(access)
}

pub(crate) fn generate_mode() -> Mode {
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        map_mode(Map::default(), symbol_literal_mode(), Mode::default()),
    );
    map_mode(map, symbol_literal_mode(), Mode::default())
}

pub(crate) fn generate_ctx(ctx: CtxVal) -> Val {
    let ctx = Ctx::from(ctx).destruct();
    let mut map = Map::default();
    let reverse = ctx.variables.is_reverse();
    if reverse {
        map.insert(symbol(REVERSE), Val::Bit(Bit::true1()));
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
        !matches!(ctx_value.access, VarAccess::Assign)
    };
    if use_normal_form {
        let func = Val::Unit(Unit);
        let access = generate_var_access(ctx_value.access);
        let pair = Val::Pair(Pair::new(ctx_value.val, Val::Symbol(access)).into());
        Val::Call(Call::new(func, pair).into())
    } else {
        ctx_value.val
    }
}

pub(crate) fn generate_var_access(access: VarAccess) -> Symbol {
    let access = match access {
        VarAccess::Assign => ASSIGNABLE,
        VarAccess::Mut => MUTABLE,
        VarAccess::Const => CONST,
    };
    Symbol::from_str(access)
}

pub(crate) struct Binding<T> {
    name: T,
    extra: Extra,
}

#[derive(Default, Copy, Clone)]
pub(crate) struct Extra {
    pub(crate) access: VarAccess,
    pub(crate) static1: bool,
}

#[derive(Copy, Clone)]
pub(crate) struct PatternCtx {
    pub(crate) extra: Extra,
    pub(crate) allow_extra: bool,
}

pub(crate) enum Pattern {
    Any(Binding<Symbol>),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
    Abstract(Box<Abstract<Pattern, Pattern>>),
    Ask(Box<Ask<Pattern, Pattern>>),
    Change(Box<Change<Pattern, Pattern>>),
    List(List<Pattern>),
    Map(Map<Val, Pattern>),
}

pub(crate) fn parse_pattern(pattern: Val, ctx: PatternCtx) -> Option<Pattern> {
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
        Val::Abstract(abstract1) => parse_pattern_abstract(abstract1, ctx),
        Val::Ask(ask) => parse_pattern_ask(ask, ctx),
        Val::Change(change) => parse_pattern_change(change, ctx),
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

fn parse_pattern_abstract(abstract1: AbstractVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let abstract1 = Abstract::from(abstract1);
    let func = parse_pattern(abstract1.func, ctx)?;
    let input = parse_pattern(abstract1.input, ctx)?;
    Some(Pattern::Abstract(Box::new(Abstract::new(func, input))))
}

fn parse_pattern_ask(ask: AskVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let ask = Ask::from(ask);
    let func = parse_pattern(ask.func, ctx)?;
    let output = parse_pattern(ask.output, ctx)?;
    Some(Pattern::Ask(Box::new(Ask::new(func, output))))
}

fn parse_pattern_change(change: ChangeVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let change = Change::from(change);
    let from = parse_pattern(change.from, ctx)?;
    let to = parse_pattern(change.to, ctx)?;
    Some(Pattern::Change(Box::new(Change::new(from, to))))
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
    if ctx.extra.static1 {
        return None;
    }
    parse_pattern(pair.first, ctx)
}

pub(crate) fn assign_pattern(ctx: MutFnCtx, pattern: Pattern, val: Val) -> Val {
    match pattern {
        Pattern::Any(binding) => assign_any(ctx, binding, val),
        Pattern::Pair(pair) => assign_pair(ctx, *pair, val),
        Pattern::Call(call) => assign_call(ctx, *call, val),
        Pattern::Abstract(abstract1) => assign_abstract(ctx, *abstract1, val),
        Pattern::Ask(ask) => assign_ask(ctx, *ask, val),
        Pattern::Change(change) => assign_change(ctx, *change, val),
        Pattern::List(list) => assign_list(ctx, list, val),
        Pattern::Map(map) => assign_map(ctx, map, val),
    }
}

fn assign_any(ctx: MutFnCtx, binding: Binding<Symbol>, val: Val) -> Val {
    let ctx_value = CtxValue {
        val,
        access: binding.extra.access,
        static1: false,
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

fn assign_abstract(mut ctx: MutFnCtx, pattern: Abstract<Pattern, Pattern>, val: Val) -> Val {
    let Val::Abstract(val) = val else {
        return Val::default();
    };
    let val = Abstract::from(val);
    let func = assign_pattern(ctx.reborrow(), pattern.func, val.func);
    let input = assign_pattern(ctx, pattern.input, val.input);
    Val::Abstract(Abstract::new(func, input).into())
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

fn assign_change(mut ctx: MutFnCtx, pattern: Change<Pattern, Pattern>, val: Val) -> Val {
    let Val::Change(val) = val else {
        return Val::default();
    };
    let val = Change::from(val);
    let from = assign_pattern(ctx.reborrow(), pattern.from, val.from);
    let to = assign_pattern(ctx, pattern.to, val.to);
    Val::Change(Change::new(from, to).into())
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
