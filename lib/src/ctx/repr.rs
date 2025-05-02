use crate::{
    Bit,
    Call,
    CallVal,
    CodeMode,
    Ctx,
    CtxVal,
    FuncMode,
    List,
    ListVal,
    Map,
    MapVal,
    Mode,
    MutFnCtx,
    Pair,
    PairVal,
    Symbol,
    SymbolMode,
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

pub(crate) fn parse_mode() -> Option<Mode> {
    let mut map = Map::default();
    map.insert(
        symbol(VARIABLES),
        FuncMode::map_mode(
            Map::default(),
            FuncMode::uni_mode(CodeMode::Form, SymbolMode::Literal),
            FuncMode::default_mode(),
        ),
    );
    FuncMode::map_mode(map, FuncMode::symbol_mode(SymbolMode::Literal), FuncMode::default_mode())
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
    let variables = parse_variables(variables)?;
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

fn parse_variables(map: Map<Val, Val>) -> Option<Map<Symbol, CtxValue>> {
    map.into_iter()
        .map(|(binding, val)| {
            let binding = parse_binding(binding)?;
            let ctx_value =
                CtxValue { access: binding.extra.access, static1: binding.extra.static1, val };
            Some((binding.name, ctx_value))
        })
        .collect()
}

fn parse_binding(val: Val) -> Option<Binding> {
    match val {
        Val::Symbol(name) => Some(Binding { name, extra: Extra::default() }),
        Val::Call(call) => {
            if !call.func.is_unit() {
                return None;
            }
            let call = Call::from(call);
            let Val::Pair(pair) = call.input else {
                return None;
            };
            let pair = Pair::from(pair);
            let Val::Symbol(name) = pair.first else {
                return None;
            };
            let extra = parse_extra(pair.second, Extra::default())?;
            Some(Binding { name, extra })
        }
        _ => None,
    }
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
    }
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
            let extra = Extra { access: v.access, static1: v.static1 };
            let k = generate_binding(k, extra);
            let v = v.val;
            (k, v)
        })
        .collect();
    Some(Val::Map(map.into()))
}

fn generate_binding(name: Symbol, extra: Extra) -> Val {
    if extra == Extra::default() {
        return Val::Symbol(name);
    }
    let extra = generate_extra(extra);
    let pair = Pair::new(Val::Symbol(name), extra);
    Val::Call(Call::new(Val::default(), Val::Pair(pair.into())).into())
}

fn generate_extra(extra: Extra) -> Val {
    if !extra.static1 {
        return Val::Symbol(generate_var_access(extra.access));
    }
    if extra.access == VarAccess::default() {
        return symbol(STATIC);
    }
    let mut map = Map::default();
    map.insert(symbol(STATIC), Val::default());
    map.insert(symbol(ACCESS), Val::Symbol(generate_var_access(extra.access)));
    Val::Map(map.into())
}

pub(crate) fn generate_var_access(access: VarAccess) -> Symbol {
    let access = match access {
        VarAccess::Assign => ASSIGNABLE,
        VarAccess::Mut => MUTABLE,
        VarAccess::Const => CONST,
    };
    Symbol::from_str(access)
}

pub(crate) struct Binding {
    name: Symbol,
    extra: Extra,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
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
    Any(Binding),
    Pair(Box<Pair<Pattern, Pattern>>),
    Call(Box<Call<Pattern, Pattern>>),
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
        Val::List(list) => parse_pattern_list(list, ctx),
        Val::Map(map) => parse_pattern_map(map, ctx),
        _ => None,
    }
}

fn parse_pattern_any(name: Symbol, ctx: PatternCtx) -> Pattern {
    Pattern::Any(Binding { name, extra: ctx.extra })
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

fn parse_pattern_list(list: ListVal, mut ctx: PatternCtx) -> Option<Pattern> {
    ctx.allow_extra = true;
    let list = List::from(list);
    let list =
        list.into_iter().map(|item| parse_pattern(item, ctx)).collect::<Option<List<_>>>()?;
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
        Pattern::List(list) => assign_list(ctx, list, val),
        Pattern::Map(map) => assign_map(ctx, map, val),
    }
}

fn assign_any(ctx: MutFnCtx, binding: Binding, val: Val) -> Val {
    let ctx_value = CtxValue { val, access: binding.extra.access, static1: false };
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
